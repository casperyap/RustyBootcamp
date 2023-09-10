#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;
use ui::Page;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    // TODO: create database and navigator
    let db = Rc::new(JiraDatabase::new("./data/db.json".to_owned()));
    let mut nav = Navigator::new(db);
    
    loop {
        clearscreen::clear().unwrap();

        // TODO: implement the following functionality:
        // 1. get current page from navigator. If there is no current page exit the loop.
        if let Some(curr_page) = nav.get_current_page(){
            // 2. render page
            if let Err(error) = curr_page.draw_page(){
                println!("Error rendering page: {}\nPress any key to continue...", error);
                wait_for_key_press();
            }
            else {
                // 3. get user input
                let user_input = get_user_input();

                // 4. pass input to page's input handler
                if let Ok(Some(action)) =  curr_page.handle_input(user_input.as_str().trim_end()){
                    // 5. if the page's input handler returns an action let the navigator process the action
                    let mut error = false;
                    nav.handle_action(action).unwrap_or_else(|e| error = true);
                    if error {
                        println!("Error at navigator handling action.");
                        continue;
                    }
                }
            }
        } else{ // Exit Program
            break;
        }       
    }

    println!("Good Bye!!!");
    println!("");
}