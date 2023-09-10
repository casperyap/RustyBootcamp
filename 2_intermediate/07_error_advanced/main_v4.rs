#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::HashMap, io, error::Error};
use std::fmt::{self, Display};

struct ParsePaymentInfoError {
    source: Option<Box<dyn Error>>,
    msg: String
}

impl Error for ParsePaymentInfoError{
    fn source(&self) -> Option<&(dyn Error + 'static)>{
        self.source.as_deref()
    }
}

impl Display for ParsePaymentInfoError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Parsing payment error: invalid payment info!")
    }
}

impl std::fmt::Debug for ParsePaymentInfoError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{self}\n\t{}", self.msg)?;

        if let Some(e) = self.source.as_ref(){
            write!(f, "\n\nCaused by:\n\t{e:?}")?;
        }

        Ok(())
    }
}

// //1: Use From triat so that 'parse' can automatically change from ParseIntError to ParsePaymentInfoError
// // However, this method will not be able to have a customised msg.
// impl From<ParseIntError> for ParsePaymentInfoError{
//     fn from(e: ParseIntError) -> Self{
//         ParsePaymentInfoError{
//             source: Some(Box::new(e)),
//             msg: None,
//         }
//     }
// }

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    // let numbers = card
    //     .split(" ")
    //     .into_iter()
    //     .map(|s| {
    //         s.parse()
    //     })
    //     .collect::<Result<Vec<u32>, _>>()?;

    let numbers = card
        .split(" ")
        .into_iter()
        .map(|s| {
            s.parse().map_err(|_| 
                ParsePaymentInfoError{
                    source: None,
                    msg: format!("{s:?} could not be parsed as u32"),
                })
        })
        .collect::<Result<Vec<u32>, _>>()
        .map_err({|e | ParsePaymentInfoError {
            source: Some(Box::new(e)),
            msg: format!("Failed to parse input as numbers. Input: {card}"),
        }})?;

    Ok(numbers)
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(ParsePaymentInfoError { 
            source: None, 
            msg: format!("Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}"),
        });
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap(); 

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv
    })
}

enum CreditCardError{
    InvalidInput(String),
    Other(Box<dyn Error>, String), //First item contains the internal error, Second item contains the display error string.
}

impl Error for CreditCardError{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self{
            CreditCardError::InvalidInput(_) => None,
            CreditCardError::Other(src_err, _) => Some(src_err.as_ref()),
        }
    }
}

impl Display for CreditCardError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Credit card error: Could not retrieve credit card.")
    }
}

impl std::fmt::Debug for CreditCardError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::InvalidInput(msg)=> write!(f, "{}\n{}", self, msg),
            Self::Other(e, a)=> write!(f, "{self}\n{a}\n\nCaused by: {e:?}"),
        }
    }
}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name).ok_or(
        CreditCardError::InvalidInput(format!("No credit card was found for {name}."))
    )?;

    let card = parse_card(card_string)
        .map_err( |e | 
            CreditCardError::Other(Box::new(e), format!("{name}'s card could not be parsed."))
        )?;

    Ok(card)
}

fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 12 16 123"),
        ("Tim", "2234567 0616 123"),
        ("Bob", "3234567 Dec 08 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");
    
    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => {
            match &err{
                CreditCardError::InvalidInput(estr) => println!("{estr}"),
                CreditCardError::Other(_, dis_str) => println!("{dis_str}"),
            }
            
            log::error!("\n{err:?}");
        },
    }
}