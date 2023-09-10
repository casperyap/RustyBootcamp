#![allow(dead_code)]
#![allow(unused_variables)]

use std::rc::Rc;
use std::cell::RefCell;

struct Database{
    max_connections: u32,
}

struct AuthService{
    db: Rc<RefCell<Database>>,
}

struct ContentService{
    db: Rc<RefCell<Database>>
}

fn main(){
    let db = Rc::new(RefCell::new(Database{max_connections: 100}));
    let auth_service = AuthService{db: Rc::clone(&db)};
    let content_service = ContentService{db: Rc::clone(&db)};

    let mut r1 = db.borrow_mut();
    // let mut r2 = db.borrow_mut(); //Refcell is unsafe. This is not allowed at runtime.
    r1.max_connections = 200;
}