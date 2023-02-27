fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn with_loop (){
    let mut a = 0;
    loop {
        if a == 5 {
            break;
        }
        println!("{:?}", a);
        a = a + 1;
    }
}

fn with_while () {
    let mut a = 0;
    while a != 5 {
        println!("{:?}", a);
        a = a + 1;
    }
}

fn main() {
    let x = add(1, 1);
    println!("{}", x);
    let y = add(3, 0);
    println!("{}", y);
    let z = add(x, 1);
    println!("{}", z);

    let life = 42;
    println!("hello");
    println!("{:?}", life); // debug 모드에만 표시하겠다는 뜻. {:? }
    println!("{:?} {:?}", life, life);

    let a = 99;
    if a > 99 {
        if a > 200 {
            println!("Huge Number");
        } else {
            println!("Big Number");
        }
    } else {
        println!("Small number");
    }

    with_loop();
    with_while();
}
