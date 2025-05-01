use std::io;

fn main() {
    let mut inp = String::new();
    println!("Enter a number: ");
    let num1: i32;
    let num2: i32;
    let sum: i32;
    /*
    // One way of reading the parsing the inputs.
    // Note: expect() will panic if there is an error. Thus, avoid using the expect function.
    // @ref: https://doc.rust-lang.org/std/io/type.Result.html#method.expect
    io::stdin().read_line(&mut inp).expect("Failed to read line");
    num1 = inp.trim().parse().expect("Please enter a number");

    // after each readline, clear the inp string. Without this, the input will be
    // appended to the string and thus lead to invalid string.
    // @ref: https://doc.rust-lang.org/std/io/trait.BufRead.html#method.read_line
    inp.clear();
    println!("Enter another number: ");
    io::stdin().read_line(&mut inp).expect("Failed to read line");
    num2 = inp.trim().parse().expect("Please enter a number");
    */
    if io::stdin().read_line(&mut inp).is_err() {
        println!("failed to readline");
        return;
    }
    match inp.trim().parse::<i32>() {
        Ok(num) => {
            num1 = num;
        }
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    }
    println!("Enter another number: ");
    inp.clear();
    if io::stdin().read_line(&mut inp).is_err() {
        println!("failed to readline");
        return;
    }
    // ::<type> is the turbofish syntax that tells what type we want the string slice
    // to be parsed.
    match inp.trim().parse::<i32>() {
        Ok(num) => num2 = num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    }
    sum = num1 + num2;
    println!("Sum of {} and {} is {}", num1, num2, sum);
}