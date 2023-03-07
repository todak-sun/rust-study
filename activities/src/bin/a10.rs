// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:

// * Use a function to print the messages
fn print_message(result: bool) {
  // * Use a match expression to determine which message
  //   to print
  match result {
    true => println!("It's big"),
    false => println!("It's small")
  }
}

// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
fn main() {
  let num: i32 = 50;
  let result: bool = num > 100;

  println!("{:?}", result);
  print_message(result)
}
