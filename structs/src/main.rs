//Normal Struct
struct User {
    active: bool,
    username: String,
    email: String,
    id: u64,
}

//Tuple Struct
struct Color(i32, i32, i32);

fn main() {
    let user1: User = User {
        active: true,
        username: String::from("Amey"),
        email: String::from("amey01962@gmail.com"),
        id: 44,
    };

    println!(
        "{}\n{}\n{}\n{}",
        user1.active, user1.username, user1.email, user1.id
    );

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("another@example.com"),
    //     id: user1.id,
    // };

    let user2: User = User {
        email: String::from("amey44@gmail.com"),
        //move occurs because of String, therefore the String of struct User1 becomes invalid, it's no longer owner of String, User2 is the owner of String.
        ..user1
    };
    
    //We can no longer use User1's moved heap allocated varFiable(ex. String)

    // print!("{}\n{}", user1.active, user1.username);

    println!(
        "{}\n{}\n{}\n{}",
        user2.active, user2.username, user2.email, user2.id
    );

    //Normal Tuple Zindagi
    let a = (0, 0, 0);

    let (x1, y1, z1) = a;

    println!("x1: {x1}\ty1: {y1}\tz1: {z1}");

    //Mentos Tuple Struct Zindagi

    let black = Color(0, 0, 0);

    let Color(x2, y2, z2) = black;

    println!("Black[0]: {}", black.0);

    println!("x2: {x2}\ty2: {y2}\tz2: {z2}");
}
