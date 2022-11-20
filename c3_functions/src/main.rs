fn main() {
    not_main();

    let sum = my_addition(4,2);
    println!("\nThe two numbers added together equal {}", sum);

    my_control_flow();

    let counter = loop_function();
    println!("\nThe counter reached {}\n", counter);

    my_while_loop();

    my_for_loop();
}

fn not_main() {
    println!("Another function.");
}

fn my_addition(x: i32, y:i32) -> i32 {
    let sum = x + y;
    // return sum
    sum // This will be implicitly returned if it is the last value
}

fn my_control_flow() {
    let number = 18;

    if number < 10 {
        println!("\nThe number was less than 10!");
    } else if number > 15 {
        println!("\nThe number was larger than 15!");
    } else {
        println!("\nThe number was between 10 and 15!");
    }
}

fn loop_function() -> i32 {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break;
        }
    }
    counter
}

fn my_while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("Happy New Year!!\n");
}

fn my_for_loop() {
    let my_numbers = [10, 20, 30, 40, 50];

    for element in my_numbers.iter() {
        println!("The value of this element is: {}", element);
    }
}