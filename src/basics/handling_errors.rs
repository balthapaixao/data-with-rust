// Divide either returns an i32 or a Sring
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if (b == 0) {
        return Err("Cannot divide by zero".to_string());
    } else {
        return Ok(a / b);
    }
}

fn main() {
    // Any call to divide, needs to account for all cases defined in Result
    match divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(error) => println!("Error: {}", error),
    }
}
