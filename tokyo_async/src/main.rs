//!
//!Sequential execution (f1.await; f3.await; f2.await)
//!Concurrent futures (join!) create a single task of multiple futures
//!Concurrent tasks (spawn) creates a single task of single future
//!
//! Blocking tasks [like heavy computation tasks, async block without any await] (spawn_blocking) creates a task of the blocking code. This is a special task, which is scheduled on a separate thread as it can block the whole thread, unable to switch with other tasks
//!
//!  tokio::task::spawn_blocking creates a task which runs on a separate OS thread. running paralle to Main thread or any other thread where thes tasks are concurrently implemented.
//!
//! tokio::task::yield_now method is used in a task/future to volunterily give up CPU, control goes back to runtimes tokio and runs other ready tasks.
//!
//!
//! it can be used instead of tokio::task::spawn_blocking, to run blocking task concurrently with some other tasks on the same thread by calling yield_now method.
//!
//! ex: async fn process_large_dataset() {
//!    for chunk in dataset {
//!        process(chunk);
//!
//!        tokio::task::yield_now().await;
//!    }
//! }
//!
//! yield means to give up.
//! | -------------------------- | ------------------ | ------------- |
//! | Operation                  | What yields?       | Waits?        |
//! | -------------------------- | ------------------ | ------------- |
//! | `yield_now().await`        | Current Tokio task | No            |
//! | `sleep(1s).await`          | Current Tokio task | Yes, 1 second |
//! | `std::thread::yield_now()` | OS thread          | No            |
//! | -------------------------- | ------------------ | ------------- |
//!
//!
//!The expected speed is usually(it may differ based on how each task is implemented):
//!
//!Sequential awaits   <- slowest
//!join!               <- faster
//!spawned tasks       <- similar to join!, sometimes slightly faster/slower
//!
//! select! is used to race between multiple futures/tasks, one of them wins (completes first), then the other futures/tasks are dropped/canceled. (usually used to set a timeout on future/task)
//!
//! you can also race a future with a task in select.
//!
//! simple ex: tokio::select!{
//!    //let's say this login future returns status of login (successful / incorrect credentials)
//!     status = login() =>  {
//!     code runs if login wins
//! },
//!     _ = tokio::sleep(Duration::from_secs(3)) => {
//!     code runs if timeout (future) of 3 seconds wins against login future
//! }
//! }
//!
//! Stream:
//! Stream is a wrapper on Iterator used for async tasks.
//! Iterator is Synchronous and Stream is Asynchronous.
//! tokio_stream::iter() associative function is used to create a stream from an iterator
//! to use a stream, next() method is used, pauses the task/future, returning control to runtime(tokio)
//! we need an Trait in scope which implements next() method to use it.
//! one of the Traits is tokio_stream::StreamExt
//! 
//! ***************************************************************
//! Rules of thumb:
//! 
//! CPU-bound, parallelizable work (chew through a big dataset where each chunk is independent) → reach for threads.
//! I/O-bound, highly concurrent work (juggling messages from many sources arriving at unpredictable times) → reach for async.
//! Need both → don't choose. Combine them, and let each handle what it's actually good at, you can reach for tokio

use std::future::Future;
use std::{
    thread,
    time::{Duration, Instant},
};
use tokio::{sync::mpsc, time::sleep};
use tokio_stream::StreamExt;


/// This fn takes max_time that can be taken by the future provided throught argument.
/// select race them so that if max_time hits (wins) future is cancelled and Err with max_time is returned otherwise if future wins, it's output is sent wrapped in Ok()
async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    tokio::select! {
        output = future_to_try => Ok(output),
        _ = tokio::time::sleep(max_time) => Err(max_time),
    }
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    let (tx, mut rx) = mpsc::channel(10);

    // let tx = Rc::new(tx);
    let tx1 = tx.clone();

    let f1 = async move {
        let vals = vec!["f1", "f1", "f1", "f1", "f1", "f1"];
        for val in vals {
            tx.send(val.to_string()).await.unwrap();
            sleep(Duration::from_millis(2)).await;
        }
    };

    let f2 = async {
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    };

    let f3 = async move {
        let vals = vec!["f3", "f3", "f3", "f3"];
        for val in vals {
            tx1.send(val.to_string()).await.unwrap();
            sleep(Duration::from_millis(2)).await;
        }
    };

    // let f1 = tokio::spawn(async move {
    //     let vals = vec!["f1", "f1", "f1", "f1", "f1", "f1"];
    //     for val in vals {
    //         tx.send(val.to_string()).await.unwrap();
    //         sleep(Duration::from_millis(2)).await;
    //     }
    // });

    // let f2 = tokio::spawn(async move {
    //     while let Some(value) = rx.recv().await {
    //         println!("received '{value}'");
    //     }
    // });

    // let f3 = tokio::spawn(async move {
    //     let vals = vec!["f3", "f3", "f3", "f3"];
    //     for val in vals {
    //         tx1.send(val.to_string()).await.unwrap();
    //         sleep(Duration::from_millis(2)).await;
    //     }
    // });

    // f1.await.unwrap();
    // f3.await.unwrap();
    // f2.await.unwrap();

    // f1.await;
    // f3.await;
    // f2.await;
    tokio::join!(f1, f2, f3);

    // let one_ms = Duration::from_millis(1);

    let a = async {
        println!("'a' started.");
        slow("a", 30);
        tokio::task::yield_now().await;
        slow("a", 10);
        tokio::task::yield_now().await;
        slow("a", 20);
        tokio::task::yield_now().await;
        println!("'a' finished.");
    };

    let b = async {
        println!("'b' started.");
        slow("b", 75);
        tokio::task::yield_now().await;
        slow("b", 10);
        tokio::task::yield_now().await;
        slow("b", 15);
        tokio::task::yield_now().await;
        slow("b", 350);
        tokio::task::yield_now().await;
        println!("'b' finished.");
    };

    // tokio::join!(a, b);
    // tokio::select! {
    //     _ = a => {},
    //     _ = b => {},
    // }

    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = values.iter().map(|n| n * 2);
    let mut stream = tokio_stream::iter(iter);

    while let Some(value) = stream.next().await {
        println!("The value was: {value}");
    }

    println!("Execution time {:2?}", start_time.elapsed());
}
