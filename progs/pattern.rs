fn print_top_bottom() {
    let mut i: i32 = 0;
    while i < 5 {
        print!("*");
        i += 1;
    }
    println!();
}

fn print_middle(rows: i32) {
    let mut i: i32 = 0;
    while i < rows {
        println!("*   *");
        i += 1;
    }
}

fn main() {
    print_top_bottom();
    print_middle(8);
    print_top_bottom();
}