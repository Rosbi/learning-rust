use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let random_number = rand::thread_rng().gen_range(1, 101);

    println!("Input your guess: ");
    loop{
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("Input a *number*");
                continue;
            }
        };

        match guess.cmp(&random_number){
            Ordering::Less    => println!("Too low. Try again"),
            Ordering::Greater => println!("Too high. Try again"),
            Ordering::Equal   => {
                println!("You guessed right. The number was {}.", random_number);
                break;
            }
        }
    }
}
