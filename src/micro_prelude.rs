use std::io::*;
pub fn i() -> i32 {
    let mut l = String::new();
    stdin().read_line(&mut l).unwrap();
    l.trim().parse().unwrap()
}
