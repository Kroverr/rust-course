// use std::io;
// mod random_number_generator;
// mod function_with_params;

// fn main() {
//     // hello_world();
//     // get_input_from_user();
//     // create_mutable_variable();
//     // random_number_generator::generate_random_number();
//     // for_loop_sample();
//     // greeting_sample();
//     function_with_params::argument_sample();
// }

// fn get_input_from_user() {
//     println!("Guess the number!");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {guess}");
// }

// fn hello_world() {
//     println!("Hello World!");
// }

// fn create_mutable_variable() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn for_loop_sample() {

//     for i in (1..=10).rev() {
//         println!("the value is: {i}");
//     }
// }

// fn greeting_sample(){
//     println!("Please enter a greeting:");
//     let mut name = String::new();
//     io::stdin().read_line(&mut name).expect("Failed to read input");

//     // Convert input to lowercase for case-insensitive matching
//     match name.trim().to_lowercase().as_str() {
//         "good bye" => println!("Sorry to see you go."),
//         "good morning" => println!("Goodmorning mate"),
//         "good evening" => println!("Goodevening too"),
//         "hello" => println!("Hi, nice to meet you!"),
//         _ => println!("I can't find a greeting, good bye."),
//     }
// }

// fn borrow_vec(vector: &Vec<i32>) {
//     // borrow and print; this does not take ownership
//     println!("Borrowed vector: {:?}", vector);
// }

// fn borrow_string(s: &String) {
//     // borrow and print; this does not take ownership
//     println!("Borrowed String {}", s);
// }

// fn own_vec(vector: &Vec<i32>) -> Vec<i32> {
//     // return a new vector that preserves the original contents
//     let mut new_vector = vector.clone();
//     new_vector.push(10);
//     new_vector
// }

// fn own_integer(x: &i32) -> i32 {
//     // return the incremented value; do not try to modify the borrow
//     *x + 1
// }

// fn own_string(s: &String) {
//     // borrow and print; this does not take ownership
//     println!("{}", s);
// }

// Borrowing is the mechanism by which Rust allows you to lend ownership of a variable to a function 
// or another part of your program without actually transferring ownership of the variable. 
// When you borrow a variable, you're essentially saying 
// "I want to use this variable for a little while, but I promise I won't modify it."
// fn main() {
//     let my_vec = vec![1, 2, 3, 4, 5];
//     let my_int = 10;
//     let my_string = String::from("Hello, world!");

//     // use returned value from own_integer
//     let incremented = own_integer(&my_int);
//     println!("original my_int: {}", my_int);
//     println!("returned incremented: {}", incremented);

//     // borrow the string (no move) and then still use it afterwards
//     own_string(&my_string);
//     println!("my_string is still usable: {}", my_string);

//     // borrow the vector and get a new owned vector back
//     let new_vector = own_vec(&my_vec);
//     println!("original vector: {:?}", my_vec);
//     println!("new vector: {:?}", new_vector);

//     borrow_vec(&my_vec);
//     borrow_string(&my_string);
// }

// Borrowing is a key concept in Rust because it allows you to write code that is both safe and efficient. 
// By lending ownership of a variable instead of transferring it, Rust ensures that only 
// one part of your program can modify the variable at a time, which helps prevent 
// bugs and makes it easier to reason about your code.


// use std::fs::File;
// use std::io::{BufRead, BufReader};

// fn main() {
//     let file = File::open("non_existent_file.txt");
//     let file = match file {
//         Ok(file) => file,
//         Err(error) => {
//             match error.kind() {
//                 std::io::ErrorKind::NotFound => {
//                     // If file not found, create it with some default content then open it
//                     if let Err(e) = std::fs::write("non_existent_file.txt", "This file was created because it was missing.") {
//                         eprintln!("Failed to create file: {}", e);
//                         std::process::exit(1);
//                     }
//                     File::open("non_existent_file.txt").expect("Failed to open file after creating it")
//                 }
//                 std::io::ErrorKind::PermissionDenied => {
//                     eprintln!(
//                         "Permission denied opening 'non_existent_file.txt': {}. \
//                         Check file permissions or run with appropriate privileges.",
//                         error
//                     );
//                     std::process::exit(1);
//                 }
//                 _ => {
//                     eprintln!("Error opening file: {}", error);
//                     std::process::exit(1);
//                 }
//             }
//         }
//     };

//     // Optionally write to a separate output file
//     write_to_file("output.txt", "Hello, world!!!!");

//     let reader = BufReader::new(file);
//     for line in reader.lines() {
//         match line {
//             Ok(line) => println!("{}", line),
//             Err(error) => {
//                 eprintln!("Error reading line: {}", error);
//                 std::process::exit(1);
//             }
//         }
//     }
// }

// //create a function that writes to a file and handles errors using the match statement
// fn write_to_file(filename: &str, content: &str) {
//     let result = std::fs::write(filename, content);
//     match result {
//         Ok(_) => println!("Successfully wrote to {}", filename),
//         Err(error) => {
//             match error.kind() {
//                 std::io::ErrorKind::NotFound => {
//                     eprintln!("File not found when writing to '{}': {}", filename, error);
//                     return;
//                 }
//                 std::io::ErrorKind::PermissionDenied => {
//                     eprintln!("Permission denied writing to '{}': {}. Check permissions.", filename, error);
//                     return;
//                 }
//                 _ => {
//                     eprintln!("Error writing to file '{}': {}", filename, error);
//                     return;
//                 }
//             }
//         }
//     }
// }

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

fn main() {
    // Build the path to src/non_existent_file.txt
    let mut file_path = std::env::current_dir().expect("Failed to get current directory");
    file_path.push("src");
    file_path.push("non_existent_file.txt");

    let file = File::open(&file_path);
    let file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    // If file not found, create it with some default content then open it
                    if let Err(e) = std::fs::write(&file_path, "This file was created because it was missing.") {
                        eprintln!("Failed to create file: {}", e);
                        std::process::exit(1);
                    }
                    File::open(&file_path).expect("Failed to open file after creating it")
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };
    
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}