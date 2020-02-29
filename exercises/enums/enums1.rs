// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!


#[derive(Debug)]
enum Message {
    Quit,
    Echo(String),
    Move(u32, u32),
    ChangeColor(u32, u32, u32)
    // TODO: define a few types of messages as used below
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo(String::from("Echo")));
    println!("{:?}", Message::Move(2,3));
    println!("{:?}", Message::ChangeColor(25, 255, 255));
}
