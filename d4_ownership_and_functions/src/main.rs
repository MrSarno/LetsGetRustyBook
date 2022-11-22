fn main() {
   let s: String = String::from("variable_s");
    takes_ownership(s);
    // println!("{}", s); the compiler will not allow this

    let x: i32 = 5;
    makes_copy(x);
    println!("[main] {}.", x); // but this is fine

    let s1: String = gives_ownership();
    let s2: String = String::from("variable_s2");
    let s3: String = takes_and_gives_back(s2);
    println!("[main] s1 = {}, s3 = {}", s1, s3);
}

fn takes_ownership(some_string: String) {
    println!("[takes_ownership] {}.", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("[makes_copy] {}.", some_integer);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}