fn main() {
    println!("Hello, world!");
    {
        let name = "Michael";
        println!("Nice to meet you, {}!", name);
    }
    println!("Have a great day");
}
// run:
// rustfmt format.rs
// for formatting the rust source file as per rust convention