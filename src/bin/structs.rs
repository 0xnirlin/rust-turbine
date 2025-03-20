// User struct definition with lifetime parameter
struct User<'a> {
    active: bool,
    username: &'a str,
    email: &'a str,
    sign_in_count: u64,
}

// Implementation for User struct
impl<'a> User<'a> {
    // Method to get sign_in_count
    fn get_count(&self) -> u64 {
        self.sign_in_count
    }
    
    // Static method to build a User instance
    fn build_user(email: &'a str, username: &'a str) -> Self {
        User {
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }
}

fn main() {
    // Create user1 using the build_user method
    let user1 = User::build_user("0xnirlin@gmail.com", "0xnirlin");
    
    println!("User details:");
    println!("Username: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);
    println!("Sign-in count: {}", user1.sign_in_count);
    
    // Create user2 using struct update syntax, borrowing fields from user1
    let user2 = User {
        username: "japarjam",
        ..user1
    };
    
    println!("User2 details:");
    println!("Username: {}", user2.username);
    println!("Email: {}", user2.email);
    println!("Active: {}", user2.active);
    println!("Sign-in count: {}", user2.sign_in_count);
    
    // Demonstrate use of the get_count method
    println!("User1 sign-in count using method: {}", user1.get_count());
    println!("User2 sign-in count using method: {}", user2.get_count());
}