use std::fmt;
use std::fmt::{write, Formatter};

struct Satellite {
    name: String,
    velocity: f64,
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} with velocity {}", self.name, self.velocity)
    }
}

impl PartialEq for Satellite {
    fn eq(&self, other: &Satellite) -> bool {
        self.velocity == other.velocity
    }
}

trait Altitude {
    fn altitude(&self) -> f64;
}

impl Altitude for Satellite {
    fn altitude(&self) -> f64 {
        let G = 6.667430 * (1e-11);
        let M = 5.972 * 1e24;
        let Rearth = 6.371 * 1e6;
        let v = self.velocity;
        G * M / (v * v) - Rearth
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };

    let galileo = Satellite {
        name: String::from("Galileo"),
        velocity: 7905.60897,
    };

    println!("hubble is {}", hubble);
    println!("hubble is equal to galileo: {}", galileo == hubble);
    println!("hubble's altitude is {}", hubble.altitude());
    println!("galileo's altitude is {}", galileo.altitude());
}
