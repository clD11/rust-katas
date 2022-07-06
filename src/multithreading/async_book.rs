use async_std::task;
use futures::executor::block_on;

use {
    futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
    },
    std::{
        future::Future,
        pin::Pin,
        sync::atomic::compiler_fence,
        sync::mpsc::{sync_channel, Receiver, SyncSender},
        sync::{Arc, Mutex},
        task::{Context, Poll, Waker},
        thread,
        time::Duration,
    },
};

// async builds state machine that implements Future
async fn hello_world() {
    println!("hello world")
}

async fn learn_song() -> String {
    println!("learning song");
    task::sleep(Duration::from_millis(2)).await;
    println!("learnt song");
    String::from("my great song")
}

async fn sing_song(song: String) {
    println!("signing song: {}", song)
}

async fn learn_and_sing() {
    let song = learn_song().await;
    sing_song(song).await; // cannot use block_on to execute future as executor inside executor
}

async fn dance() {
    for _ in 1..5 {
        println!("dancing");
        task::sleep(Duration::from_millis(3)).await;
    }
}

async fn perform() {
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1, f2);
}

// Timer

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> TimerFuture {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// Executor

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

struct Task {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let future = hello_world();
        block_on(future); // executor blocks current thread and polls the future
    }

    #[test]
    fn test_perform() {
        let future = perform();
        block_on(future); // executor blocks current thread
    }
}
