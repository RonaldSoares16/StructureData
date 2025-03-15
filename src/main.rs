use std::io;
fn main() {
    test_struct()
}

struct User{
    name : String,
    email : String,
    is_online : bool
}
fn test_struct() {
    let mut name = String::new();
    let mut email = String::new();
    println!("Write your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line.");
    println!("Write your email: ");
    io::stdin().read_line(&mut email).expect("Failed to read line.");
    let user = create_user(name, email);
    println!(" name: {} \n email: {} \n Is online: {}", user.name, user.email, user.is_online);
}

fn create_user(name: String, email: String) -> User {
    User{
        name,
        email,
        is_online: true
    }
}
