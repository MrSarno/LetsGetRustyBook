fn main() {
    let x = 5; // immutable by default
    println!("The value of x is: {}", x);
    // x = 7; // cannot assign twice to immutable variable

    let mut y = 12;
    println!("\nThe value of y is: {}", y);
    y = 8;
    println!("The value of y has been changed to: {}", y);

    const ALL_UPPER_CASE_NAME: u32 = 100000;
    println!("\nMy const's value is {}", ALL_UPPER_CASE_NAME);
    println!("As a constant, it cannot be made mutable.")
}
