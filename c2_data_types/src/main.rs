fn main() {
    // NOTE: In this program, all variable names are prefixed with an underscore.
    // This is to signal to the compiler that the variables are intentionally not being used.
    // If the underscore was not present, the compiler would complete the build, but emit a warning
    // for each variable that had not been used.

    println!("Rust has two categories of data types; scalar types, and compound types.");
    println!("Scalar types will contain a single value.");
    println!("Compound types will contain multiple values.");


    // SCALAR TYPES

    println!("\n\nFirst, we will look at the scalar types in Rust.");
    println!("They are; integers, floating-point numbers, booleans, and characters.");

    // Integers
    println!("\nAn integer will have a single whole number.");
    println!("Integers can be of various sizes - ranging from 8-bit, to 128-bit.");
    println!("Signed integers can be positive or negative. Unsigned integers are always positive.");
    println!("Rust will default to signed 32-bit integers for numbers.");

    let _a = 98_222; // Decimal
    let _b = 0xff; // Hex
    let _c = 0o77; // Octal
    let _d = 0b1111_0000; // Binary
    let _e = b'A'; // Byte (unsigned 8-bit only)
    // let _f: u8 = 256; // uncomment this line to see a compiler error
    // this error is because 256 is too large to fit in the size specified
    // in debug, Rust will not allow the variable to overflow
    let _f: u8 = 255;

    // Floating-point numbers
    println!("\nA floating point variable will contain a single number.");
    println!("Rust will default to 64-bit floating-point numbers.");

    let _g = 19.4;

    // Booleans
    println!("\nA boolean represents the value 'true' or 'false'.");

    let _t = true;

    // Characters
    println!("\nA character represents a single-digit variable.");
    println!("Characters can be letters, numbers, punctuation, or other characters.");

    let _h = 'h';
    let _i = 'i';


    // COMPOUND TYPES

    println!("\n\nNow, let's look at compound types.");
    println!("\nThey are: tuples; arrays; vectors");

    // Tuples
    println!("\nA tuple is a fixed-size array of related data - possibly of different types.");

    let _tup = ("I am learning Rust, which for me is programming language", 5);

    // Arrays
    println!("\nArrays are fixed-size and can only contain variables of a single type.");

    let _error_codes = [404, 429, 500];
    let _http_not_found = _error_codes[0]; // this sets the variable to '404';

    // Vectors
    println!("\nVectors are resizable arrays.");

    let mut _my_vector = vec![1i32, 2, 3];
}
