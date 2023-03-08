// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:

// * Use an enum for the box color
enum Color {
    GREEN,
}

impl Color {
    fn print(&self) {
        match self {
            Color::GREEN => println!("brown"),
        }
    }
}
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

impl ShippingBox {
    // * Implement functionality on the box struct to create a new box
    fn new(dimensions: Dimensions, weight: f64, color: Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let dimensions = Dimensions {
        width: 5.5,
        height: 5.5,
        depth: 5.5,
    };
    ShippingBox::new(dimensions, 10.2, Color::GREEN).print();
}
