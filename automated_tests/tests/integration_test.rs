use automated_tests::add_two;


// to run specific integration test (file under tests folder), run 'cargo test --test <file_name>'
#[test]
fn it_add_two() {
    assert_eq!(add_two(2), 4);
}