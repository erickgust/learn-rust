struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

fn build_user (username: String, email: String) -> User {
    User { active: true, username, email, sign_in_count: 1 }
}

fn main() {
    let user = User {
        active: true,
        sign_in_count: 1,
        email: String::from("someone@example.com"),
        username: String::from("someusername123")
    };

    println!("User: {}", user.username);
    
    let mut user = build_user(user.username, user.email);
    user.username = String::from("teste");
    
    println!("User: {}", user.username);

}
