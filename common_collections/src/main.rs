use std::{
    collections::{BTreeMap, HashMap},
    io,
};

use crate::employee_dept_management::get_dept;
mod employee_dept_management;
mod pig_latin;

fn main() {
    let v = vec![1, 2, 3, 3, 5, 4, 7, 2];

    let lenght = v.len();

    if lenght % 2 == 1 {
        //Odd lenght
        println!("median: {}", ((lenght / 2) + 1));
    } else {
        //Even lenght
        println!("median: {}", (lenght / 2));
    }
    // println!("{}", (lenght % 2 ));

    let mut h = HashMap::new();

    for i in v {
        let count = h.entry(i).or_insert(0);
        *count += 1;
    }

    println!("{h:#?}");

    let normal_latin = String::from("Can you help me? apple");
    println!("{}", pig_latin::normal_to_pig_latin(&normal_latin));

    // Dept , Emp
    let mut emp_dept_manager: BTreeMap<String, Vec<String>> = BTreeMap::new();

    // let v = vec![String::from("Amey"), String::from("Bhaskar"), String::from("Sawant")];

    // emp_dept_manager.insert(String::from("Engineering"), v);

    let mut stop = true;

    println!(
        "
-----Welcome to Employee & Department Management CLI!------"
    );

    while stop {
        println!(
            "
----------------------OPTIONS----------------------
1. Add <Employee> to <Department>
2. Get <Department>
3. Fetch (Displays Whole Company's Department and Employee)
4. Exit"
        );
        let mut input = "".to_string();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to take input");

    
        let tokens: Vec<&str> = input.split_whitespace().collect();

        let first = *(tokens.first().unwrap_or(&""));

        match first {
            "Add" | "add" => {
                if employee_dept_management::add_emp(&mut emp_dept_manager, &tokens) {
                    println!(
                        "\n\n Added {} employee in {} Department Successfully!",
                        &tokens[1], &tokens[3]
                    );
                } else {
                    println!(
                        "\n<Department or <Employee> missing. Please follow the format of input given in the Options"
                    );
                }
            }
            "Get" | "get" => {
                if !get_dept(&emp_dept_manager, &tokens) {
                    println!("Invalid/Incomplete Get <Department> Command",);
                }
            }
            "Fetch" | "fetch" => println!("{emp_dept_manager:#?}"),
            "Exit" | "exit" => stop = false,
            _ => println!("\nInvalid Command"),
        }
    }
}
