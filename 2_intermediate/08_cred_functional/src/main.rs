#![allow(dead_code)]
#![allow(unused_variables)]

use std::default;

struct Credentials<T> where T: Fn(&str, &str)->bool {
    username: String,
    password: String,
    validator: T,
}

impl<T> Credentials<T> where T: Fn(&str, &str)->bool {
    fn is_valid(&self) -> bool{
        (self.validator)(&self.username, &self.password)
    }
}

fn get_default_creds<T>(f:T) -> Credentials<T> where T: Fn(&str, &str)->bool{
    Credentials{
        username: "guest".to_owned(),
        password: "password123".to_owned(),
        validator: f
    }
}

fn get_password_validator(min_len:usize, special_char: bool) -> Box<dyn Fn(&str, &str)->bool> {
    if special_char{
        Box::new(move |_: &str, password: &str| {           
            password.len() > min_len &&
            password.contains(['!', '@', '#', '$', '%', '^', '&', '*'])})
    }else {
        Box::new(move |_: &str, password: &str| !password.len() >= min_len)
    }
    
}

fn main(){
    
    let validator = |username: &str, password: &str| {
        !username.is_empty() && !password.is_empty()
    };

    let weak_password = "password123!".to_owned();

    let validator2 = |username: &str, password: &str| {
        !username.is_empty() &&
        !password.is_empty() &&
        password.len() > 8 &&
        password.contains(['!', '@', '#', '$', '%', '^', '&', '*']) && 
        password != weak_password
    };

    let password_validator = get_password_validator(8, true);

    let creds = Credentials{
        username: "admin".to_owned(),
        password: "password123!".to_owned(),
        validator: validator2,
    };

    println!("{}", creds.is_valid());
    let default_creds = get_default_creds(validator2);
}