use std::io;
mod random_number_generator;
mod function_with_params;

fn main() {
    // hello_world();
    // get_input_from_user();
    // create_mutable_variable();
    // random_number_generator::generate_random_number();
    // for_loop_sample();
    // greeting_sample();
    function_with_params::argument_sample();
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

fn for_loop_sample() {

    for i in (1..=10).rev() {
        println!("the value is: {i}");
    }
}

fn greeting_sample(){
    println!("Please enter a greeting:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    // Convert input to lowercase for case-insensitive matching
    match name.trim().to_lowercase().as_str() {
        "good bye" => println!("Sorry to see you go."),
        "good morning" => println!("Goodmorning mate"),
        "good evening" => println!("Goodevening too"),
        "hello" => println!("Hi, nice to meet you!"),
        _ => println!("I can't find a greeting, good bye."),
    }
}

