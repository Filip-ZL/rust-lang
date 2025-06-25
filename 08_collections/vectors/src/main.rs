fn main() {

    // vector basics
    {
        let v: Vec<i32> = Vec::new();
        // Rust will give the type automatically --> default i32
        let v2 = vec![1, 2, 3];

        let mut v3 = Vec::new();

        v3.push(5);
        v3.push(6);
        v3.push(7);
        v3.push(8);
    }

    // Reading elements of vector
    {
        let v = vec![1, 2, 3, 4, 5];

        let third: &i32 = &v[2];
        let third: Option<&i32> = v.get(2);

        match third {
            Some(v) => println!("The value of the index is {v}"),
            None => println!("There's no third element"),
        }

        // Exceed the array length
        let v = vec![10; 8];

        let high_el: Option<&i32> = v.get(100);

        let no = match high_el {
            Some(v) => v,
            None => &0,
        };

        println!("Index is larger than vector length. Therefore I return {no}");
        
        // let high_el: &i32 = &v[100];
        println!("Index is larger than vector length. Program panic. This won't print!");
    }

    // NOTE: We can't have mutable reference and immutable reference
    // Thus code below won't compile!
    {
        let mut v = vec![1, 2, 3, 4];

        let first = &v[0];

        v.push(10);

        // println!("The first element is: {first}")
    }

    // Iterating over vector
    {
        let mut v = vec![1, 2, 3, 4];
        
        for element in &v {
            println!("{element}");
        }
        for element in &mut v {
            *element += 50;
        }
        for element in &v {
            println!("{element}");
        }
    }

    // Storing different types in vector
    // using enum
    {
        enum SpreadSheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        };

        let row = vec![
            SpreadSheetCell::Int(10),
            SpreadSheetCell::Float(8.21),
            SpreadSheetCell::Text(String::from("blue")),
        ];
    }

}
