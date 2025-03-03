use std::io;

fn main() {
    let mut inp = String::new();
    println!("Enter a number: ");
    let num1: i32;
    let num2: i32;
    let sum: i32;
    io::stdin().read_line(&mut inp).expect("Failed to read line");
    num1 = inp.trim().parse().expect("Please enter a number");
    inp.clear();
    println!("Enter another number: ");
    io::stdin().read_line(&mut inp).expect("Failed to read line");
    num2 = inp.trim().parse().expect("Please enter a number");
    sum = num1 + num2;
    println!("Sum of {} and {} is {}", num1, num2, sum);
}