fn main() {
    let x = 5;
    let _y = x; // Copy

    let s1: String = String::from("hello");
    let _s2: String = s1; // this is a Move instead of a Shallow Copy for performance reasons
    // println!("{}, world!", s1); // the compiler will reject this
    // that is because s1 has been invalidated to ensure memory safety

    let s3: String = String::from("hello");
    let _s4: String = s3.clone();
    println!("{}, world!", s3); // this works now, as we've told Rust not to just move from s3 to s4
}
