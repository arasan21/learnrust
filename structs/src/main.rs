struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct Marker;
fn main() {
    println!("Hello, world!");

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let mut user1 = User {
        email: String::from("user1@test.com"),
        username: String::from("user1"),
        sign_in_count: 1,
        active: true,
    };
    user1.email = String::from("user1new@test.com");
    build_user(String::from("user2@gmail.com"), String::from("user2"));

    let user2 = User {
        email: String::from("user2@gmail.com"),
        username: String::from("user2"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
    // println!("user2 :: {}",user2);

    let user2 = User {
        email: String::from("user2@gmail.com"),
        username: String::from("user2"),
        ..user1
    };

    // println!("user2 :: {}",user2);
}
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
