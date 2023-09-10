use std::collections::HashMap;

use decalarative_macros::*;

fn main() {
    hello!();
    hello![];
    hello!{};

    let scores: HashMap<String, i32> = HashMap::new();
    let mut scores2 = HashMap::new();

    scores2.insert("Red team".to_owned(), 3);
    scores2.insert("Blue team".to_owned(), 5);
    scores2.insert("Green team".to_owned(), 2);

    let scores3 = map!(String, i32);
    let score4 = map!(
        "Red team".to_owned() => 3,
        "Blue team".to_owned() => 5,
        "Green team".to_owned() => 2
    );

}
