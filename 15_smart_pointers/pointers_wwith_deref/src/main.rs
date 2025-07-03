use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {

    {
        let x = 5;
        let y = &x;

        
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    {
        // Deref coercion
        // Converts a reference to a type that implements the Deref trait into 
        // a referenceto another type.
        let m = MyBox::new(String::from("Rust"));

        hello(&m)
    }
}


fn hello(name: &str) {
    println!("Hello, {name}")
}