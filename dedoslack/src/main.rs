use slack_morphism::prelude::*;

use hyper::{Body, Response};
use tracing::*;

use axum::Extension;
use std::sync::Arc;

async fn test_oauth_install_function(
    resp: SlackOAuthV2AccessTokenResponse,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) {
    println!("{:#?}", resp);
}

async fn test_welcome_installed() -> String {
    "Welcome".to_string()
}

async fn test_cancelled_install() -> String {
    "Cancelled".to_string()
}

async fn test_error_install() -> String {
    "Error while installing".to_string()
}

async fn test_post_message(
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

async fn test_push_event(
    Extension(_environment): Extension<Arc<SlackHyperListenerEnvironment>>,
    Extension(event): Extension<SlackPushEvent>,
) -> Response<Body> {
    println!("Received push event: {:?}", event);

    let body = match event {
        SlackPushEvent::UrlVerification(url_ver) => Body::from(url_ver.challenge),
        SlackPushEvent::EventCallback(event_callback) => {
            test_post_message("toto".into(), "#general".into())
                .await
                .unwrap();
            Body::empty()
        }
        _ => Body::empty(),
    };
    Response::builder()
        .status(200)
        .header("Content-Type", "text/plain")
        .body(body)
        .unwrap()
}
async fn test_command_event(
    Extension(_environment): Extension<Arc<SlackHyperListenerEnvironment>>,
    Extension(event): Extension<SlackCommandEvent>,
) -> axum::Json<SlackCommandEventResponse> {
    println!("Received command event: {:?}", event);
    axum::Json(SlackCommandEventResponse::new(
        SlackMessageContent::new().with_text("Working on it".into()),
    ))
}

async fn test_interaction_event(
    Extension(_environment): Extension<Arc<SlackHyperListenerEnvironment>>,
    Extension(event): Extension<SlackInteractionEvent>,
) {
    println!("Received interaction event: {:?}", event);
}

fn test_error_handler(
    err: Box<dyn std::error::Error + Send + Sync>,
    _client: Arc<SlackHyperClient>,
    _states: SlackClientEventsUserState,
) -> http::StatusCode {
    println!("{:#?}", err);

    // Defines what we return Slack server
    http::StatusCode::BAD_REQUEST
}

async fn test_server() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
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
        SlackClientEventsListenerEnvironment::new(client.clone())
            .with_error_handler(test_error_handler),
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
        .route("/installed", axum::routing::get(test_welcome_installed))
        .route("/cancelled", axum::routing::get(test_cancelled_install))
        .route("/error", axum::routing::get(test_error_install))
        .route(
            "/push",
            axum::routing::post(test_push_event).layer(
                listener
                    .events_layer(&signing_secret)
                    .with_event_extractor(SlackEventsExtractors::push_event()),
            ),
        )
        .route(
            "/command",
            axum::routing::post(test_command_event).layer(
                listener
                    .events_layer(&signing_secret)
                    .with_event_extractor(SlackEventsExtractors::command_event()),
            ),
        )
        .route(
            "/interaction",
            axum::routing::post(test_interaction_event).layer(
                listener
                    .events_layer(&signing_secret)
                    .with_event_extractor(SlackEventsExtractors::interaction_event()),
            ),
        );

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
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
