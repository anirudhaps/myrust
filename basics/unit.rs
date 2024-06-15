fn main() {
    /* () or unit type:
    A unit type is a type with exactly one valid value.
    That value is called a unit value.
    Since an expression of unit type can only ever have one value,
    the value is meaningless.
    Languages often have a predefined unit type called Unit and/or ().
    They typically name the type's one value the same. */
    let _ret: () = println!("hello");
    /*put _ before the name of a variable that is going to be unused.
     This ensures that the compiler does not print warnings for unused variables
     */
    // unit type can't be printed
    // println!("ret: {}", ret);

    // Every expression produces a value.
    // But if it doesn’t have anything useful to produce, it produces unit ().

    let x: i32 = 12;
    let y: i32 = 13;
    assert_eq!(x, y); // panics because x != y
    //Error:
    /*
    thread 'main' panicked at unit.rs:18:5:
    assertion `left == right` failed
      left: 12
      right: 13
    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    */
}