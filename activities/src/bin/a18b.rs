// Topic: Result & the question mark operator
//
// Requirements:
// * Determine if an employee can access a building using a digital keycard
// * Employees that can access the building are:
//   * Maintenance crews
//   * Marketing department employees
//   * Managers
// * Other employees that work at the company are:
//   * Line supervisors
//   * Kitchen staff
//   * Assembly technicians
// * Ensure that terminated employees cannot access the building
//   regardless of their position
//
// Notes:
// * Use an enum to represent all types of employees

#[derive(Clone)]
enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisor,
    KitchenStaff,
    AssemblyTech,
}

enum Status {
    Active,
    Terminated,
}

// * Use a struct to store the employee type and whether they are
//   still employed
struct Employee {
    position: Position,
    status: Status,
}
// * Use a function that returns a Result to determine if the employee
//   may enter the building

fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("terminated".to_owned()),
        _ => (),
    }

    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("invalid position".to_owned()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    let attempt_access = try_access(employee)?;
    println!("access ok");
    return Ok(());
}

fn main() {
    for position in vec![
        Position::Manager,
        Position::Maintenance,
        Position::Marketing,
        Position::AssemblyTech,
        Position::LineSupervisor,
        Position::KitchenStaff,
    ] {
        for status in vec![Status::Active, Status::Terminated] {
            let manager = Employee { position: position.clone(), status };
            match print_access(&manager) {
                Err(e) => println!("access denied: {:?}", e),
                _ => println!("access ok"),
            };
        }
    }
}

// * Print whether the employee may access the building
//   * Must use a function that utilizes the question mark operator to do this
