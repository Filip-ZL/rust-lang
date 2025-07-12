use std::slice;

// static variables -> basically global variables, might be problematic if more
// threads are tryin' to access them at the same time (e.g.)-> data races
static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    
    // Dereferencing a Raw Pointer
    {
        let mut num =5;

        let r1 = &raw const num;
        let r2 = &raw mut num;

        // there's usually no good reason to access random part of memory
        // could lead to memory issues
        let adress = 0x012345usize;
        let r = adress as *const i32;

        // we can't dereference raw pointers in safe mode
        // thus we need to use unsafe rust

        unsafe {
            println!("r1 is : {}", *r1);
            println!("r2 is : {}", *r2);
        }
    }

    // Calling an Unsafe Function or Method
    {
        unsafe fn dangerous() {}

        unsafe {
            dangerous();
        }
    }

    // Creating a Safe Abstraction over Unsafe Code
    {
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        // this function needs to be implemented using unsafe rust!
        // Check example below!
        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    // Using extern Functions to Call External Code
    {
        unsafe{
            println!("Absolute valueof -3 according to C: {}", abs(-3));
        }
        // Import following line if abs imported as 'safe'
        // println!("Absolute valueof -3 according to C: {}", abs(-3));

    }

    //Accessing or Modifying a Mutable Static Variable
    {
        // Modifying Static Variable is unsafe (accessing immutable static
        // variable is safe)
        println!("name is {HELLO_WORLD}");

        unsafe {
            // SAFETY: This is only called from a single thread in `main`.
            add_to_count(3);
            println!("Counter: {}", *(&raw const COUNTER));
        }
    }
}



fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    // this generates the raw pointer to the slice
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            // must be sure thatthe pointer and it's lenght is valid
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid)
        )
    }
}

// Extern functions (example)
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}

// We know that abs function from C'standard library does not have any memory
//safety consideration... we can import it as safe
// unsafe extern "C" {
//     safe fn abs(input: i32) -> i32;
// }

// we can export Rust functions to run within' others language environment
#[unsafe(no_mangle)]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// Here we access the static variable and add 1 to count
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc
    }
}

// Implementing Unsafe Trait
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}