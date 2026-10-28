

pub struct User {
    username: String,
    age: u32,
}

impl User {
    pub fn new(username: String, age: u32) -> Self {
        
        Self {username, age}
    }
    
    pub fn greet(&self) {
        println!("My username is {}.", self.username);
    }
    
    pub fn greet_msg(&self, msg: String) {
        println!("{msg}");
    }
    
    pub fn display_age(&self) {
        println!("Age is {}.", self.age);
    }
    
    //like getter
    pub fn age(&self) -> u32 {
        self.age
    }
    
    //setter
    pub fn set_age(&mut self, age: u32) {
        self.age = age;
    }
    
    pub fn have_birthday(&mut self, years: u32) {
        self.age += years;
    }
    
    pub fn can_access(&self, min_age: u32, is_admin: bool) -> bool {
        is_admin || self.age >= min_age
    }
    
    //static method no '&self'
    pub fn calculate_by_days(age: u32) {
        let days = age * 365;
        
        println!("Your age in days = {days} ")
    }
}