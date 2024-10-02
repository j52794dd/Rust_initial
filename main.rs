fn main() {
    let mut a: f32 = 0.671985;
    let mut b = 0b1111_0000u8;
    let mut c: u16 = 0x23f9;

    println!("a is {2:<>9.6}, b is {0:016b}, c is {1:08o}", b, c, a);

    println!("Hello, world!");

    let mut a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = ((a as f64) + (b as f64) + (c as f64)) / 3.0;
    assert_eq!(average, 45.1);
    println!("Test passed!");
}
