fn main() {
    // println!("Hello, world!");
    let s = String::from("Hellow");
    let a = first_word(&s);


    println!("{a}");
   
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    // for i in bytes.iter() {
    //     println!("{i}");
    // }

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s
}

