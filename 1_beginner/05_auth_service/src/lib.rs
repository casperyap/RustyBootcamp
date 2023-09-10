#![allow(dead_code, unused_variables)]

mod database;
mod auth_utils;

use rand::prelude::*;
pub use auth_utils::models::Credentials;
use database::Status;

pub fn authenticate(creds: Credentials){
    let timeout = thread_rng().gen_range(100..500);

    println!("The timeout is {timeout}");
    if let Status::Connected = database::connect_to_database(){
        auth_utils::login(creds);
    }
}
