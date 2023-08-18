use std::io;

struct Employee {
    employee_name: String,
    employee_id: i32,
    email: String,
    age: i32,
    phone_number: String,
}

impl Employee {
    fn new() -> Employee {
        Employee {
            employee_name: String::new(),
            employee_id: 0,
            email: String::new(),
            age: 0,
            phone_number: String::new(),
        }
    }
}

fn get_employee_by_id(employees: &Vec<Employee>, employee_id: i32) -> Option<&Employee> {
    employees.iter().find(|emp| emp.employee_id == employee_id)
}

fn get_employees_by_age(employees: &Vec<Employee>, age: i32) -> Vec<&Employee> {
    employees.iter().filter(|emp| emp.age == age).collect()
}

fn main() {
    let mut employees: Vec<Employee> = Vec::new();

    println!("Enter the number of employees: ");
    let mut num_employees = String::new();
    io::stdin().read_line(&mut num_employees).expect("Failed to read line");
    let num_employees: i32 = num_employees.trim().parse().expect("Please enter a valid number");

    for _ in 0..num_employees {
        let mut emp = Employee::new();

        println!("Enter details for employee:");
        println!("Name: ");
        io::stdin().read_line(&mut emp.employee_name).expect("Failed to read line");
        emp.employee_name = emp.employee_name.trim().to_string();

        println!("Employee ID: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        emp.employee_id = input.trim().parse().expect("Please enter a valid number");

        println!("Email: ");
        io::stdin().read_line(&mut emp.email).expect("Failed to read line");
        emp.email = emp.email.trim().to_string();

        println!("Age: ");
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        emp.age = input.trim().parse().expect("Please enter a valid number");

        println!("Phone Number: ");
        io::stdin().read_line(&mut emp.phone_number).expect("Failed to read line");
        emp.phone_number = emp.phone_number.trim().to_string();

        employees.push(emp);
    }

    println!("Enter an Employee ID to get employee details: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let emp_id: i32 = input.trim().parse().expect("Please enter a valid number");

    if let Some(emp) = get_employee_by_id(&employees, emp_id) {
        println!("Employee Details:");
        println!("Name: {}", emp.employee_name);
        println!("Employee ID: {}", emp.employee_id);
        println!("Email: {}", emp.email);
        println!("Age: {}", emp.age);
        println!("Phone Number: {}", emp.phone_number);
    } else {
        println!("Employee not found.");
    }

    println!("Enter an Age to get employees with that age: ");
    input.clear();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let search_age: i32 = input.trim().parse().expect("Please enter a valid number");

    let employees_by_age = get_employees_by_age(&employees, search_age);
    if !employees_by_age.is_empty() {
        println!("Employees with Age {}:", search_age);
        for emp in employees_by_age {
            println!("Name: {}, Employee ID: {}", emp.employee_name, emp.employee_id);
        }
    } else {
        println!("No employees found with Age {}.", search_age);
    }
}
