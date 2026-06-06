use std::collections::HashMap;

fn main(){
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{scores:#?}");

    let team_name = String::from("Blue");

    // Normal way of extracting value from Option enum
    let score = scores.get(&team_name);
    println!("Normal way:");
    match score{
        Some(i) => println!("{team_name} Team Score: {i}"),
        None => println!("{team_name} Team's Score NOT FOUND!")
    }

    //Mentos way of extracting value from Option enum
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("\n\nMentos way:");
    println!("{team_name} Team Score: {score}");

    println!("Iterating Key value pair: ");

      for (key, value) in &scores {
        println!("{key}: {value}");
    }
    println!("\n");

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 1);

    let Yellow_score = scores.entry(String::from("Yellow")).or_insert(50);
    // let Blue_score = scores.entry(String::from("Blue")).or_insert(50);

    *Yellow_score += 98;
    println!("{Yellow_score}");
    println!("{scores:?}");


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("\n\n\n{map:?}\n\n");

}