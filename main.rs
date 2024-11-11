use rand::prelude::*;
use std::io;
fn main() {
    let mut counter = 0;
    println!("You have 10 chances");
    let mut rng = rand::thread_rng().gen_range(1..101);
    loop{
        counter += 1;
        println!("Please enter an integer between 1 and 100... ?");
        let mut buffer = String::new();
        //io::stdin().read_line(&mut buffer).unwrap();
        let result = io::stdin().read_line(&mut buffer);
        let mut guess = match result {
            Ok(_) => buffer.trim().parse().unwrap_or_else(|_| -1),
            Err(_) => -2,
        };
        if(guess == -2){
            println!("Sorry for inconvenience! You can type again a number!");
            counter -= 1;
        }else if(guess == -1) {
            println!("Guess invalid, please type a number between 1 and 100");
        }else if (guess > rng){
            println!("Too high\n");
        }else if(guess < rng){
            println!("Too low\n");
        }
        else{
            println!("You win\n");
            break;
        }
        if(counter > 10){
            println!("You lose\n");
            break;
        }
    }
    println!("End of the game\n");
}
