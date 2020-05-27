// By default, Rust brings only a few types into the scope of every
// program in "the prelude".
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Guess the number.");
        println!("Input:");

        // Mutable variable
        // String is a string type provided by the standard library that
        // is a growable, UTF-8 encoded bit of text.
        let mut guess = String::new(); // new is a static method of String

        // References are immutable by default. Hence, you need to write
        // &mut guess rather than &guess to make it mutable.
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        // read_line() returns a io::Result (enumeration, Ok/Err)

        // Shadow previous guess variable
        let guess: u32 = guess.trim().parse().expect("Please type in a number!");

        match guess.cmp(&number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => println!("You win!"),
        }       
    }
}
