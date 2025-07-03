struct CustomPointer {
    data: String,
}

impl Drop for CustomPointer {
    fn drop(&mut self) {
        println!("Dropping CustomPointer with data `{}`!", self.data);
    }
}

fn main() {
    {
        let c =CustomPointer {
            data: String::from("my stuff"),
        };

        let d = CustomPointer {
            data:String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }
    {
        // in cases smart pointers manage locks and we want to force the drop so
        // the lock is released. Check code below:
        let c = CustomPointer {
            data: String::from("some data"),
        };
        println!("CustomSmartPointer created.");
        // c.drop();        // NOTE: This will panic
        drop(c);            // drop is in the prelude!
        println!("CustomSmartPointer dropped before the end of main.");

        // Rust won't let you call the Drop trait's drop method manually
        // you'd need tocall std::mem::drop function provided by standart 
        // library.
        // Rust will call the drop method at the end of the program anyways.
        // This would lead to double free error.
    }
}
