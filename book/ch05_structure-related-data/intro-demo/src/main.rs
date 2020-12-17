struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    let email = String::from("someone@example.com");
    let username = String::from("someusername222");
    let mut user2 = build_user(email, username);

    user2.sign_in_count += 1;
    println!("{}", user2.active);

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println!("{}", user3.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
