struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn build_user (username: String, email: String) -> User {
    User { active: true, username, email, sign_in_count: 1 }
}

fn main() {
    let user = build_user(
        String::from("someusername123"),
        String::from("someone@example.com")
    );

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
