use loony_threads::run;

async fn hello() {
    println!("Hello! World.");
}
fn main() {
    let a = hello;

    let (tx, handle) = run();

    tx.send(a).unwrap();
    handle.join().unwrap();
}