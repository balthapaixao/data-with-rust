use std::io;

fn main() {
    println!("Type any number!");
    let mut number_input = String::new();
    io::stdin()
        .read_line(&mut number_input)
        .expect("Failed to read input.");

    let number: i32 = number_input.trim().parse().unwrap();

    match number {
        1..=i32::MAX => println!("Positive"),
        i32::MIN..=-1 => println!("Negative"),
        0 => println!("Zero"),
    }

    match number {
        n if n > 0 => println!("Positive"),
        n if n < 0 => println!("Negative"),
        _ => println!("Zero"), // match requires all possible outcomes to be covered
    }
}
