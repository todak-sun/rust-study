// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//



// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
struct GroceryItem {
    id: i32,
    quantity: i32,
}

// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(item: &GroceryItem) {
  println!("{:?}", item.quantity);
}

// * Create a function to display the id number, with the struct as a parameter
fn display_id(item: &GroceryItem) {
  println!("{:?}", item.id);
}

fn main() {
  let item = GroceryItem{
    id: 1,
    quantity: 10
  };

  display_id(&item);
  display_quantity(&item);
}
