// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer

struct Customer {
    age: i32,
}

// * Use a function to determine if a customer can make a restricted purchase
fn purchase(customer: &Customer) -> Result<(), String> {
    if customer.age >= 21 {
        Ok(())
    } else {
        Err("customer is young".to_owned())
    }
}
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

fn main() {
    let customers = vec![Customer { age: 10 }, { Customer { age: 22 } }];
    
    for customer in customers {
      let success = purchase(&customer);
      println!("{:?}", success)
    }
}
