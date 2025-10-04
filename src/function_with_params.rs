use std::io;

fn sum_2(numbers: &[i32]) -> f64 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    
   result as f64 / numbers.len() as f64
}

pub fn argument_sample() {
    // Get User Input
    println!("Enter number of elements:");
    let mut elem_count = String::new();
    io::stdin()
        .read_line(&mut elem_count)
        .expect("Failed to read line");

    let count: usize = match elem_count.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number entered.");
            return;
        }
    };

    let mut numbers = Vec::new();
    for i in 1..=count {
        println!("Enter number {}:", i);
        let mut num_input = String::new();
        io::stdin().read_line(&mut num_input).expect("Failed to read line");
        match num_input.trim().parse::<i32>() {
            Ok(num) => numbers.push(num),
            Err(_) => {
                println!("Invalid number, please try again.");
                continue;
            }
        }
    }

    let result = sum_2(&numbers);
    println!("The sum is {}", result);
    // numbers now contains all user input
}
