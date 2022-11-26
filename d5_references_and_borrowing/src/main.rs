fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1); // reference to a string - not taking ownership
    println!("The length of '{}' is {}.", s1, len);
    change(&mut s1);
    println!("{}",s1);
}

fn calculate_length(s: &String) -> usize {
    // we passed a reference, which is immutable by default - we cannot modify s1 in this function
    let length = s.len();
    length
}

fn change(some_string: &mut String) {
    // we can modify s1 here because it is a mutable variable and we passed a mutable reference
    some_string.push_str(", world!");
    // NOTE: we can only have one mutable reference to (each bit of) data in scope at any one time
}

/*
fn multiple_mutable_borrows() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    // "Cannot borrow 's' as mutable more than once at a time: 25"
    // Look at line 26 - the error was thrown before the multi-line comment was added.

    println!("{}, {}", r1, r2);
}
 */

/*
fn dangle() -> &String {
    let s = String::from("hello");

    &s

    // The compiler prevents this as it would not be memory-safe.
    // The 's' variable would be dropped out of scope when the function it was declared in ends.
}
 */