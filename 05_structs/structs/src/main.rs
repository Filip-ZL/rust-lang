struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
// tuple like struct
struct Color(u8, u8, u8);

// empty struct Unit-like
struct AlwaysEqual;

fn main() {

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("some@email.com"),
        sign_in_count: 1,
    };

    // creating user 2 without update func
    let user2 = User {
        active: user1.active,
        username: String::from("someone123"),
        email: String::from("some@email.com"),
        sign_in_count: user1.sign_in_count,
    };

    // using update syntax
    let user2 = User {
        email: String::from("some@email.com"),
        ..user1
    };

    println!("{}", user1.email);

    let white = Color(255, 255, 255);

    let subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}