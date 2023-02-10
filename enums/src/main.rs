#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        dbg!(self);
    }
}

struct QuiteMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

fn main() {
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));
    dbg!(home, loopback);

    let quite_msg = QuiteMessage {};
    let move_msg = MoveMessage { x: 0, y: 1 };
    let write_msg = WriteMessage(String::from("hello"));
    let change_color_msg = ChangeColorMessage(256, 256, 256);
}
