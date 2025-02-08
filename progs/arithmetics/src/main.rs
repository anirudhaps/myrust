// import the sum function
use arithmetics::sum;

/*
constants: here, the constants are assigned in the module scope. We must
explicitly provide the type for every constant (eg. i32). Also, the value of
the constant must be a constant expression (literal or const expr function)
*/
const VAL_ONE: i32 = 10;
const VAL_TWO: i32 = 5;

fn main() {
    /*
    variables are by default read-only. To update the value of a variable,
    it must be mutable (mut).
    */
    let a = VAL_ONE;
    let mut b = VAL_TWO;

    let s = sum(a, b);
    /* invoke diff and other functions using full scoped path */
    let d = arithmetics::diff(a, b);
    let m = arithmetics::mul(a, b);
    let div = arithmetics::divide(a, b);

    println!(
        "a: {}, b: {}, sum: {}, difference: {}, product: {}, division: {}",
        a, b, s, d, m, div
    );
    // updating the value of b
    b = s + d + m + div;
    println!("final b: {}", b);

    // another way of specifying the literals along with their types
    let x = 5_u16;
    // or let x = 5u16;
    let y = 3.14_f32;
    // or let y = 3.14f32;
    println!("x: {}, y: {}", x, y);

    // boolean type literals: true and false
    let cond = true;
    let other_cond: bool = false;
    if cond {
        println!("its true");
    }
    if other_cond {
        println!("its false"); // never printed
    }

    let true_val: u8 = cond as u8;
    println!("true type casted as u8: {}", true_val);
    let false_val: u8 = other_cond as u8;
    println!("false type casted as u8: {}", false_val);

    // character type: char (4 byte in length)
    let yes: char = 'y';
    let no = 'n';
    println!("yes: {}, no: {}", yes, no);

    arithmetics::compound_types();
}
