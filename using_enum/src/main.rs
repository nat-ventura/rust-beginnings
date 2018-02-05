// USING A STRUCT

// fn main() {

//     enum IpAddrKind {
//         V4,
//         V6,
//     }

//     struct IpAddr {
//         kind: IpAddrKind,
//         address: String,
//     }

//     let home = IpAddr {
//         kind: IpAddrKind::V4,
//         address: String::from("127.0.0.1"),
//     };

//     let loopback = IpAddr {
//         kind: IpAddrKind::V6,
//         address: String::from("::1"),
//     };

// }



// USING THE ENUM EFFICIENTLY

enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));

// an advantage of using the enum over the struct: each variant can have
// different types and amounts of associated data

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// like

// struct QuitMessage;
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String);
// struct ChangeColorMessage(i32, i32, i32);

// but if we used the different structs, which each have their own type,
// we wouldn't be able to as easily define a function that could
// take any of these kinds of messages
// as we could with the Message enum, which is a single type :)

// we can also define methods on enums

impl Message {
    fn call(&self) {
        // method body
    }
}

let m = Message::Write(String::from("hello"));
m.call();

// the body of the method would use self to get the value that we called the method on
// `m` is what the self will be in the body of the `call` method when `m.call()` runs


