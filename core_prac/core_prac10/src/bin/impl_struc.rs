fn main() {
    println!("Getting hand on struct");
    let mut itl = Item::new("newitem".to_owned(), 1);
    println!("Item1 : {}", itl);
    itl.next();
    println!("Item1 : {}", itl);
}

use std::fmt::Display;
use std::iter::Iterator;

#[derive(Debug)]
struct Item {
    name: String,
    age: i32,
}

impl Item {
    fn new(name: String, age: i32) -> Item {
        Item { name, age }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {} and age: {}", self.name, self.age)
    }
}

impl Iterator for Item {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.age += 1;
        if self.age <= 10 {
            Some(self.age)
        } else {
            None
        }
    }
}
