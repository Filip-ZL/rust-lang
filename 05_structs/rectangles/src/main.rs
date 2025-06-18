#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

fn main() {
    
    // first approach
    // sides are not connected anyhow!
    // {    
    //     let a: f64 = 40.0;
    //     let b: f64 = 20.0;

    //     let area: f64 = area((a, b));

    //     println!("area of rectangle with sides of sizes {}, {} is {}", a, b, area);
    // }

    // Different approach using tuples
    // {
    //     let rect = (40.0, 20.0);
    //     let area: f64 = area(rect);

    //     println!("area of rectangle with sides of sizes {}, {} is {}", rect.0, rect.1, area);
    // }

    // using structs
    {
        let scale = 2.0;
        let rect = Rectangle {
            width: dbg!(40.0 * scale),
            height: 20.0,
        };
        let area: f64 = area(&rect);
        println!("Rectangle: {rect:#?}");
        dbg!(&rect);
        println!("area of rectangle with sides of sizes {}, {} is {}", rect.width, rect.height, area);
        
    }
}

// fn area(a: f64, b: f64) -> f64 {
// //     a * b
// // }

// fn area(dimmensions: (f64, f64)) -> f64 {
//     dimmensions.0 * dimmensions.1

// }

fn area(rectangle: &Rectangle) -> f64 {
    rectangle.width * rectangle.height

}