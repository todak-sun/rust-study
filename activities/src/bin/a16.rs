// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Student {
    name: String,
    locker: Option<i32>,
}

fn main() {
    let students = vec![
        Student {
            name: "Todak".to_owned(),
            locker: Some(1324),
        },
        Student {
            name: "Hong".to_owned(),
            locker: None,
        },
        Student {
            name: "Kim".to_owned(),
            locker: Some(1324),
        },
    ];

    for student in students {
      match student.locker {
          Some(assignment) => println!("Student [{:?}] has assignment. - {:?}", student.name, assignment),
          None => println!("Student [{:?}] dosen't have assignment", student.name)
      }
    }
}
