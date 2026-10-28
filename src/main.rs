mod user;
use user::User;

fn main() {
    let user = User::new("Anon-111".to_string(), 34);
    //println!("Hello, World!");
    user.greet_msg("Rust is awesome!".to_string());
    user.greet();
    user.display_age();
    
    //static method call!
    User::calculate_by_days(34);
}