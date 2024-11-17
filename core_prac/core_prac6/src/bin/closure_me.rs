#![allow(warnings)]
#![allow(dead_code)]

#[derive(Debug)]
struct PurePhilopsopher {
    name: String,
}

#[derive(Debug)]
struct ClonePhilopsopher {
    name: String,
}

fn main() {
    let sp1 = PurePhilopsopher {
        name: "philo1".to_owned(),
    };

    let cl1 = || sp1.name.len();

    println!("Length is {:?}", cl1());

    let philoname = "Cphilo".to_owned();
    let cl2 = || philoname.len();
    println!("Length is {:?}", cl2());
    println!("Length is {:?}", cl2());

    let philoname = 56;
    let cl3 = move || {
        let yname = philoname;
        yname
    };
    println!("Length is {:?}", cl3());
    // println!("Length is {:?}", cl3());

    let mut philomut = "Cphilo".to_owned();

    let mut clmut = || {
        philomut = "got_mutated".to_owned();
        println!("New philomut is {philomut}");
    };
    clmut();
}
