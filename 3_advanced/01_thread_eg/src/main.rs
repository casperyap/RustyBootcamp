#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx,rx) = mpsc::channel();
    
    let sentences = [
        "!dlroW olleH".to_owned(),
        ".tsurT eW tsuR nI".to_owned(),
        "!tsuR teG s'teL".to_owned(),
        "!tsuB ro tsuR".to_owned(),
    ];

    for s in sentences{
        let tx_clone = tx.clone();
        thread::spawn(move || {
            let reversed:String = s.chars().rev().collect();
            tx_clone.send(reversed).unwrap();
        });
    }

    drop(tx);
    for sentence in rx{
        println!("{sentence}");
    }
}


// fn main() {
//     let jhandle = thread::spawn(||{
//         for i in 0..20{
//             println!("Spawned Thread: {}", i);
//             thread::sleep(Duration::from_millis(10));
//         }
//     });

//     for i in 0..10{
//         println!("Main Thread: {}", i);
//         thread::sleep(Duration::from_millis(10));
//     }

//     jhandle.join().unwrap();
// }
