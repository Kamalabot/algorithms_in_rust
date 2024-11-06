#![allow(dead_code)]

fn main() {
    println!("Startegy Pattern");
    let strat1 = ZipCompresser;
    let your_data1 = "yourdata";
    strat1.compress(your_data1);
    let strat2 = GzipCompresser;
    let your_data2 = "yourdata gzip";
    strat2.compress(your_data2);
}

trait Compression {
    fn compress(&self, data: &str);
}

struct ZipCompresser;

impl Compression for ZipCompresser {
    fn compress(&self, data: &str) {
        println!("Implementing Zip Compression algo on {data}")
    }
}

struct GzipCompresser;

impl Compression for GzipCompresser {
    fn compress(&self, data: &str) {
        println!("Implementing GZip Compression algo on {data}")
    }
}
