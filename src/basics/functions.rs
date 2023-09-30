use rand::Rng;
use std::io;

// a function with no inputs
fn say_hello() {
    println!("Just saying hi!");
}

fn guess_number() {
    let mut rng = rand::thread_rng();
    let secret_number = rng.gen_range(1..11);
    loop {
        println!("Please input your guess (1 - 10).");
        let mut input_guess = String::new();
        io::stdin()
            .read_line(&mut input_guess)
            .expect("Failed to read input");
        let input_guess: i32 = match input_guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {}", input_guess);
        if (input_guess == secret_number) {
            println!("You win!");
            break;
        } else if (input_guess < secret_number) {
            println!("Sorry, too low...");
        } else {
            println!("Sorry, too high...");
        }
    }

    println!("The secret number is: {}", secret_number);
}

fn add_numbers(a: i32, b: i32) -> i32 {
    let result = a + b;
    return result;
}

//returning multiple values
fn divide_numbers(a: i32, b: i32) -> (i32, bool) {
    if (b == 0) {
        return (0, false);
    }
    let result = a / b;
    return (result, true);
}

//optional parameters
fn multiply_numbers(a: i32, b: Option<i32>) -> i32 {
    let result = if let Some(b) = b { a * b } else { a };
    return result;
}

fn divide_numbers_two(a: i32, b: i32) -> Option<i32> {
    if (b == 0) {
        return None;
    } else {
        return Some(a / b);
    }
}

fn main() {
    say_hello();
    guess_number();

    let add_result = add_numbers(25, 32);
    println!("The result of my sum function is {}", add_result);

    let div_result = divide_numbers(25, 32);
    println!("The result of my divide function is {:?}", div_result);

    let mult_result_one = multiply_numbers(2, Some(5));
    println!(
        "The result of the first multiplication is {}",
        mult_result_one
    );

    let mult_result_two = multiply_numbers(2, None);
    println!(
        "The result of the second multiplication is {}",
        mult_result_two
    );

    let a = 10;
    let b = 0;
    let div_two_result = divide_numbers_two(a, b);
    match div_two_result {
        Some(c) => println!("Result {}", div_two_result),
        None => println!("Cannot divide by zero!"),
    }
}
