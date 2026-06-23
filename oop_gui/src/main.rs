//! Generics + Trait Bounds (Static Dispatch)
//! 
//! Ex: struct MyScreen<T>
//! where T: Draw {
//!     comp: Vec<Box<T>>
//! }
//! 
//! T is tied with struct MyScreen
//! 
//! here T can be of any one of the concrete type that implements Draw Trait
//! i.e, in an instance of MyScreen, T can be either Button or SelectBox, therefore the comp (Vector) can hold only that type of elements.
//! 
//! This happens because compiler, generates separate struct code for all the Types we have used in src code for that struct. (Note: generics are not limited to Struct)
//! 
//! Trait Objects (Dynamic Dispatch)
//! 
//! Ex: struct Screen{
//!     components: Vec<Box<dyn Draw>>
//! }
//! 
//! here it just says that components (Vector) can have any type implementing Draw Trait.
//! 


use oop_gui::{Button, Draw, MyScreen, Screen};

struct SelectBox {
    width: i32,
    height: i32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Select Box Draw method called: w: {} h: {}\n\tOptions: {:?}",
            self.width, self.height, self.options
        );
    }
}

fn main() {
    println!("Hello, world!");

    //Using Trait Object (Dynamic dispatch)

    let b1 = Button {
        width: 5,
        height: 10,
        label: "hello".to_string(),
    };

    let sb1 = SelectBox {
        width: 20,
        height: 43,
        options: vec!["A".to_string(), "B".to_string(), "C".to_string()],
    };

    let screen = Screen {
        components: vec![Box::new(b1), Box::new(sb1)],
    };

    screen.run();

    //Using Trait Bound (Static dispatch)

    // let b1 = Button {
    //     width: 5,
    //     height: 10,
    //     label: "hello".to_string(),
    // };

    // let sb1 = SelectBox {
    //     width: 20,
    //     height: 43,
    //     options: vec!["A".to_string(), "B".to_string(), "C".to_string()],
    // };

    // let my_screen = MyScreen {
    //     comps: vec![Box::new(b1), Box::new(sb1)],
    // };

    // my_screen.start();
}
