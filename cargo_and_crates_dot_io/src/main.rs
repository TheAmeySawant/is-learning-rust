//! We learned doc commenting outer(///) and inner(//!).
//! And for ease of use of the crate we use 'pub use' to re-export the items we want to make availabe for users in our public API
//! Cargo Workspace for crates management (Single Cargo.lock controlling dependencies)
//! Installing crates from crates.io using 'cart install <crate-name>'
//! 


use cargo_and_crates_dot_io::add_one;
use cargo_and_crates_dot_io::PrimaryColor;
use cargo_and_crates_dot_io::mix;




fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    
    println!("{:?}", mix(red, yellow));
    add_one(5);
}
