//! OOP style coding (abstraction, encapsulation, polymorphism) using structs, traits (bound & dyn), enums, etc.
//! inheritance is not supported in Rust.
//! 
//! OOP is not the best coding style/pattern for rust.

use oop_blog::{Post, EnPost};

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve(); //First time approve
    assert_eq!("", post.content());

    post.approve(); //Second time approve
    assert_eq!("I ate a salad for lunch today", post.content());

    let mut post = EnPost::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve(); //First time approve
    assert_eq!("", post.content());

    post.approve(); //Second time approve
    assert_eq!("I ate a salad for lunch today", post.content());
}