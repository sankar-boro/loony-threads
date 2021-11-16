use std::sync::Arc;
use loony_threads::{run, run_arb, ArbiterCommand};

async fn hello() {
    println!("Hello! World.");
}
fn main() {
    // let a = hello;
    // let (tx, handle) = run();
    // tx.send(a).unwrap();
    // handle.join().unwrap();

    let a = hello;
    let (tx, handle) = run_arb();

    let arb = Arc::new(ArbiterCommand {
        fut: Arc::new(a)
    });
    tx.send(arb.clone()).unwrap();
    tx.send(arb.clone()).unwrap();
    tx.send(arb.clone()).unwrap();
    tx.send(arb.clone()).unwrap();
    tx.send(arb.clone()).unwrap();
    tx.send(arb.clone()).unwrap();
    tx.send(arb.clone()).unwrap();
    tx.send(arb.clone()).unwrap();
    tx.send(arb.clone()).unwrap();
    tx.send(arb.clone()).unwrap();
    tx.send(arb.clone()).unwrap();
    tx.send(arb.clone()).unwrap();
    tx.send(arb.clone()).unwrap();
    tx.send(arb.clone()).unwrap();

    handle.join().unwrap();
}