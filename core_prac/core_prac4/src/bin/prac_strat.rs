fn main() {
    println!("Strategy Practice");
    // create a trait with method signature
    // impl the trait on different structs / enums
    // based on the strategies used in th structs
    // trait method will be implemented differently
    // When the structs are created based on the
    // requirement, their corresponding method is called
    let m1 = Mobile;
    m1.pressit("new Mobile");
    let m2 = Rocket;
    m2.pressit("new rocket");
}

trait CompressionStrategy {
    fn pressit(&self, data: &str);
}

struct Mobile;

impl CompressionStrategy for Mobile {
    fn pressit(&self, data: &str) {
        println!("{} is compressed with Mobile", data)
    }
}

struct Rocket;

impl CompressionStrategy for Rocket {
    fn pressit(&self, data: &str) {
        println!("{} is compressed with Rocket", data)
    }
}
