//! Borrowing rules says, references can be either:
//! 1. one or more inmutable
//! 2. only one mutable
//!
//! references are dereferenced automatically for borrowing, manually for mutatating the value when used.
//!
//! raw reference (pointer) can be of any number mutable as well as immutable.
//! Borrow rules are not applied on raw pointers.
//! we can create raw references (pointer) in safe and unsafe rust.
//!
//! mutable raw pointer: &raw mut var;
//! immutable raw pointer: &raw const var;
//!
//! raw refernces need to be dereferenced manually using * and can only be dereferenced in an unsafe block.
//!
//! after unsafe block, the responsibility of keeping the program memory safe, shifts from compiler to developer
//!
//! extern keyword is used to create and use Foreign Function Interface (FFI).
//! by default extern block is unsafe (because other programming languagen don't gurantee memory saftey like rust)
//! 
//! but there can be some functions in other programming language that are safe, like abs from "C language", it doesn't have anything to do with memory.
//! we can use safe keyword to indicate that function is safe to use outside the unsafe block.
//! 
//! Ex: 
//! unsafe extern "C" { //code }
//! 
//! here "C" ABI (Application Binary Interface) tells how to call the functions on assembly level.
//! 
//! "C" ABI is for C programming language.
//! 
//! safe keyword doesn't make the function safe by default
//! 
//! calling single fn from external language without extern block, just using the extern keyword
//! 
//! #[unsafe(no_mangle)]
//!pub extern "C" fn call_from_c() {//fn body}
//! 
//! Function names are by default mangled, i.e, names are slightly changed and are not human readable,
//! but the functions of external languages need to be in original name (written by programmer).
//!
//! global variables are called static variables
//! and only mutable static variables are unsafe in rust.
//! when printing the mutable static variable, we need to create raw pointer of the var and then dereference it to print it.
//! 
//! unsafe Traits,if even single method is unsafe.
//! 
//! Union
//! 
//! Miri for unsafe code, check at runtime (requires nightly version of rust),
//! cargo +nightly miri run
//! 

use std::slice;

static mut COUNTER: u8 = 0;
static HELLO : &str = "Hello";



unsafe fn dangerous() {
    unsafe {
        COUNTER += 1;
    }
}

fn main() {
    let mut num = 5;

    let r1 = &raw const num;
    let r2 = &raw mut num;
    let r3 = &mut num;

    *r3 = 7;

    unsafe {
        println!("r1 is: {:?}", *r1);
        println!("r2 is: {:?}", *r2);

        dangerous();

        println!("COUNTER: {}", *(&raw const COUNTER));

        let a = COUNTER + 5;

        println!("a: {a}");
    }

    println!("safe r3: {r3}");

    let mut v = [1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    

    let address = 0x01234usize;
    let r = address as *mut i32;

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10) };

    println!("{HELLO}");

    println!("r2 is: {:?}", values);

}
