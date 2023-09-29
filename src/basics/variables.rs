fn main() {
    let x: i32 = 5; // defining an immutable variable

    let mut y: i32 = 6; // defining a mutable variable

    const YOB: i32 = 1999; // const variables require a known type

    static mut AGE: u32 = 23; // this is a mutable static variable

    y = 10;

    println!("The value of y is {}", y);

    unsafe {
        AGE = 24;

        println!("My age is {}", AGE);
    }
}
