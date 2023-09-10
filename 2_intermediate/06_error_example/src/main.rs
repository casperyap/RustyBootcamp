#![allow(dead_code)]
#![allow(unused_variables)]

use std::{fs, io, error, num::ParseIntError};

fn main(){
    let first_line = read_first_line("example.txt");
    let second_line = read_first_line("example.txt");
    let i = parse_file("example.txt");

    match i{
        Ok(i) => println!("{}", i),
        Err(e) => {
            match e{
                ParseFileError::File(es) => println!("File Error: {es}"),
                ParseFileError::Parse(es) => println!("Parse Error: {es}"),
            }
        }
    }
}

//1: Return result with option
fn read_first_line(filename: &str) -> Result<Option<String>, io::Error>{
    fs::read_to_string(filename).map(|s | {
        s.lines().next().map(|s| s.to_owned())
    })
}

//2: Return option
fn read_second_line(filename: &str) -> Option<String>{
    fs::read_to_string(filename).ok().and_then(|s | {
        s.lines().next().map(|s| s.to_owned())
    })
}

// //1: Returning different types of errors within the same funciton by generic error type
// fn parse_file(filename: &str) -> Result<i32, Box<dyn error::Error>>{
//     let s = fs::read_to_string(filename)?;
//     let i = s.parse()?;
//     Ok(i)    
// }

enum ParseFileError{
    File(io::Error),
    Parse(ParseIntError)
}

//2: Returning different types of errors within the same funciton by custom error enum
fn parse_file(filename: &str) -> Result<i32, ParseFileError>{
    let s = fs::read_to_string(filename)
        .map_err(|e| ParseFileError::File(e))?;
    let i = s.parse().map_err(|e | ParseFileError::Parse(e))?;
    Ok(i)    
}