#![allow(warnings)]
#![allow(unconditional_panic)]

use std::error::Error;

#[derive(Debug)]
struct ZeroDivideError {
    details: String,
}

impl ZeroDivideError {
    fn new(msg: &str) -> ZeroDivideError {
        ZeroDivideError {
            details: msg.to_string(),
        }
    }
}
use std::fmt::Display;

impl Error for ZeroDivideError {}

impl Display for ZeroDivideError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Zero divide: {}", self.details)
    }
}

fn div_by_zero(a: i32, b: i32) -> Result<i32, Box<dyn Error>> {
    if b == 0 {
        return Err("Divide by 0, leads to infinty....".into());
    }
    Ok(a / b)
}

fn div_by_zero_error(a: i32, b: i32) -> Result<i32, ZeroDivideError> {
    if b == 0 {
        return Err(ZeroDivideError::new("Structured output"));
    }
    Ok(a / b)
}

// Combining different error types using `Box<dyn Error>`
pub fn parse_and_double(input: &str) -> Result<i32, Box<dyn Error>> {
    let number = input.parse::<i32>()?;
    // if number < 0 {
    //     return Err(Box::new(ZeroDivideError::new(
    //         "Negative number due to number parsing",
    //     )));
    // }
    Ok(number * 2)
}

pub fn parse_and_dbl(input: &str) -> Result<i32, ZeroDivideError> {
    let number = input.parse::<i32>();
    match number {
        Err(_) => {
            return Err(ZeroDivideError::new(
                "Negative number due to number parsing",
            ))
        }
        Ok(number) => return Ok(number * 2),
    }
}

fn main() {
    println!("Creating Custom Errors");
    // let err_rs = div_by_zero(57, 0);
    // let val_rs = div_by_zero(50, 5);

    let err_rs = div_by_zero_error(57, 0);
    let val_rs = div_by_zero_error(50, 5);

    let parseme = parse_and_double("56,");
    let prsme = parse_and_dbl("56,");

    // match err_rs {
    //     Ok(val) => println!("Got {val}"),
    //     Err(e) => eprintln!("Got {:?}", e),
    // }
    // match val_rs {
    //     Ok(val) => println!("Got {val}"),
    //     Err(e) => eprintln!("Got {:?}", e),
    // }
    match parseme {
        Ok(val) => println!("Got {val}"),
        Err(e) => eprintln!("Got {:?}", e.to_string()),
    }
    match prsme {
        Ok(val) => println!("Got {val}"),
        Err(e) => eprintln!("Got {:?}", e.to_string()),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_me() {
        let x = 50;
        let y = 10;
        // let rs = x / y;
        let cuw = std::panic::catch_unwind(|| x / y);
        // println!("{}", rs)
        match cuw {
            Ok(val) => println!("Got the {val}"),
            Err(e) => println!("Error: {:?}", e),
        }
    }
}
