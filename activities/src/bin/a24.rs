// Topic: Iterator
//
// Requirements:
// * Triple the value of each item in a vector.
// * Filter the data to only include values > 10.
// * Print out each element using a for loop.
//
// Notes:
// * Use an iterator chain to accomplish the task.

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    let filtered: Vec<i32> = data.iter().map(|i| i * 3).filter(|i| i > &10).collect();

    for item in filtered {
        println!("{:?}", item)
    }
}
