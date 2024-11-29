use std::io;
use guessing_game::Guess;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess the number!");

    let secret_number= rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {}", secret_number);

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // note that the println macro is special
        // it takes this by reference even though we did not use the & operator
        println!("You guessed: {}", guess );
        if guess.trim() == "quit" {
            println!("Quitting...");
            break;
        }

        // convert the string to a number
        let guess: Guess = match guess.trim().parse() {
            Ok(num) => match Guess::new(num) {
                Ok(guess) => guess,
                Err(msg) => {
                    println!("{}", msg);
                    continue;
                },
            },
            Err(_) => panic!("Please type a number!"),
        };
                           

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
    
}