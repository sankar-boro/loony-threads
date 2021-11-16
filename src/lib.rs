use std::future::{Future};
use futures::executor::block_on;
use std::thread::{self, JoinHandle};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;

pub fn run<T, R>() -> (Sender<T>, JoinHandle<()>)
where 
    T: Fn() -> R + Send + 'static,
    R: Future<Output=()> + Send + 'static
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


pub fn run_arb<T, R>() -> (Sender<Arc<ArbiterCommand<T, R>>>, JoinHandle<()>)
where
    T: Fn() -> R + Send + 'static + Sync,
    R: Future<Output=()> + Send + 'static
{
    let (tx, rx) = channel::<Arc<ArbiterCommand<T, R>>>();

    let handle = thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(_fn) => {
                    let f = _fn.fut.as_ref();
                    // f();
                    block_on(f());
                },
                Err(_) => {
                    println!("Oops!! Something went wrong.");
                }
            }
        }
    });
    
    (tx, handle)
}

pub struct ArbiterCommand<T, R> 
where
    T: Fn() -> R + Send + 'static + Sync,
    R: Future<Output=()> + Send + 'static
{
    pub fut: Arc<T>
}