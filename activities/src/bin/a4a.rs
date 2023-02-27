// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:

fn main() {

    // * Use a variable set to either true or false
    let my_bool = true;
    // * Use a match expression to determine which message to display
    match my_bool {
      true => println!("it's true"),
      false => println!("It's false"),
    }
}
