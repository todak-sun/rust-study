pub mod group;
pub mod helper;

pub fn print_from_lib() {
    println!("hello from lib");
    helper::print_from_helper();

    group::g1::hello_from_g1();
}
