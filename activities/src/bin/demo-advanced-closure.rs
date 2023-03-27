fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

fn main() {
    let name = "Todak";
    let add: Box<_> = Box::new(move |a, b| {
        println!("adding a number for {}!", name);
        a + b
    });

    println!("{}", math(2, 2, add))
}
