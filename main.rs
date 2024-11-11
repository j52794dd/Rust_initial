use std::cmp::max;
use std::collections::HashMap;
use std::fs;
use std::env;
use std::process::exit;

fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    println!("{:?}", v);

    let args: Vec<String> = env::args().collect();
    if args.len() < 3{
        println!("Wrong number of arguments");
        exit(1);
    }
    let x = match args[2].parse::<u32>(){
        Ok(num) => num,
        Err(e) => { println!("Error: {}", e); exit(1); }
    };
    let filename = String::from(&args[1]);
    let text = fs::read_to_string(filename);
    let text = match text{
        Ok(text) => text.to_lowercase(),
        Err(error) => {
            println!("There was a problem opening the file: {:?}", error);
            exit(1);
        },
    };
    let text = text.split(&['\n', '\r', ' ', '\t']).collect::<Vec<&str>>();
    //println!("{:?}", text);
    let mut hash_map = HashMap::new();
    for &item in text.iter(){
        let mut cur_str = item.trim().to_string();
        if cur_str == "" {
            continue;
        }
        let key = hash_map.entry(cur_str.clone()).or_insert(0);
        *key += 1;
    }
    //println!("{:?}", hash_map);
    let mut max_occurences = 0;
    for item in hash_map.iter(){
        max_occurences = max(max_occurences, *item.1);
    }
    println!("Maximum occurences: {}", max_occurences);
    let mut sol = Vec::new();
    for item in hash_map.iter(){
        if *item.1 == max_occurences {
            sol.push(item.0);
        }
    }
    println!("Sol for: {:?}", sol);
    top_5_words(&hash_map, x as usize);
}

fn top_5_words(hash_map: &HashMap<String, u32>, x: usize){
    let mut nums = HashMap::new();
    for (_, &value) in hash_map.iter(){
        nums.insert(value, 1);
    }
    let mut vect = vec![];
    for (&key, _) in nums.iter(){
        vect.push(key);
    }
    vect.sort();
    vect.reverse();
    let guard = if vect.len() >= x {x} else {vect.len()};
    let top_x_slice = vect[..guard].to_vec();
    println!("top {} words are:", x);
    for (key, &value) in hash_map.iter(){
        let mut contains = false;
        for &item in top_x_slice.iter(){
            if item == value{
                contains = true;
            }
        }
        if contains{
            println!("{} -> appearing: {} times", key, value);
        }
    }
}
