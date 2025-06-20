// enum example
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write (String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    {    
        // Simple enum usage example
        let four = IpAddrKind::V4(127, 0, 0, 1);
        let six = IpAddrKind::V6(String::from("::1"));
    }
    {
        // similar to struct we can define functions under enum type
        let m = Message::Write(String::from("Hello"));
        m.call();
    }
    {
        // Option - Some, None
        let some_number = Some(5);
        let some_char = Some('e');

        let absent_number: Option<i32> = None;
    }

}

fn route(ipaddr: IpAddrKind) {}     // example of function declaration with dtype of IpAddrKind
