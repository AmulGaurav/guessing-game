use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!\n");

    // generates a random number between 1 & 100 (inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    let mut number_of_guess = 1;

    loop {
        println!("Please input your guess:");

        // creates an empty instance of String
        let mut guess = String::new();

        //  takes user input and appends it to the `guess` variable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        //  parse unsigned-integer from `guess` variable (user-input)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid input\n");
                continue;
            }
        };

        // compare `secret_number` variable with `guess` variable
        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("You won! ({number_of_guess} guesses)");
                break;
            }
            Ordering::Greater => println!("too big!\n"),
            Ordering::Less => println!("too small!\n"),
        }

        number_of_guess += 1;
    }
}
