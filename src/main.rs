use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let quit = "quit";
    let heart_eyed_cat = '😻';
    println!("Guess the number {heart_eyed_cat}");

    let secret_number = rand::thread_rng().gen_range(0..=101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess \nor type QUIT to exit. {heart_eyed_cat}");

        let mut guess = String::new();  // new() is an associated function of String type.

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        if guess.trim().to_lowercase() == quit {
            break;
        } 
        
        let guess: i32 = match guess.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! {heart_eyed_cat}");
                break;
            }
        }
    }
}
