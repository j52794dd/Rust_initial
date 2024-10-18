fn main() {
    let mut a: f32 = 0.671985;
    let mut b = 0b1111_0000u8;
    let mut c: u16 = 0x23f9;

    println!("a is {2:<>9.6}, b is {0:2>16b}, c is {1:08o}", b, c, a);

    println!("Hello, world! {a}");

    let mut a = 13;
    let b = 2.3;
    let c: f32 = 120.0;

    let average = ((a as f64) + (b as f64) + (c as f64)) / 3.0;
    assert_eq!(average, 45.1);
    println!("Test passed!");

    let mut matrix: [[i32; 3]; 3];
    matrix = [[0; 3]; 3];
    println!("{}", matrix[0][1]);
    matrix[0][1] = 5;
    println!("{:?}", matrix[0][1]);

    let mut cub: [[[i64; 4]; 4]; 4] = [[[0; 4]; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                println!("cub[i][j] {}", cub[i][j][k]);
            }
        }
    }

    let tup = ('a', 65, 34.0);
    println!("{}", tup.2);

    let mut PI = "pi";
    let mut cars: [(f32, i32, &str); 3] = [(7.0, 7, "o"); 3];
    cars[0] = (3.1415, 3, "pi");
    println!("{}", cars[0].2);

    let mut x = 1;
    let mut y = 2;
    for i in 0..y{
        println!("FCSB {}", i);
    }
    //x = y = 2;
    print_sum(x, y);
    print_sum2(x as i16, y as i16);
    println!("{}", print_usize(x));
}

fn print_sum(x: i8, y: i8) {
    println!("{}", x + y);
}

fn print_sum2(x: i16, y: i16) {
    println!("{}", x + y);
}

fn print_usize(x: i8) -> usize {
    if x % 2 == 1 {
        return (x + 1) as usize;
    }else{
        return x as usize;
    }
    println!("FCSB");
}
