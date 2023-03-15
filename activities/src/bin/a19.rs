// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert("Chairs", 5);
  map.insert("Beds", 3);
  map.insert("Tables", 2);
  map.insert("Couches", 0);

  let mut total_count: i32 = 0;
  for (furniture, count) in map.iter() {
    total_count = total_count + count;
    let count_message = match count {
      0 => "out of stock".to_owned(),
      _ => format!("{:?}", count)
    };

    println!("{:?} - {:?}", furniture, count_message);
  }
  println!("total count : {:?}", total_count);
}
