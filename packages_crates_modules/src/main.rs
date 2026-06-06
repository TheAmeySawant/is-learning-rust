// mod lib;
use std::fmt::Result;
use std::io::Result as io_result;
use packages_crates_modules::calling_lib;

fn main() {
    calling_lib();
    packages_crates_modules::hosting::add_to_waitlist();
}
