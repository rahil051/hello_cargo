use std::io;

pub mod guessing_number;
pub mod sequences;

fn main() {
    // guessing_number::exec();

    // take user input
    println!("input an nth_number for the fib seq");
    let mut nth_number = String::new();
    io::stdin()
        .read_line(&mut nth_number)
        .expect("failed to read line");

    let nth_number = nth_number.trim().parse::<u32>().expect("invalid parsing");
    let result = sequences::fib_of_nth_number(nth_number);
    println!("{0}", result);
}
