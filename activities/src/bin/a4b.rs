// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:

fn main() {
    // * Use a variable set to any integer
    let any_int = 3;

    match any_int {
        // * Use a match expression to determine which message to display
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        // * Use an underscore (_) to match on any value
        _ => println!("other"),
    }
}
