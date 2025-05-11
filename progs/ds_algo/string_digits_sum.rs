fn digits_sum(s: &str) -> Result<i32, &str> {
    if s.is_empty() {
        return Err("empty string");
    }
    let mut num: i32;
    let mut sum: i32 = 0;
    let mut digit: i32;
    let res = s.parse::<i32>();
    if res.is_err() {
        return Err("invalid string: not a number");
    }

    num = res.ok().unwrap();
    while num != 0 {
        digit = num % 10;
        num /= 10;
        sum += digit;
    }
    return Ok(sum);
}

fn main() {
    // Test cases:
    let mut res = digits_sum("234");
    assert_eq!(res.is_ok(), true); // no error
    assert_eq!(res.ok().unwrap(), 9); // value of sum
    println!("T1: success");

    res = digits_sum("");
    assert_eq!(res.is_err(), true); // should be error case
    assert_eq!(res.err().unwrap(), "empty string"); // error value
    println!("T2: success");

    res = digits_sum("12t");
    assert_eq!(res.is_err(), true); // should be error case
    assert_eq!(res.err().unwrap(), "invalid string: not a number"); // error value
    println!("T3: success");

    res = digits_sum("5");
    assert_eq!(res.is_ok(), true); // no error
    assert_eq!(res.ok().unwrap(), 5); // error value
    println!("T4: success");
}
