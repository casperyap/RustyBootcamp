#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::time::Duration;
use tokio::time::sleep;
use tokio_stream::StreamExt;

#[tokio::main(flavor = "current_thread")] //flavor using current thread means it is using single thread with time slicing.
async fn main() {
    let mut stream = tokio_stream::iter(["Let's", "Get", "Rusty"])
    .map(|s| s.to_ascii_uppercase());

    //streams are not supported in for loops yet.
    while let Some(s) = stream.next().await{
        println!("{s}");
    }

    let mut handles = vec![];

    for i in 0..2{
        let handle = tokio::spawn(async move{
            my_function(i).await;
        });
        handles.push(handle);
    }

    handles.push(tokio::spawn(async{
        let _res = tokio::task::spawn_blocking(|| {
            expensive_computation();
        });        
    }));
    
    for handle in handles{
        handle.await.unwrap();
    }
}

async fn my_function(i: i32){
    println!("[{i}] I'm an async function");
    let s1 = read_from_database().await;
    println!("[{i}] First result:{s1}");
    let s2 = read_from_database().await;
    println!("[{i}] Second result:{s2}");
}

async fn read_from_database() -> String{
    sleep(Duration::from_millis(1000)).await;
    "DB Result".to_owned()
}

fn expensive_computation(){
    let mut i = 0;

    for _ in 0..400000000{
        i = i + 1;
    }

    println!("Done with expensive computation i = {i}");
}

// fn my_function() -> impl Future<Output = ()>{
//     println!("I'm an async function");
// }