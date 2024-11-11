use std::env;
use std::fmt::format;
use std::fs;
use std::io::{ Write};

fn main() {
    if env::args().len() < 3 {
        println!("Number of arguments provided not enough");
        return;
    }
    let args: Vec<String> = env::args().collect();
    let mut path = args[1].clone();
    let mut name = args[2].clone();
    println!("File Path is: {:?}", path);
    for (index, line) in fs::read_to_string(&path).expect(&format!("Could not read {}", path)).lines().enumerate() {
        if line.eq(name.as_str()) {
            fs::write("result.txt", &format!("Name found at position {}", index)).expect(&format!("Could not write {}", "result.txt"));
            return;
       }
    }
    fs::write("result.txt", &format!("{} not found, added to the end of the text file", name)).expect(&format!("Could not write {}", name));
    let mut file_handle = fs::OpenOptions::new().append(true).open(path).expect("Could not open file");
    file_handle.write(b"\n");
    file_handle.write(name.as_bytes()).expect("Could not write to file");

}

