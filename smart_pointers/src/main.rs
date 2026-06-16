//!
//! Smart Pointers are type of reference used for different purpose more than just referencing
//!
//!
//! 1. Box<T> is a smart pointer, allows you to allocate value on heap, useful for recurring data structure types
//! ex: linked list : struct List { n: i32, next: List } this is not possible because size of List can be infinite, therefore we define next as Box<List> type in List struct.
//! so compiler knows size of List is [i32 + usize (size of a pointer, depending on the system)]
//!
//! 
//! 
//! 2. Traits like Deref, DerefMut and Drop allow you to create new smart pointer as you like.
//! Deref : allows you to convert &T to &U or &mut to &U.
//! DerefMut : allows you to convert &mut T to &mut U.
//! Drop : allows you to do stuff right before the instance goes out of scope
//! &T to &mut U is not allowed because, we can have mutliple immutable references therefore we can convert &mut to &U without worrying anything
//! but when converting immutable refernce (&T) to mutable refernce (&mut U) we need to know for sure that there are not references other than this, neither mutable nor immutable.
//!
//! 
//! 
//! 3. Rc<T> is also a smart pointer, Rc stands for Reference Counted.
//! Normally we have single owner per value, but with this we can have multiple owners of the same value (on heap).
//! Rc<T> keeps count of no. of owners and when it's 0, that value is dropped.
//!
//! Instead of deep cloning (creating separate copy in heap) it adds new owner to the same data (no new data is stored in heap), saving performace by eliminating allocation task and saves memory by no need to store new data on the heap. win-win situation right?
//! 
//! Rc::clone(&Rc<T>) is used to create a strong reference, this increase the strong_count by 1.
//! Initially when a Rc<T> variable is created strong_count = 1, as this variable itself is the sole owner of this value.
//! When strong_count == 0, the value is cleaned up from the heap.
//! 
//! we can also create a weak reference using Weak::new() which returns a Weak<T>.
//! strong refernces have ownership, but weak references don't have ownership.
//! therefore when cleaning up the value from heap, the strong_count has to be 0, and the weak_count (no.of weak references) doesn't matter.
//! 
//! so when using weak references, use upgrade method to get Option<Rc<T>>, we get Some if value still exist, otherwise None.
//!
//! 
//! 
//! 4. RefCell<T> is again a smart pointer, it allows interior mutablility i.e, an immutable variable containing a mutable value like,
//! let a = "hi".to_string();
//! with let a = RefCell::new("hi".to_string()); we can use borrow_mut method on immutable reference of a to get RefMut<T> type and mutate it.
//! borrow method Ref<T> smart pointer.
//! RefCell<T> tracks no. active Ref<T> and RefMut<T>
//! One mutable borrow, multiple immutable borrow
//!
//! RefCell uses unsafe code i.e, code which doesn't find at any errors at compile time, instead panic occures when unpredictable error might happen

use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(t: T) -> Self {
        MyBox(t)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

enum List2 {
    Cons2(i32, Rc<List2>),
    Nil2,
}

use crate::List2::{Cons2, Nil2};

#[derive(Debug)]
enum List3 {
    Cons3(Rc<RefCell<i32>>, Rc<List3>),
    Nil3,
}

use crate::List3::{Cons3, Nil3};

#[derive(Debug)]
enum ListCycle{
    CycleCons(i32, RefCell<Rc<ListCycle>>),
    CycleNil
}

use crate::ListCycle::{CycleNil, CycleCons};

impl ListCycle {
    fn tail (&self) -> Option<&RefCell<Rc<ListCycle>>> {
        match self {
            CycleCons(_, item) => Some(item),
            CycleNil => None
        }
    }
}

fn main() {
    //although i32 is a scalar type, i.e stored on stack
    //but using Box the value is stored on heap and it's owned reference (Box) is stored in var b.
    let b = Box::new(5);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!("b = {list:?}");

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, y.0);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    drop(c);
    println!("CustomSmartPointers created");

    let a = Rc::new(Cons2(5, Rc::new(Cons2(10, Rc::new(Nil2)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons2(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons2(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons3(Rc::clone(&value), Rc::new(Nil3)));

    let b = Cons3(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons3(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");


    let a = Rc::new(CycleCons(5, RefCell::new(Rc::new(CycleNil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(CycleCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack.
    // println!("a next item = {:?}", a);
}

fn hello(name: &str) {
    println!("Hello {name}!");
}
