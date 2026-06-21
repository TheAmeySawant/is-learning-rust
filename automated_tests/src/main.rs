// use automated_tests::*;

fn _prints_and_returns_10 (a: i64) -> i64 {
    //std output are printed only if the test fails.
    //Run cargo 'test -- --show-output' to get std output printed of successful test also. 
    println!("I got the Value : {a}"); 
    10
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;


    //To run a particular test use 'cargo test <fn name>'
    //<fn name> works by searching all the fn names matching with <fn name>.
    //for 'this_test_passes' only this_test_passes runs
    //for 'test' both fn containing (not just starting) test word in their fn name are runned.
    #[test]
    fn this_test_passes() {
        assert_eq!(_prints_and_returns_10(5), 10);
    }

    //cargo test -- --ignored runs only ignored tests (functions).
    #[ignore = "for a reason"]
    #[test]
    fn this_test_fails() {
        assert_eq!(_prints_and_returns_10(8), 8);
    }
}
