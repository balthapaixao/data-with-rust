use std::io;

fn main() {
    println!("Type any number!");
    let mut number_input = String::new();
    io::stdin()
        .read_line(&mut number_input)
        .expect("Failed to read input.");

    let number: i32 = number_input.trim().parse().unwrap();

    if number < 0 {
        println!("Negative");
    } else if number > 0 {
        println!("Positive");
    } else {
        println!("Zero");
    }
}
