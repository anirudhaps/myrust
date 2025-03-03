use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    // if & is not used, args will be moved and cannot be used again
    for a in &args {
        println!("{}", a);
    }
    println!("Args length: {}", args.len());
    let prog_name = &args[0];
    let first_arg = &args[1];
    let sec_arg = &args[2];
    let num_arg = &args[3];
    // convert string to number
    let num: u32 = num_arg.parse().unwrap();
    println!("Program name: {}", prog_name);
    println!("First argument: {}", first_arg);
    println!("Second argument: {}", sec_arg);
    println!("Parsed number: {}", num);
}