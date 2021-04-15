#[allow(dead_code)]
struct User {
    username: String,
    first: String,
    last: String,
    uid: u32,
    gid: u32,
}

fn create_user(first: String, last: String) -> User {
    // let username = String::from(first[..1] + &last);
    let username = first[..1].to_string() + &last;
    let user = User {
        first,
        last,
        username,
        uid: 0,
        gid: 0
    };
    return user;
}


fn main() {
    let mut user1 = create_user("foo".to_string(), "bar".to_string());
    user1.uid = 1000;
    println!("user1 = {}", user1.username);

    let mut user2 = user1;
    user2.username = "baz".to_string();
    println!("user2 = {}", user2.username);
}
