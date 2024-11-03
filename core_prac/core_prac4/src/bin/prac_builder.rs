fn main() {
    println!("Practice builder");
    // start with actual struct and its attrs
    // create a builder with attrs as options
    // impl methods to fill the optional attrs
    // after the new method returns the builder
    // each method must return the self
    // while the build method must return actual struct
    let b1 = CarBuilder::new()
        .name("thetmoto".to_owned())
        .age(7)
        .price(966.5)
        .build();
    println!("Car B1: {}", b1);
}

struct Car {
    name: String,
    age: u8,
    price: f32,
}

use std::fmt::Display;

impl Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Car {} is {} priced and {} old",
            self.name, self.price, self.age
        )
    }
}

struct CarBuilder {
    name: Option<String>,
    age: Option<u8>,
    price: Option<f32>,
}

impl CarBuilder {
    fn new() -> Self {
        CarBuilder {
            name: None,
            age: None,
            price: None,
        }
    }

    fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    fn price(mut self, price: f32) -> Self {
        self.price = Some(price);
        self
    }
    fn age(mut self, age: u8) -> Self {
        self.age = Some(age);
        self
    }
    fn build(self) -> Car {
        Car {
            name: self.name.unwrap_or("noname".to_owned()),
            price: self.price.unwrap_or(57.7),
            age: self.age.unwrap_or(57),
        }
    }
}
