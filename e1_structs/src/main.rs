struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User { // structs must be entirely mutable, or not at all; can't pick and choose
        // attributes can be in any order
        email: String::from("hello@example.com"),
        username: String::from("friendly_stranger_123"),
        active: true,
        sign_in_count: 1
    };

    let name = user1.username;
    user1.username = String::from("friendly_stranger");

    let user2 = build_user(String::from("kyle@example.com"), String::from("kyle91"));

    let user3 = User {
        email: String::from("sarah_davies@example.com"),
        username: String::from("sarah_d"),
        ..user2
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username, // field init shorthand syntax - equivalent to previous line, but more concise
        active: true,
        sign_in_count: 1,
    }
}