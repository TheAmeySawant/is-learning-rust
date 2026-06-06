fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    // The concept of copying the pointer, length, and capacity without copying the data probably sounds like making a shallow copy. But because Rust also invalidates the first variable, instead of being called a shallow copy, it’s known as a move. Move is done to avoid 'double free' error, which occurs when drop (cleaning up heap memory) function is called implicitly on two different variables (pointers) pointing on the same heap memory address, leading to memory corruption.

    println!("{s2}, world!");

    //clone() method creates a deep copy of s2 i.e., new stack of s3 with pointer, lenght and capacity, where the pointer is pointing to a separate copy of data of s2 in heap memeory. Key difference is that here heap has 2 saparate same String for s2 and s3. Keeping s2 in scope.
    let s3 = s2.clone();

    println!("{s3}, world!");

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here

    // println!("{s} in main"); //This will cause error saying s is invalid

    let x = 5; // x comes into scope

    makes_copy(x); // Because i32 implements the Copy trait,
    // x does NOT move into the function,
    // so it's okay to use x afterward.
    println!("{x} in main");

    let a1 = gives_ownership(); // gives_ownership moves its return
    // value into a1

    let a2 = String::from("helloA2"); // a2 comes into scope

    let a3 = takes_and_gives_back(a2); // a2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into a3

    println!("a1: {a1} \na3: {a3}");

    return_ownership_back_to_variable_after_function_call();
}

// Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns a String.
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}


fn return_ownership_back_to_variable_after_function_call() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}