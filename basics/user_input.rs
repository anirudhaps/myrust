use std::io;

fn main() {
    let mut name = String::new();
    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("Hello, {}", name.trim());

    let age: u32;
    let mut inp = String::new();
    println!("Enter your age: ");
    io::stdin().read_line(&mut inp).expect("Failed to read line");
    // converting the string to a number
    age = inp.trim().parse().expect("Please enter a number");
    println!("You are {} years old", age);
}