use rand::thread_rng; // imported thread_rng from the rand crate
use rand::Rng; // imported a trait Rng which allows gen_range to have the Range trait
use std::cmp::Ordering; // captures the Result of comparision values
use std::io; // importing I/O of the standard rust lib

pub fn exec() {
    println!("<<<NUMBER GUESSING GAME>>>");

    // generating a random number with Rng trait
    // and storing it in an immutable variable, because this will only be used for comparision
    let secret_number = thread_rng().gen_range(1..=10);

    // specifying an infinite loop, will break or continue conditionally
    // using a loop to allow the user to guess multiple times unless breaking out of the loop...
    // ...with a correct guess
    loop {
        println!("please guess a number..");

        // assign an empty string to a mutable variable
        let mut guess = String::new();

        // use stanard command line user i/o operation, we read a string into {guess}
        // the reason for having guess mutable is to allow the read_line to store a user input into it
        // we passed {guess} as a reference to read_line for allowing a user input to be stored into it
        // we use pattern matching for Ok or Err Result types
        match io::stdin().read_line(&mut guess) {
            Ok(_) => {
                println!("you have guessed {}", guess);
            }
            Err(error) => println!("Error {error}"),
        };

        // in order to compare the {secret_number} with {guess}, {guess} is required to be parsed to an integer
        // here we use shadowing to store parsed {guess} value to {guess} of an integer
        // this will prevent us from specifying another variable just for storing the parse integer value
        // parsing to unsigned integer (non negative)
        // again using pattern matching on {guess} for...
        let guess = match guess.trim().parse::<u32>() {
            // ...returning a correctly a parsed string to an unsigned integer
            Ok(num) => num,

            // ..or just continuing the loop if a string wasn't parsable
            // the {_} is a catchall, simply drops any value assigned into it
            Err(_) => continue,
        };

        // once {guess} is parsed to u32 integer, it's compared with the {secret_number}...
        // ...where the {secret_number} is passed by reference to cmp function
        match guess.cmp(&secret_number) {
            // Ordering will store the result of the comparision..
            // and will allow to execute any expression of match success
            Ordering::Less => println!("it\'s small!"),
            Ordering::Greater => println!("it\'s to big!"),
            Ordering::Equal => {
                println!("you have guessed it!");
                // break out of loop on a correct guess
                break;
            }
        }
    }
}
