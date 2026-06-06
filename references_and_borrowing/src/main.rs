// At any given time, you can have either one mutable reference or any number of immutable references.
// References must always be valid.

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    mutable_references();

    multiple_mutable_borrow_but_not_simultaneous();

    not_allowed_combining_mutable_and_immutable_references();
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the String is not dropped.

fn mutable_references() {
    let mut s = String::from("hello");

    change(&mut s);

    println!("[From mutable_references function] s = {}", { s });
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("[From change function] some_string = {some_string}");
}

fn multiple_mutable_borrow_but_not_simultaneous() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("r1 = {r1}");
    }

    let r2 = &mut s;

    println!("r2 = {r2}");
}

fn not_allowed_combining_mutable_and_immutable_references() {
    //add mut keyword to make mutable reference
    let s = String::from("hello");

    // Multiple immutable borrow are allowed.
    // let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r2}");

    //But immutable borrow and 
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{r1}, and {r3}");
}
