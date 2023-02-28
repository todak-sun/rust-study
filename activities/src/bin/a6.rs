// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:

fn main() {
    // * Use a mutable integer variable
    let mut counter = 5;
    
    // * Use a while statement
    while counter > 0 {
        // * Print the variable within the while loop
        // * Do not use break to exit the loop
        println!("{:?}", counter);
        counter = counter - 1;
    }

    println!("done!")
}
