//use std::option;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

pub fn main() {
    let n1 = Node {value: 4, next: None};
    let n2 = Node {value: 5, next: Some(Box::new(n1))};
    println!("{:#?}", n2);
}
