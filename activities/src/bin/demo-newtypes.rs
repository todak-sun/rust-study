#[derive(Debug, Clone, Copy)]

struct NeverZero(i32);

impl NeverZero {
    pub fn new(i: i32) -> Result<Self, String> {
        if i == 0 {
            Err("connote be zero".to_owned())
        } else {
            Ok(Self(i))
        }
    }
}

fn divide(a: i32, b: NeverZero) -> i32 {
    let b = b.0;
    return a / b;
}

fn main() {
    match NeverZero::new(5) {
        Ok(nz) => println!("{:?}", divide(10, nz)),
        Err(e) => println!("{:?}", e),
    }
}
