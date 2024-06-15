fn main() {
    let val: i32 = {
        println!("Entered the block");
        5;
        6 // tail expression of the block
    };
    println!("value: {}", val);

    let sum: i32 = {
        let x: i32 = 5;
        let y: i32 = 11;
        x + y // tail expression of the block
    };
    println!("sum: {}", sum);
}
