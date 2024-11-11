use std::any::type_name;
use std::ops::Deref;
use std::fmt;
use std::fmt::Formatter;

struct Shuttle<'a>{
    name: &'a str,
}

impl<'a, 'b> Shuttle<'a> {
    fn send_transmission(&self, msg: &str) -> &str{
        println!("Transmission message: {}", msg);
        self.name
    }
    fn send_message(&self, msg: &'b str) -> &'b str{
        msg
    }
}

fn main() {
    println!("Hello, world!");
    let result;
    let x = String::from("hello");
    {
        let mut y = x.clone();
        y.push_str(", world!");
        result = longer_str(&x, &y);
    }
    println!("result: {}", result);
    let mut vehicle: Shuttle;
    {
        let comm = String::from("fcsb e steaua");
        vehicle = Shuttle{name: "234"};
    }
    let sender = vehicle.send_transmission("Greetings! ");
    println!("sender: {}", sender);

    let my_number = 8u8;
    match my_number {
        0 | 5 => {println!("zero"); println!("zero")},
        1 => {println!("one"); println!("one")},
        2 => {println!("two"); println!("two")},
        3..9 => {println!("three"); println!("three")},
        default=> {println!("default={}", default)},
    }

    let countdown = [5,4,3,2,1];
    let Some(&number) = countdown.get(4) else { todo!() };
    //let number = *number + 1;
    //println!("number is of type : {}",  type_of(*(number.unwrap())));
    println!("number: {}", number);


    let address = Location::Unknown;
    //address.display();
    println!("{}", address);
    let address = Location::Anonymous;
    //address.display();
    println!("{}", address);
    let address = Location::Known(28.608295, -80.604177);
    //address.display();
    println!("{}", address);


}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Location::Unknown => write!(f, "Unknown Location"),
            Location::Anonymous => write!(f, "Anonymous Location"),
            Location::Known(x, y) => write!(f, "Known Location (latitude and longitude) : ({}, {})", x, y),
        }
    }
}

enum Location{
    Unknown,
    Anonymous,
    Known(f64, f64),
}

impl Location{
    fn display(&self){
        match self {
            Location::Unknown => println!("Unknown Location"),
            Location::Anonymous => println!("Anonymous Location"),
            Location::Known(x, y) => println!("Known Location (latitude and longitude) : ({}, {})", x, y),
        };
    }
}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn longer_str<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
    x
}
