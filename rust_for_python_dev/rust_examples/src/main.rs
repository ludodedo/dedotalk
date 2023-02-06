use std::collections::HashMap;

#[derive(Debug)]
struct User {
    id: u64,
    email: String,
}

fn main() {
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
    let user_by_id: HashMap<u64, User> = user_with_dedo_mail
        .into_iter()
        .map(|user| (user.id, user))
        .collect(); // Fail because user_with_dedo_mail took ownership of all_user_list, would work either with iter instead of into_iter but we would endup with Vec<&User> of by cloning all_user_list.clone().into_iter()
    println!("{:#?}", user_by_id)
}
