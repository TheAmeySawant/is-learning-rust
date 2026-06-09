fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        
        //Cannot move because we have a borrow on it
        // drop(string2); 

        //Cannot create a mutable reference as we already have immutable reference
        // let a = &mut string1;
        println!("The longest string is {result}");
    }

    // println!("{result}");
}


fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
