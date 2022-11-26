fn main() {
    let my_string = String::from("hi mom");
    first_word_v1(&my_string);

    let mut s = String::from("hello world");
    let _hello = &s[0..5];
    // we can omit the '0', as that will be assumed if no starting point is provided
    let _world = &s[6..11];
    // we can omit the '11', as it will default to the final character if no end point is chosen
    let word = first_word_v2(&s);
    println!("The first word is: {}", word);
    s.clear();
    arrays_and_slices();
}

// PROBLEM: our return value is not tied to the word itself.
// For example, clearing the string will not reset the length value.
// This means we need to manually keep our return value in sync, which is error-prone.
fn first_word_v1(s: &String) -> usize { // takes in a reference as we don't want to claim ownership
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i
        }
    }

    s.len()
}

fn first_word_v2(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn arrays_and_slices() {
    let a = [1, 2, 3, 4, 5];
    let _slice = &a[0..2];
}