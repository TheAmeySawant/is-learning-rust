//! Refutable: match may fail for a possible value
//! Irrefutable: match any possible value
//!
//! Refutable & Irrefutable:
//! - if let
//! - while let
//! - let...else
//!
//! Irrefutable:
//! - fn (parameter)
//! - let statement
//! - for loop
//!
//! In match expression, we can have multiple Refutable patterns but the last one has to Irrefutable pattern
//!
//! _ and .. can be used as placeholder or widlcards.
//! _ is irrefutable but doesn't bind value to it
//!
//! Patterns can be used to destruct Tuples, Structs, etc.
//!
//! Destruct with match expression is just supeerrrrr
//!
//! 

struct Point {
    x: i32,
    y: i32,
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum MessageId {
    Hello{
        id: i32
    }
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) | Some(60) => println!("Got 50 or 60"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    match y {
        1..=10 => println!("1 to 10, including 1 & 10, nobody is excluded"),
        _ => println!("any Number"),
    }

    let z = 'a';

    match z {
        'a'..='e' => println!("a to e"),
        _ => println!("any char"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };

    match origin {
        Point3D { x, .. } => println!("x is {x}"),
    }

    
    let msg = MessageId::Hello { id: 5 };

    match msg {
        MessageId::Hello { id: id @ 3..=7 } => {
            println!("Found an id in range: {id}")
        }
        MessageId::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        MessageId::Hello { id } => println!("Found some other id: {id}"),
    }
}
