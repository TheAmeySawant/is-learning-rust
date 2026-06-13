use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        //This is a clousre ↓
        user_preference.unwrap_or_else(|| self.most_stacked())
        // Syntax: |argument| body
        // The clousre is an anonymous function, i.e, can be used immediately without name, compiler implicitly determines Type, and can accesss out of it's scope variable (variables from outside the clouser), as &var / &mut var / var (move).
        // used to handle values, here this closure is called by unwrap_or_else,
        // when the user_preference is None.
        // So the return type of closure becomes the return type of the unwarp method, i.e, ShirtColor
        // we don't need the None so we don't define argument var in |args|.
        // we just call most_stacked method to return the most stacked ShirtColor for giveaway
    }

    fn most_stacked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    //The type of clouser args and return value are determined when this clouser is called.
    //That's how a clouser of concrete type is created.
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();

    // move keyword before || of closure tells it to move env vars used in it's body
    // move is needed because println macro uses immutable reference of list.
    //there is a chance of main Thread to outlive or underlive the new Thread. Thererfore move is needed, otherwise dangling poniter might be used.

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    // println!("After calling closure: {list:?}");

    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
        Rectangle {
            width: 4,
            height: 12,
        },
    ];

    let mut num_sort_operations = 0;

    //for (r.height ,r.width), sort by height first, if multiple elements have same height sort those by width
    //for r.height, other sort by height, if there are multiple elements with same height then they are ordered in their initial order (as they were)
    list.sort_by_key(|r| {
        //This counting works but if move of an environment var is done in this, then first iteration can move value but on second how can it move an already moved value therefore doesn't compile
        num_sort_operations += 1;
        (r.height, r.width)
    });
    println!("{list:#?}");

    let v1 = vec![1, 2, 3];

    //Each call to next(), eats up an item from the iterator.
    //Methods that call next() are called 'consuming adapters'.
    //sum() uses to next() to fetch all elements of vector v1.
    //thus sum() consumes the whole iterator.
    //sum() requires to explicitly mention the type of value is returned.
    let sum: i32 = v1.iter().sum();

    println!("{}", sum);

    let v1: Vec<i32> = vec![1, 2, 3];

    //map() calls next() consuming the Iterator to perform operation(s) given in the closure.
    //map() returns new iterator with the updated values after the operations are performed of the closure on each element. 
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    println!("{:?}", v2);
}


#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

//In filter() returns a new iterator containing only the elements fullfilling the condition defined in the closure body
//then collect() is used to convert the Iterator to Vector collection.
fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}