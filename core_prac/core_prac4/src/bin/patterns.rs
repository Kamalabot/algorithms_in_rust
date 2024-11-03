fn main() {
    println!("From bin...");
    let cbuild = CarBuilder::new()
        .engine("Newyork Mondays")
        .seats(5)
        .color("Peach")
        .build();
    println!("{}", cbuild);
    let c1 = ZipCompress;
    println!("{}", c1.compress("tomorrow compress"));

    let k2 = GCompress;
    println!("{}", k2.compress("Afternoon compress"));

    if let Some(ani) = AnimalFactory::create_animal("dog") {
        println!("ani says:{}", ani.speak())
    }
}

struct Car {
    color: String,
    seats: u8,
    engine: String,
}

use std::fmt::Display;

impl Display for Car {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Car with {} seats and model {}", self.seats, self.engine)
    }
}

struct CarBuilder {
    color: Option<String>,
    seats: Option<u8>,
    engine: Option<String>,
}

impl CarBuilder {
    fn new() -> Self {
        CarBuilder {
            color: None,
            seats: None,
            engine: None,
        }
    }

    fn color(mut self, color: &str) -> Self {
        self.color = Some(color.to_string());
        self
    }
    fn seats(mut self, seats: u8) -> Self {
        self.seats = Some(seats);
        self
    }
    fn engine(mut self, engine: &str) -> Self {
        self.engine = Some(engine.to_string());
        self
    }
    fn build(self) -> Car {
        Car {
            color: self.color.unwrap_or("white".to_string()),
            engine: self.engine.unwrap_or("Mastodon Ford".to_string()),
            seats: self.seats.unwrap_or(4),
        }
    }
}

trait CompressionStrategy {
    fn compress(&self, data: &str) -> String;
}

struct ZipCompress;

impl CompressionStrategy for ZipCompress {
    fn compress(&self, data: &str) -> String {
        format!("{} zip compressed", data)
    }
}

struct GCompress;

impl CompressionStrategy for GCompress {
    fn compress(&self, data: &str) -> String {
        format!("{} gzip compressed", data)
    }
}

enum Animal {
    Dog,
    Cat,
}

impl Animal {
    fn speak(&self) -> &'static str {
        match self {
            Animal::Dog => "dubl woof",
            Animal::Cat => "Moot meow",
        }
    }
}

struct AnimalFactory;

impl AnimalFactory {
    fn create_animal(ani_type: &str) -> Option<Animal> {
        match ani_type {
            "dog" => Some(Animal::Dog),
            "cat" => Some(Animal::Cat),
            _ => None,
        }
    }
}
