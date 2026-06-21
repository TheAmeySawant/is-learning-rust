//! *** FUTURE ***
//! 
//! Future is a trait (state machine i.e, keeps track of state)
//! Future is lazy, await needs to be called on it to use it, similar to the next method calling on iterators.
//! Future can be in two states (by calling poll() method on it):
//! Pending (not ready, try again later)
//! Ready (Output)
//! 
//!  *** ASYNC ***
//! 
//! async block creates future
//! async block looks like,
//! 
//! async {
//!     code..
//! }
//! 
//! async block tells CPU that this block of code can be paused.
//! 
//! async fn returns a Type implementing Future, which is lazy and needs to be polled.
//! 
//!  *** AWAIT ***
//! 
//! await is always used in async block.
//! await tells CPU, where in the async block code can be paused achieving concurrency (running task 2 by pausing task 1 if task 1 not ready in a single thread)
//! await polls the Future, if it's pending pauses the future and control goes back to whaterver is driving it (the executor) without blocking the current thread.
//! 
//! *** THREAD ***
//! 
//! Thread allow real parallelism, individual thread runs on independent OS threads, own separate stack, heap, program counter,etc. therefore they are expensive.
//! 
//! *** TASK ***
//! 
//! a unit (part/ defined inside) of async work given to runtime Wraps a future i.e, takes future as argument.
//! Lightweight than Thread, cooperatively scheduled — only yields at .await points.
//! 
//! *** RUNTIME ***
//! 
//! runtime is (tokio, async-std) the thing that actually polls Futures/tasks to completion. Has two halves:
//! 
//! executor — schedules and polls tasks
//! reactor — watches OS events (I/O, timers) and wakes the right task when data's ready
//! 
//! 
//! This happens on a single thread:-
//! asyn fn : crates future
//! .await : polls future, pauses if pending
//! task : future handled/driven by runtime, 
//! runtime : polls tasks repeatedly until ready, threads are what it polls on

use std::time::Duration;
use trpl::{Either, Html};

async fn page_title(url: &str) -> (&str, Option<String>) {
    let response = trpl::get(url).await;
    let response_text = response.text().await;

    // let response_text = trpl::get(url).await.text().await;

    let title = Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html());

    (url, title)
}
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url1_future = page_title(&args[1]);
    let url2_future = page_title(&args[2]);

    trpl::block_on(async {
        let (url, maybe_title) = match trpl::select(url1_future, url2_future).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!(
            "\n\n{url} loaded first\nTitle: {}",
            maybe_title.unwrap_or("Title Not Found".to_string())
        );
    });

    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(50)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(50)).await;
        }

        handle.await.unwrap();

        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the third task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        // let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the fourth task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        // };

        fut1.await;

        // trpl::join(fut1, fut2).await;
    });
}
