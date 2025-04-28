fn main() {
    println!("Hello, world!");

    struct Uber {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    let mut user1 = Uber {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1 // This will copy the remaining fields from user1
    };

    user1.email = String::from("anotheremail@example.com");
    println!("User email: {}", user1.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}