use std::thread;
use std::time::Duration;

fn main() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });
    // Uncomment the code below to see what happens if we put the join before
    // main thread loop
    handle.join().unwrap();

    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    // Waiting for all threads to finish using join Handles
    // prevent program to stop before spawned handle finishes it's job
    // handle.join().unwrap();
}
