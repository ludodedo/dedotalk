use redis::{aio::ConnectionManager, AsyncCommands, Commands};
use slack_morphism::prelude::*;

use hyper::{Body, Response};
use tracing::*;

use axum::{extract::State, Extension};
use std::sync::Arc;

fn error_handler(
    err: Box<dyn std::error::Error + Send + Sync>,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> http::StatusCode {
    println!("{:#?}", err);

    // Defines what we return Slack server
    http::StatusCode::BAD_REQUEST
}

async fn post_message(
    message: String,
    channel_id: SlackChannelId,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = SlackClient::new(SlackClientHyperConnector::new());
    let token_value: SlackApiTokenValue = config_env_var("SLACK_TEST_TOKEN")?.into();
    let token: SlackApiToken = SlackApiToken::new(token_value);
    let session = client.open_session(&token);

    // let message = WelcomeMessageTemplateParams::new("".into());

    let post_chat_req = SlackApiChatPostMessageRequest::new(
        "#general".into(),
        SlackMessageContent::new().with_text(message),
    );

    let post_chat_resp = session.chat_post_message(&post_chat_req).await?;
    println!("post chat resp: {:#?}", &post_chat_resp);

    Ok(())
}

async fn test_oauth_install_function(
    resp: SlackOAuthV2AccessTokenResponse,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) {
    println!("{:#?}", resp);
}

async fn push_event(
    Extension(_environment): Extension<Arc<SlackHyperListenerEnvironment>>,
    Extension(event): Extension<SlackPushEvent>,
    State(mut pool): State<ConnectionManager>,
) -> Response<Body> {
    println!("Received push event: {:?}", event);
    let s: String = pool.get("key").await.unwrap();
    println!(" {s} ");
    let value_set: String = pool.set("key", "11").await.unwrap();
    let body = match event {
        SlackPushEvent::UrlVerification(url_ver) => {
            println!("toto");
            Body::from(url_ver.challenge)
        }
        SlackPushEvent::EventCallback(event_callback) => match event_callback.event {
            SlackEventCallbackBody::Message(message) => {
                if message.sender.bot_id.is_some() {
                    Body::empty()
                } else {
                    let text = message.sender.user.unwrap().to_string();
                    let mut last_range: Vec<String> = pool.lrange("Users", -1, -1).await.unwrap();
                    match last_range.pop() {
                        Some(last) => {
                            if last == text {
                                post_message(
                                    "You are already in the queue".into(),
                                    "#general".into(),
                                )
                                .await
                                .unwrap();
                            } else {
                                add_user_to_queue(pool, text).await;
                            };
                        }
                        None => {
                            add_user_to_queue(pool, text).await;
                        }
                    }
                    Body::empty()
                }
            }
            _ => Body::empty(),
        },
        _ => Body::empty(),
    };
    Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(body)
        .unwrap()
}

async fn add_user_to_queue(mut pool: ConnectionManager, text: String) {
    let var: u8 = pool.rpush("Users", text).await.unwrap();
    let queue_content: Vec<String> = pool.lrange("Users", 0, -1).await.unwrap();
    let queue_message = queue_content.join(", ");
    post_message(queue_message, "#general".into())
        .await
        .unwrap();
}

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}
async fn test_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    println!("Starting server");

    let pool = redis::aio::ConnectionManager::new(client).await?;

    // let mut con = client.get_connection()?;

    // con.set("SLACK_CLIENT_ID", "1234567890")?;
    let client: Arc<SlackHyperClient> =
        Arc::new(SlackClient::new(SlackClientHyperConnector::new()));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 8080));
    info!("Loading server: {}", addr);

    let oauth_listener_config = SlackOAuthListenerConfig::new(
        config_env_var("SLACK_CLIENT_ID")?.into(),
        config_env_var("SLACK_CLIENT_SECRET")?.into(),
        config_env_var("SLACK_BOT_SCOPE")?,
        config_env_var("SLACK_REDIRECT_HOST")?,
    );

    let listener_environment: Arc<SlackHyperListenerEnvironment> = Arc::new(
        SlackClientEventsListenerEnvironment::new(client.clone()).with_error_handler(error_handler),
    );
    let signing_secret: SlackSigningSecret = config_env_var("SLACK_SIGNING_SECRET")?.into();

    let listener: SlackEventsAxumListener<SlackHyperHttpsConnector> =
        SlackEventsAxumListener::new(listener_environment.clone());

    // build our application route with OAuth nested router and Push/Command/Interaction events
    let app = axum::routing::Router::new()
        .nest(
            "/auth",
            listener.oauth_router("/auth", &oauth_listener_config, test_oauth_install_function),
        )
        .route(
            "/push",
            axum::routing::post(push_event)
                .layer(
                    listener
                        .events_layer(&signing_secret)
                        .with_event_extractor(SlackEventsExtractors::push_event()),
                )
                .with_state(pool),
        );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("axum_events_api_server=debug,slack_morphism=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;
    test_server().await?;

    Ok(())
}
