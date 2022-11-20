fn main() {
    { // s is not valid here, as it is not yet declared
        let _s: &str = "hello"; // s is now valid
        // do stuff with s
    } // s goes out of scope, and is no longer valid

    /*
    the above is a string literal
    the below is a string
     */

    {
        let _s: String = String::from("hello");
    }
}
