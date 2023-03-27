#[derive(Debug)]
enum Answer {
    Yes,
    No,
}
fn main() {
    let yes = Answer::Yes;
    let yes_heap = Box::new(yes);
    let yes_stack = *yes_heap;
    println!("{:?}", yes_stack)
}
