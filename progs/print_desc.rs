fn print() {
    let mut num: i32 = 5;
    while num > 0 {
        println!("{} {} of beer on the wall,", num, if num > 1 {
            "bottles"
        } else {
            "bottle"
        });
        println!("{} {} of beer.", num, if num > 1 {
            "bottles"
        } else {
            "bottle"
        });
        println!("You take one down, pass it around.");
        num -= 1;
        if num > 0 {
            println!("{} {} of beer on the wall.", num, if num > 1 {
                "bottles"
            } else {
                "bottle"
            });
        } else {
            // num == 0
            println!("No bottles of beer on the wall.");
        }
        println!();
    }
}

fn main() {
    print()
}