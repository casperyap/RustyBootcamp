#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::{sync::Mutex, thread};
use std::sync::Arc;

#[derive(Debug)]
struct Database{
    connections: Vec<u32>,
}

impl Database{
    fn new() -> Database{
        Database { connections: vec![] }
    }

    fn connect(&mut self, id: u32){
        self.connections.push(id);
    }
}

fn main(){
    let db = Arc::new(Mutex::new(Database::new()));
    let mut handles = vec![];

    for i in 0..10{
        let db = Arc::clone(&db);
        let handle = thread::spawn(move || {
            let mut db_lock = db.lock().unwrap();
            db_lock.connect(i);
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    let db_lock = db.lock().unwrap();
    print!("{db_lock:?}");
}