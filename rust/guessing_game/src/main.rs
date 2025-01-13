use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop{
        println!("please input your guess:");
        let mut guess = String::new();
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                if guess.trim() == "quit".to_string() || guess.trim() == "exit".to_string() {
                    println!("Exiting the game...");
                    break;
                } else {
                    println!("Invalid input. Please enter a number or 'exit' to quit.");
                    continue;
                }
            }

        };

        println!("You guessed : {}", guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You guessed correctly!");
                break;},
        }
    }
}