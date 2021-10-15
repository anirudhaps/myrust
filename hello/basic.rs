fn main() {
  let x = 5 + 90 + 5;
  println!("x = {}", x); // {} will be replaced with the argument and it will be stringified

  // positional arguments
  println!("All {0} are {1} but all {1} are not {0}", "teachers" /* {0} */, "persons" /* {1} */);

  // named arguments
  println!("{subject} {verb} {object}", object="the lazy dog", subject="the quick brown fox", verb="jumps over");

  // special formatting using a ':', :b prints binary representation
  println!("{} of {:b} people know binary, the other half does not", 1, 2);
}