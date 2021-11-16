use std::future::{Future};
use futures::executor::block_on;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{channel, Sender};

pub fn run<T, F>() -> (Sender<T>, JoinHandle<()>)
where 
    T: Fn() -> F + Send + 'static,
    F: Future<Output=()> + Send + 'static
{
    let (tx, rx) = channel::<T>();

    let handle = thread::spawn(move || {
        match rx.recv() {
            Ok(_fn) => {
                block_on(_fn());
            },
            Err(_) => {
                println!("Oops!! Something went wrong.");
            }
        }
    });

    (tx, handle)
}
