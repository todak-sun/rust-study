// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:

// * Use an enum with color names as variants
enum Colors {
    RED,
    GREEN,
    BLUE,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn print_color_name(color: Colors) {
    // * Use a match expression to determine which color
    //   name to print
    let color_name = match color {
        Colors::RED => "red",
        Colors::GREEN => "green",
        Colors::BLUE => "blue",
    };

    println!("{:?}", color_name);
}

fn main() {
    print_color_name(Colors::RED);
    print_color_name(Colors::GREEN);
    print_color_name(Colors::BLUE);
}
