// std -> standard library
// ::io -> io module under the standard library
use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    // let for creating a variable
    // bydefault variables in immutable; for making them mutable we use mut  keyword
    // if we don't specify the variable type, rust uses type inferencing to derive  the type for simple data types
    // String is a type here, provided by standard library called growable
    // ::new() is an  associated function
    // String::new() creates an empty instance of string
    println!("guess the number game");
    let num = rand::rng().random_range(1..=100);

    loop {
        let mut guess: String = String::new();
        println!("Enter any number: ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
        }
    }
}
