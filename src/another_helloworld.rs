use std::io;

fn main() {
    println!("What is your name?");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input.");

    println!("What is your gender? (Male, Female, Non-binary)");
    let mut gender = String::new();
    io::stdin()
        .read_line(&mut gender)
        .expect("Failed to read input.");

    if gender.trim() == "Male" {
        println!(
            "Hello, Mr. {}! Welcome to the world of Rust!!!",
            name.trim()
        )
    } else if gender.trim() == "Female" {
        println!(
            "Hello, Ms. {}! Welcome to the world of Rust!!!",
            name.trim()
        )
    } else {
        println!(
            "Hello, Mx. {}! Welcome to the world of Rust!!!",
            name.trim()
        )
    }
}
