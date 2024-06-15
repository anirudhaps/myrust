fn factorial(num: u32) -> u32 {
    if num == 0 {
        1
    } else {
        num * factorial(num - 1)
    }
}

fn main() {
    println!("Factorial of {} is {}", 5, factorial(5));
    println!("Factorial of {} is {}", 3, factorial(3));
}
