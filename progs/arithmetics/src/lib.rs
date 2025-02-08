/*
all the functions are private in the project by default. So, we have to make
them public using `pub` keyword even if we intend to use them in the same project.
*/
pub fn sum(x: i32, y: i32) -> i32 {
    x + y
}

pub fn diff(x: i32, y: i32) -> i32 {
    x - y
}

pub fn mul(x: i32, y: i32) -> i32 {
    x * y
}

pub fn divide(x: i32, y: i32) -> i32 {
    x / y
}

/*
unsigned integer types: u8, u16, u32, u64, u128, usize.
usize is the size of the platform's pointer type. It is also used as type of a
variable that is used to index array or vectors.
Signed integer types: i8, i16, i32, i64, i128, isize.
The default type of an integer literal is i32 because it is generally the
fastest integer even on the 64 bit architecture.

Integer literals can be specified in a number of ways:
Decimal: 1000000 or 1_000_000 (_ are ignored)
Hex: 0xdeadbeef or 0xdead_beef (_ is ignored)
Octal: 0o77543211 or 0o 77_54_32_11 (_ are ignored)
Binary: 0b11110011 or 0b1_1_1_1_0_0_1_1 (_ are ignored)
Byte (u8 only): b'A'  (within '', there will be an ascii character)
Byte and u8 are interchangeably used for a byte.

Floating point types: f32, f64.
f64 is the default type for the floating point numbers/literals.
 */

pub fn compound_types() {
    // tuple
    let info = (1, 3.3, 999);
    println!(
        "first val: {}, second val: {}, third val: {}",
        info.0, info.1, info.2
    );
    let student: (&str, u16, u32) = ("Anirudha", 34, 1991);
    let (name, age, year_of_birth) = student;
    println!(
        "Student: name: {}, age: {}, year of birth: {}",
        name, age, year_of_birth
    );

    // array: stores multiple values of same type
    let buf = [1, 2, 3];
    let buf2 = [0; 3]; // [value; how many]
    let buf3: [u8; 3] = [4, 5, 6];
    /*
    [type; array size]
    in case of arrays, we specify type and size even if the initializer list is provided.
    max array size: 32
    arrays are of fixed size. These are usually stored in stack.
    */
    let mut i = 0;
    while i < 3 {
        println!("buf[{}] = {}", i, buf[i]);
        i += 1;
    }
    println!();
    i = 0;
    while i < 3 {
        println!("buf2[{}] = {}", i, buf2[i]);
        i += 1;
    }
    println!();
    i = 0;
    while i < 3 {
        println!("buf3[{}] = {}", i, buf3[i]);
        i += 1;
    }
}
