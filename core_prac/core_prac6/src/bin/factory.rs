#![allow(warnings)]
#![allow(dead_code)]

fn main() {
    println!("Factory Pattern");
    println!("Had to review the code comments before impl");

    if let Some(doggy) = AnimalFactory::create("dog") {
        println!("{}", doggy.sound());
    }
}

enum Animal {
    Dog,
    Cat,
}

impl Animal {
    fn sound(&self) -> &str {
        match &self {
            Animal::Dog => "Make me a dog",
            Animal::Cat => "Make me a cat",
        }
    }
}

struct AnimalFactory;

impl AnimalFactory {
    fn create(atype: &str) -> Option<Animal> {
        if atype == "dog" {
            Some(Animal::Dog)
        } else if atype == "cat" {
            Some(Animal::Cat)
        } else {
            None
        }
    }
}
