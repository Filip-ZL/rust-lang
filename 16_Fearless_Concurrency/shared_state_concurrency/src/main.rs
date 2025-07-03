use std::sync::Mutex;

fn main() {

    // mutex in single threaded context
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {m:?}");
}
