// Traits, group of methods which can be implemented in a Type
// Traits are similar to interface of Java.
use std::fmt::Display;

pub mod my_traits {
    pub trait Summary {
        fn get_formatted_name(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {})", self.get_formatted_name())
        }
    }

    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn get_formatted_name(&self) -> String {
            format!("@{}", self.author)
        }
    }

    pub struct SocialPost {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub repost: bool,
    }

    impl Summary for SocialPost {
        fn get_formatted_name(&self) -> String {
            format!("@{}", self.username)
        }
    }

    pub fn notify(item: &impl Summary) {
        println!("{}", item.summarize());
    }
}

pub fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


pub struct Point<X1, Y1> {
    pub x: X1,
    pub y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    pub fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}



pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

impl<T: Display> ToString for Pair<T> {
    fn to_string(&self) -> String {
        "You called to string method".to_string()
    }
}
pub struct Anime{}