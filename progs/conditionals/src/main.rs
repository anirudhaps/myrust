fn main() {
    println!("loop1:");
    loop1();
    println!("loop2:");
    loop2();
    println!("\nloop3:");
    loop3();
    println!("loop4:");
    loop4();
    println!("loop5:");
    loop5();
    println!("loop6:");
    loop6();
}

fn loop1() {
    // while loop
    let arr: [i32; 5] = [2, 4, 6, 8, 10];
    let mut i = 0;

    while i < 5 {
        println!("arr[{}] = {}", i, arr[i]);
        i += 1;
    }
}

fn loop2() {
    // unconditional loop
    let mut i = 0;
    'one: loop {
        if i == 5 {
            break;
        }
        let mut j = 0;
        loop {
            if j == 5 {
                break 'one;
            }
            print!("*");
            j += 1;
        }
        println!();
        i += 1;
    }
}

fn loop3() {
    // for loop
    for num in [1, 2, 3].iter() {
        print!("{} ", num);
    }
    println!();
}

fn loop4() {
    // for loop
    for num in 0..10 {
        print!("{} ", num);
    }
    println!();
}

fn loop5() {
    // for loop
    for num in 0..=10 {
        print!("{} ", num);
    }
    println!();
}

fn loop6() {
    // for loop
    let arr = [(1, 31), (2, 45)];
    for (roll, age) in arr.iter() {
        println!("{} roll number has {} age ", roll, age);
    }
}
