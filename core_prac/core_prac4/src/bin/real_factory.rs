fn main() {
    println!("Hello from factory");
    // create enums based on the choices
    // create behaviour that is expected from the choices
    // impl the behaviour on the enum
    // create struct ClassFactory that implements create_class
    // method. Create method takes input of some type and
    // tries to return the enum variant based on the input
    // then on the variant we can use the behaviour method
    if let Some(nani) = AnimalFactory::make_animal("lion") {
        println!("{}", nani.growl());
    }
}

enum Animal {
    Deer,
    Lion,
}

impl Animal {
    fn growl(self) -> &'static str {
        match self {
            Animal::Deer => "No Sound",
            Animal::Lion => "Super Growl",
        }
    }
}

struct AnimalFactory;

impl AnimalFactory {
    fn make_animal(anim: &str) -> Option<Animal> {
        match anim {
            "lion" => Some(Animal::Lion),
            "deer" => Some(Animal::Deer),
            _ => None,
        }
    }
}
