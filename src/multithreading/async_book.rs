use async_std::task;
use futures::executor::block_on;
use std::time::Duration;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hello_world() {
        let future = hello_world();
        block_on(future); // executor blocks current thread
    }

    #[test]
    fn test_perform() {
        let future = perform();
        block_on(future); // executor blocks current thread
    }
}
