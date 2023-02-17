fn main() {}

#[cfg(test)]
mod dedo_talk_test {
    use std::collections::HashMap;
    #[derive(Debug, Clone)]
    struct User {
        id: u64,
        email: String,
    }

    #[derive(Debug)]
    struct SubscribeEvent {
        user: User,
        meetup: String,
    }
    #[derive(Debug)]
    struct UnsubscribeEvent {
        user: User,
        meetup: String,
    }
    #[derive(Debug)]
    struct ShareEvent {
        user: User,
        meetup: String,
        friend: String,
    }

    #[derive(Debug)]
    enum Event {
        Subscribe(SubscribeEvent),
        Unsubscribe(UnsubscribeEvent),
        Share(ShareEvent),
    }

    fn subscribe(event: &SubscribeEvent) {
        println!("Subscribe: {:#?}", event)
    }

    fn unsubscribe(event: &UnsubscribeEvent) {
        println!("Unsubscribe: {:#?}", event)
    }

    fn share(event: &ShareEvent) {
        println!("Share: {:#?}", event)
    }

    fn handle_event(event: &Event) {
        match event {
            Event::Share(e) => share(e),
            Event::Subscribe(e) => subscribe(e),
            Event::Unsubscribe(e) => unsubscribe(e),
        }
    }
    // fn handle_event_wrong(event: &Event) {
    //     match event {
    //         Event::Subscribe(e) => unsubscribe(e),
    //         Event::Unsubscribe(e) => subscribe(e),
    //     }
    // }
    #[test]
    fn event_consumer_n_good() {
        let user_1 = User {
            id: 1,
            email: "alice@dedomainia.com".to_string(),
        };
        let events = vec![
            Event::Subscribe(SubscribeEvent {
                user: user_1.clone(),
                meetup: "dedotalk".into(),
            }),
            Event::Subscribe(SubscribeEvent {
                user: user_1.clone(),
                meetup: "dedotalk".into(),
            }),
            Event::Unsubscribe(UnsubscribeEvent {
                user: user_1.clone(),
                meetup: "dedotalk".into(),
            }),
            Event::Share(ShareEvent {
                user: user_1.clone(),
                meetup: "dedotalk".into(),
                friend: "Bob".into(),
            }),
        ];
        events.iter().for_each(handle_event)
    }

    // fn event_consumer_n_bad(event: &Event) {
    //     match event {
    //         Event::Share(e) => share(e),
    //         Event::Subscribe(e) => unsubscribe(e),
    //         Event::Unsubscribe(e) => subscribe(e),
    //     }
    // }
    #[test]
    fn list_comprehension() {
        let user_1 = User {
            id: 1,
            email: "alice@dedomainia.com".to_string(),
        };
        let user_2 = User {
            id: 2,
            email: "bob@dedomainia.com".to_string(),
        };
        let user_3 = User {
            id: 3,
            email: "charlie@gmail.com".to_string(),
        };
        let all_user_list = vec![user_1, user_2, user_3];
        let user_with_dedo_mail: Vec<User> = all_user_list
            .into_iter()
            .filter(|user| user.email.ends_with("dedomainia.com"))
            .collect();
        println!("{:#?}", user_with_dedo_mail);
    }

    #[test]
    fn dict_comprehension() {
        let user_1 = User {
            id: 1,
            email: "alice@dedomainia.com".to_string(),
        };
        let user_2 = User {
            id: 2,
            email: "bob@dedomainia.com".to_string(),
        };
        let user_3 = User {
            id: 3,
            email: "charlie@gmail.com".to_string(),
        };
        let all_user_list = vec![user_1, user_2, user_3];
        let user_by_id: HashMap<u64, User> = all_user_list
            .into_iter()
            .map(|user| (user.id, user))
            .collect();
        println!("{:#?}", user_by_id)
    }

    use std::net::{Ipv4Addr, Ipv6Addr};
    // use std::net::IpAddr as stdIpAddr
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
    fn optional_argument_not_needed_function(ip_adress: IpAddr) {
        match ip_adress {
            IpAddr::V4(_) => todo!(),
            IpAddr::V6(_) => todo!(),
        }
    }

    fn python_way(ip_address_v4: Option<Ipv4Addr>, ip_address_v6: Option<Ipv6Addr>) {
        match (ip_address_v4, ip_address_v6) {
            // INVALID
            (None, None) => todo!(),
            (None, Some(_)) => todo!(),
            (Some(_), None) => todo!(),
            // INVALID
            (Some(_), Some(_)) => todo!(),
        }
    }
    // #[test]
    // fn ownership_example() {
    //     let user = User {
    //         id: 1,
    //         email: "alice@dedomainia.com".to_string(),
    //     };
    //     let a = user;
    //     println!(user)
    // }
}
