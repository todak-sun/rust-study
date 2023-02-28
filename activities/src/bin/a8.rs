// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavors {
    Orange,
    Grape,
}

struct Drink {
    flavor: Flavors,
    oz: f64,
}

fn print_drink(drink: Drink) {
    let flavor = match drink.flavor {
        Flavors::Grape => "Grape",
        Flavors::Orange => "Orange",
    };

    println!("Drink[flavor={:?}, oz={:?}]", flavor, drink.oz);
}

fn main() {
  let grape_drink = Drink {
    flavor: Flavors::Grape,
    oz: 12.24,
  };
  print_drink(grape_drink);

  let orange_drink = Drink {
    flavor: Flavors::Orange,
    oz: 9.2,
  };
  print_drink(orange_drink);
}
