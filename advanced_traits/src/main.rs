//! Advanced Traits
//!
//! Associated Types:
//! Associated Types act as placeholder similar to generic types.
//! The main difference is that Trait with generic types can be implemented for multiple different types on the struct or enum.
//! But Trait with associated types can only be implemented once on a struct or enum.
//!
//! Generic Types allow you to choose any Type(s) you want to implement Trait for.
//! Associated Types allow you to choose any but only one Type to implement Trait for.
//!
//!
//! Default Generic Parameter and Operator Overloading
//!
//! trait Add<Rhs=Self> {
//!    type Output;
//!
//!    fn add(self, rhs: Rhs) -> Self::Output;
//!}
//!
//! Add trait can be overriden for a struct or enum to override + operator.
//!
//! Rhs is a Generic type and Self is it's Default Type.
//! so when we implement Add trait without Rhs Generic Type, it's Default value(Self) is set for Add trait.
//!
//! name of the parameter doesn't matter, only the order of Type of parameter is considered.
//! 
//! 
//! 

use std::ffi::os_str::Display;
use std::ops::Add;

trait Iterator<Rhs = Self> {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    fn add(&self, rhs: Rhs) -> Self::Item;
}

struct Counter {
    value: i32,
}

impl Iterator for Counter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.value)
    }

    fn add(&self, other: Counter) -> Self::Item {
        self.value + other.value
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
struct PowerPoint {
    x: i32,
    y: i32,
}

impl Add<PowerPoint> for Point {
    type Output = Point;

    fn add(self, other: PowerPoint) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct NewPoint {
    x: i32,
    y: i32,
    z: i32,
}

impl OutlinePrint for NewPoint {}

impl fmt::Display for NewPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let point = Point { x: 1, y: 0 } + PowerPoint { x: 2, y: 3 };
    println!("{:?}", point);

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    println!("A baby dog is called a {}", Dog::baby_name());

    let n_point = NewPoint { x: 5, y: 7, z: 11 };

    n_point.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
