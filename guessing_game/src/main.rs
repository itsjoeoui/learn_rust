use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io; // io comes from std
             // we need this to get user input and output

fn main() {
    // entry point into the program
    println!("Hello, world!");

    // https://doc.rust-lang.org/reference/expressions/range-expr.html
    // 1..=100 is a range expression, =100 means inclusive

    let secret_number = thread_rng().gen_range(1..=100);
    println!("The random number is {secret_number}");

    loop {
        println!("Please input your guess: ");
        // variables are immutable by default
        // we use the "mut" keyword to make it mutable
        let mut guess = String::new(); // new() creates a new empty String
                                       // You can expect the new() methmods on many different types

        // the stdin() returns an instance of type std::io::Stdin
        // & indicates it is a reference, Rust makes it very safe to use reference
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // read_line returns a Result type.
        // Result is a Enum, its variants are Ok and Err (we can pattern match on them)
        // If this instance of Result is an Err, 'expect' will crash the program
        // and display the error message we defined.
        // If everything is Ok, then expect will return the value Ok is holding
        // this case, it is the number of bytes in the user's input
        // Always call expect otherwise you will get a warning

        // Rust allows us to shadow the previously defined guess variable
        // prase will parse the input based on the variable type, this case u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        println!("You guessed: {guess}"); // {} defines a placeholder

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
        }
    }
}
