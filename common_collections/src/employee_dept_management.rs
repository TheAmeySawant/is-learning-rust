use std::{collections::BTreeMap};

enum ValidateForWhat{
    AddEmp,
    GetDept
}

pub fn add_emp(
    emp_dept_manager: &mut BTreeMap<String, Vec<String>>,
    input: &[&str],
) -> bool {

    if !validate_input(input, ValidateForWhat::AddEmp){
        return false;
    }
    
    // println!("input : {input:?}");
    let dept = input[3].to_string();
    let emp = input[1].to_string();
    // println!("\ndept : {dept} \t emp : {emp}");

    let lower_emp = emp.to_lowercase();

    let dept_vec = emp_dept_manager
    .entry(dept)
    .or_default();

    let idx = dept_vec.partition_point(|s| s.to_lowercase() < lower_emp);

    dept_vec.insert(idx, emp);

    true
}

pub fn get_dept(
    emp_dept_manager: &BTreeMap<String, Vec<String>>,
    input: &[&str],
) -> bool {

    if !validate_input(input, ValidateForWhat::GetDept){
        return false;
    }

    let dept = input[1].to_string();

    if let Some(dept_vec) = emp_dept_manager.get(&dept) {
        println!("\n{dept} : {dept_vec:#?}");
        return true;
    }

    false
}


fn validate_input(input: &[&str], vfw: ValidateForWhat) -> bool{
    match vfw{
        ValidateForWhat::AddEmp => {
            if input.len() != 4 || input[2].to_lowercase() != "to"{
                return false;
            }
        },
        ValidateForWhat::GetDept => {
            if input.len() != 2 {
                return false;
            } 
        }
    }

    true
}