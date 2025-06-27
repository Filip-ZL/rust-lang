fn main() {

    // Panic function -> cause program to Crash
    // panic!("Crash and burn");
    
    let v = vec![1, 2, 3];

    // we can backtrace code so the panic still display the error correctly
    // thread 'main' panicked at src/main.rs:8:6:
    // index out of bounds: the len is 3 but the index is 10
    // note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace    
    // ^^^^ We get error above when tryin' to access not existing element!
    v[10];
}
