use std::clone::Clone;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: f64,
    height: f64,
}


impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, val: f64) {
        self.width *= val;
        self.height *= val;
    }

    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let p = Point { x: 1, y: 2 }; // p is stored on the stack.

    //let p2 = p.clone();
    let p2 = p;
    let p3 = &p2;

    println!("{p:#?}");

    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.area(), 1.02);
    println!("Tests passed!");


    let one = Box::new(1);
    let two = Box::new(2);
    assert_eq!(*sum_boxes(&one, &two), 3);

    let pi = Box::new(3.14159265358979);
    let e = Box::new(2.718281828459045);
    assert_eq!(*sum_boxes(&pi, &e), 5.859874482048835);
    assert_eq!(eq_boxes(&one, &two), false);
    assert_eq!(higher_boxes(&one, &two), false);
}

fn sum_boxes<T: Add<Output = T> + Clone>(a: &Box<T>, b: &Box<T>) -> Box<T> {
    //let z = (**a).clone();
    let mut c = Box::new((**a).clone() + (**b).clone());
    c
}

fn eq_boxes<T: PartialEq>(a: &Box<T>, b: &Box<T>) -> bool {
    **a == **b
}

fn higher_boxes<T: PartialEq + PartialOrd>(a: &Box<T>, b: &Box<T>) -> bool {
    **a > **b
}
