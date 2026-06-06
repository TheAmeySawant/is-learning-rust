#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
    area: u32,
}

impl Rectangle {
    // All functions defined in impl of a struct are Associated functions of that struct
    //The Associated functions that have self as first parameter are called Methods of that struct

    //Method
    fn area(&self) -> u32 {
        self.height * self.width
    }

    //Method
    fn calulate_area(&mut self) {
        self.area = self.area();
    }

    //Method
    fn can_hold_this_inside(&self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }

    //Associated functions
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
            area: 0
        }
    }
}

fn main() {
    let scale = 2;
    let mut rect1 = Rectangle {
        //dbg takes ownership of value and then returns back to width attribute.
        width: dbg!(30 * scale),
        height: 50,
        area: 0,
    };

    println!(
        "Using area FUNCTION, The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!(
        "Using area METHOD, The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    rect1.calulate_area();

    //Normal Debugging logging
    println!("{:#?}", rect1);

    //Mentos Debugging logging
    //This function takes ownership and returns back but why to make such a hassel when you can provide reference
    dbg!(&rect1);

    let rect2 = Rectangle {
        width: 10,
        height: 40,
        area: 0
    };

    //Associated Functions Call
    let rect3 = Rectangle::square(70);

    //Method Call
    println!("Can rect1 hold rect2? {}", rect1.can_hold_this_inside(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold_this_inside(&rect3));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
