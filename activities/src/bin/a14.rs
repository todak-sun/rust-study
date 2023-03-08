// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
struct Person {
    age: i32,
    name: String,
    favorite_color: String,
}

impl Person {
    fn print(&self) {
        println!("age: {:?}", self.age);
        println!("name: {:?}", self.name);
        println!("favorite_color: {:?}", self.favorite_color);
    }
}

fn main() {
    // * Create and store at least 3 people in a vector
    let people = vec![
        Person {
            age: 7,
            name: "Todak".to_owned(),
            favorite_color: "Blue".to_owned(),
        },
        Person {
            age: 12,
            name: "Todak2".to_owned(),
            favorite_color: "Red".to_owned(),
        },
        Person {
            age: 8,
            name: "Todak3".to_owned(),
            favorite_color: "Green".to_owned(),
        },
    ];

    // * Iterate through the vector using a for..in loop
    for person in &people {
        // * Use an if expression to determine which person's info should be printed
        // * The name and colors should be printed using a function
        if person.age < 10 {
            person.print()
        }
    }

    
}
