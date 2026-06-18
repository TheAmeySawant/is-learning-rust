//! Use 'std::thread::available_parallelism().unwrap().get()' to get the recommended no. of threads to be used on the current system.
//! It returns the no. of thread that can acutally run prallel on the current sytem (hardware)
//! It's an estimate of no. of threads, i.e it may return less no. threads than the system actually have, but never returns more than acutally are.
//! 
//! thread::spawn creates an actual os thread
//! 
//! Data transfer between mutliple threads:
//! 1. Use move with closure, one way communication, main thread to user thread only
//! 2. Channels return tx (transmitter) and rx (reciever) for communication, tx sends data and rx stops the current thread and waits for the data from tx. We use move with closure to move tx from main thread to user thread and then use tx to send data from user thread to main thread.
//! We can have multiple tx for multiple threads, just need to clone tx for multiple threads.
//! 3. Shared State (Shared Memory), here we can use Mutex (Mutual Exclusion: only one thread access a data at a time).
//! Mutex is a datastructure which encapsultes the data, and we use lock() method to access the data.
//! Mutex doesn't explicitly store data in heap, where to store the data depends on the data (stack or heap).
//! for i32 Mutex(i32) is stored on the stack.
//! lock() returns LockResult<MutexGuard<'_, T>>, stops the current thread until it gets LockResult Variant.
//! 
//!LockResult similar to Result, variants:
//! Ok(MutexGuard)
//! Err(PoisonError)
//! 
//! MutexGuard is a Smart Poiter:
//! holds the lock
//! Gives exclusive access to the protected value
//! Unlocks the mutex when dropped (automatically drops when Mutex)
//! 
//! PoisonError:
//! when a thread leaves shared data in a potentially inconsistent state (maybe thread panics while holding the lock of the data). To prevent data corruption, Rust marks that lock as poisoned.
//! This causes all subsequent threads calling lock(), return PoisoError instead of MutexGuard. 
//! 


use std::sync::{Arc, mpsc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let n = thread::available_parallelism().unwrap().get();

    println!("Available CPUs: {}", n);

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("{v:?}");
        drop(v);
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");

            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    let (tx, rx) = mpsc::channel::<String>();

        // --snip--

    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_micros(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_micros(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        
    }

    println!("m = {}", m.lock().unwrap());

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

}
