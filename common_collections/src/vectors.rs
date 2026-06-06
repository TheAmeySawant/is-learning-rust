 #[derive(Debug)]
 enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    
fn main() {

    let mut ab = "Amey";
    // let mut v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    
    let first = &v[0];
    
    println!("{first}");

    //push method of Vector creates a mutable reference,therefore don't use any reference decalred before using push method.
    //make sure to redeclare the reference(s) if you wanna use them after push as the scope of mutable referece created by psuh method ends right at the call.
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    println!("v: {v:?}");
    let first = &v[0];
    
    println!("{first}");

    let third = &v[2]; //panic stops the execution when out of index
    println!("{}", third);

    let third = v.get(9); //returns None when out of index

    match third{
        Some(third) => println!("Third: {third}"),
        None => println!("third doesn't exists")
    }

    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    let mut row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let mut row1 = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let col = vec![&mut row,&mut row1];
    
    println!("{row:?}");
}
