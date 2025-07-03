use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // Comment the line below out, returns in error... val was moved!
        // println!("val is {val}")
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
    // try_recv similar to recv() but will get the value immediately returning
    // Result<T, E> Ok --> holding a message, Err --> no messages at the time
}
