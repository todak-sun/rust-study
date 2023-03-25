// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Figure {
    fn calc_perimeter(&self) -> i32;
}

struct Square {
    side: i32,
}

impl Figure for Square {
    fn calc_perimeter(&self) -> i32 {
        self.side * 4
    }
}

struct Triangle {
    site_a: i32,
    site_b: i32,
    site_c: i32,
}

impl Figure for Triangle {
    fn calc_perimeter(&self) -> i32 {
        self.site_a + self.site_b + self.site_c
    }
}

fn print_perimeter(figure: impl Figure) {
    println!("perimeter : {:?}", figure.calc_perimeter())
}

fn main() {
    print_perimeter(Square { side: 30 });
    print_perimeter(Triangle {
        site_a: 10,
        site_b: 20,
        site_c: 18,
    });
}
