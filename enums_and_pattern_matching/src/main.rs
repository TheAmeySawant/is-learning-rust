#[derive(Debug)]
enum Message {
    _Quit,
    _Move { x: i32, y: i32 },
    Write(String),
    _ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Message called!");
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    _Alabama,
    Alaska,
    // --snip--
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::_Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

#[derive(Debug)] 
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
    dbg!(m);

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();
    let a: i8 = match y {
        Some(v) => v,
        None => {
            println!("None");
            0
        }
    };

    println!("a: {a}");

    println!("sum: {sum}");

    let coin: Coin = Coin::Quarter(UsState::Alaska);

    println!("Amount in cents: {}", value_in_cents(&coin));

    let dice_roll = 9;

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    if let Coin::Quarter(x) = &coin {
        if x.existed_in(2000) {
            println!("{coin:?} coin existed in 2000");
        }
    }
}
fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {
    println!("num_spaces: {num_spaces}");
}
fn reroll() {}
