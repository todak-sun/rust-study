// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:

fn main() {
    // * Use a mutable integer variable
    let mut counter = 1;

    // * Use a loop statement
    loop {
        // * Print the variable within the loop statement
        println!("{:?}", counter);
        counter = counter + 1;
        if counter > 4 {
            // * Use break to exit the loop
            break;
        }
    }
}
