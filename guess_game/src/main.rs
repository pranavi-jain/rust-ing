use std::cmp::Ordering;

use std::io;

use rand::Rng;

fn main() {
    println!("Guessing Game!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("Shhh, secret number is: {secret_num}");

    loop {
        println!("Please input your number:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You guessed it!");
                break;
            }
        }
    }

}
