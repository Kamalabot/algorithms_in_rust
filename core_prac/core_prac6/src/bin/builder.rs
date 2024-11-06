#![allow(warnings)]
#![allow(dead_code)]

fn main() {
    println!("Car Builder Pattern");
    let car1 = CarBuilder::new()
        .price(1000)
        .name("elevated".to_owned())
        .make("Honda".to_owned())
        .build();
    println!("{}", car1)
}

struct Car {
    name: String,
    price: i32,
    make: String,
}

use std::fmt::Display;

impl Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} car made by {} is priced at {}",
            self.name, self.make, self.price
        )
    }
}
#[derive(Debug, Clone)]
struct CarBuilder {
    name: Option<String>,
    price: Option<i32>,
    make: Option<String>,
}

impl CarBuilder {
    fn new() -> Self {
        Self {
            name: None,
            price: None,
            make: None,
        }
    }
    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    fn price(mut self, price: i32) -> Self {
        self.price = Some(price);
        self
    }
    fn make(mut self, make: String) -> Self {
        self.make = Some(make);
        self
    }
    fn build(self) -> Car {
        Car {
            name: self.name.unwrap_or("name1".to_owned()),
            price: self.price.unwrap_or(100),
            make: self.make.unwrap_or("make1".to_owned()),
        }
    }
}
