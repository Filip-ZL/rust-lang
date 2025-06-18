struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0.0
    }

    fn can_hold(&self, comp_rect: &Rectangle) -> String {
        if self.area() > comp_rect.area() { String::from("Yes!")} else  {String::from("No!")}
    }

    fn square(size: f64) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    
    let mut rect1 = Rectangle {
        width: 30.0,
        height: 50.0,
    };
    let rect2 = Rectangle {
        width: 10.0,
        height: 40.0,
    };
    let rect3 = Rectangle {
        width: 60.0,
        height: 45.0,
    };
    // println!("area of rectangle of sides length {}, {} is {}", rect.width, rect.height, rect.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(32.0);

}
