use std::io;
mod random_number_generator;

fn main() {
    // hello_world();
    // get_input_from_user();
    // create_mutable_variable();
    random_number_generator::generate_random_number();
}

fn get_input_from_user() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}

fn hello_world() {
    println!("Hello World!");
}

fn create_mutable_variable() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}