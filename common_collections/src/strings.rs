fn main() {
    let s = String::from("foo");
    let s1 = String::from("bar and cake");

    //needs s to be mutable
    // s.push_str(&s1);

    // s is moved here
    // + works like 'String + &str' (any number of &strc can be added)
    //Also &String are implicitly converted to &str like &s1 becomes &s1[..]

    let _s3 = &s1;
    let _s4 = &s1[1..2];
    let s2 = s + &s1 + " CocoMelon";

    println!("{_s4}");


    let s = String::from("foo");
    // format! macro is much better, nobody's onwership is taken.
    let s5 = format!("- {s} - {s2} -");

    println!("s5: {s5}");

    //"Amey" is str, s is mut &str i.e, s can me modified to point to another str (string literal "")

    // let mut s = "Amey";

    println!("s2: {s2}");
}
