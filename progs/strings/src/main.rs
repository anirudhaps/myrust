fn main() {
    // &str is the string slice or the borrowed string
    let name: &str = "Anirudha";
    println!("Hi this is {}", name);

    // name.chars(): to get the iterator over the chars of the name string
    // slice
    println!(
        "printing char by char.. no. of chars in {}: {}",
        name,
        name.chars().count() /* number of chars in the string */
    );
    let mut i = 0;
    while i < name.chars().count() {
        // get to the ith position using nth(i)
        // unwrap will return the char
        print!("{} ", name.chars().nth(i).unwrap());
        i += 1
    }
    println!();

    // another way to print the string char by char
    for ch in name.chars() {
        print!("{ch} ");
    }
    println!();

    // printing the Some type
    println!("{:?}", name.chars().nth(1));

    let s: &str = "";
    if s.is_empty() {
        println!("it is empty");
    }

    if !name.is_empty() {
        println!("{name} is not empty");
    }

    // String is that can be modified
    let hello = String::from("Hello, World!");
    println!("{hello}");
    // hello string object was the owner of the string "Hello, World!"
    let hello_new = hello;
    // "Hello, World!" is not copied/cloned. It (ownership) is moved to the hello_new.
    // println!("{hello}"); // hello is now uninitialized. This will give error
    println!("{hello_new}");
    print_string(hello_new); // value moved again to s (parameter of the function)
    // println!("{hello_new}");

    let some = String::from("some");
    //let some2 = some; // moved
    if some.is_empty() {
        println!("some is empty after move");
    }
    let some2 = some.clone(); // string is cloned
    println!("some: {}, some2: {}", some, some2);

    /*
     In rust, the string (some) will have the following:
     In stack: structure some{ptr, len, capacity}.
     In heap, "some" and ptr pointing to "some" in heap.

     In rust, copying means copying the stucture some{ptr, len, capacity}.
     cloning means copying both i.e. structure some in stack and the heap's
     "some" string.
     */

    // passing string by reference
    show_string(&some2); // some2 is not moved. Just a reference to it is passed
    println!("some2: {}", some2);

    let mut item = String::from("wood");
    modify_string(&mut item); // pass a mutable reference to item
    println!("final String: {}", item);

    // passing immutable reference to a mutable object
    show_string(&item);
}

fn print_string(s: String) {
    println!("Received: {}", s);
}

// s is an immutable reference
fn show_string(s: &String) {
    println!("Received ref: {}", s);
    // s.replace_range(1..2, "a");
    // above will give compile time error because s is an immutable reference
    // to some2. Thus, can't modify some2 using s
}

// s is a mutable reference
fn modify_string(s: &mut String) {
    println!("String before modification: {}", s);
    s.replace_range(3..4, "l");
    println!("String after modification: {}", s);
}
