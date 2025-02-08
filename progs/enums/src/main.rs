enum Message {
    // the following are called the variants of this enum
    Start,
    Quit,
    Write(String),
    ChangeColor(i32, i32, i32),
    Move{x: i32, y: i32},
}

fn main() {
    let m: Message = Message::Start;
    print_msg(&m);
    print_msg(&Message::Quit);
    print_msg(&Message::Write(String::from("Hello")));
    print_msg(&Message::ChangeColor(45, 77, 34));
    print_msg(&Message::Move{x: 12, y: 9});

    let num: i32 = 7;
    let nstr: &str = match num {
        1 => "one",
        2 => "two",
        _ => "something else",
    };
    println!("string for {} is {}", num, nstr);

    // using if let for pattern matching
    if let Message::Start = m {
        println!("Starting now...");
    } else {
        println!("already working...");
    }

    let something: Option<i32> = Some(7);
    if let Some(j) = something {
        // on match, 7 will be assigned to j
        println!("Got number: {}", j);
    } else {
        println!("None");
    }
}

fn print_msg(msg: &Message) {
    // match is like switch-case in c/c++
    match msg {
        Message::Start => println!("Starting..."),
        Message::Quit => println!("Quitting..."),
        Message::Move{x,y} => println!("Moving to ({}, {})", x, y),
        Message::Write(s) => println!("Writing {}", s),
        Message::ChangeColor(r, g, b) => println!("Changing color to (r={},g={},b={})", r, g, b),
        //_ => println!("Err: invalid message passed!"), // default case, unreachable code here
    };
}
