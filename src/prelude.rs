use std::convert::TryFrom;
use std::fmt::Binary;
use std::io::*;
//use std::ops::BitAnd;
use std::{
    convert::TryInto,
    fmt::Debug,
    time::{SystemTime, UNIX_EPOCH},
};

pub fn pint() -> i32 {
    let mut l = String::new();
    stdin().read_line(&mut l).unwrap();
    l.trim().parse().unwrap()
}

pub fn nano() {
    1_000_000_000;
}

pub fn now() -> (u128, u128) {
    let tau = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let tau_s = u128::from(tau.as_secs());
    let tau_n = tau.as_nanos() - tau_s;
    (tau_s, tau_n)
}
pub fn l<M: Debug>(message: M) {
    eprintln!("[SimpleLog 𝜏{:?}] {:#?}", now(), message)
}
pub fn lh<S: Debug, M: Debug>(source: S, message: M) {
    eprintln!("[{:?} 𝜏{:?}] {:#?}", source, now(), message)
}
pub fn ll<M: Debug>(message: M) {
    eprintln!("[SimpleLog 𝜏{:?}] {:?}", now(), message)
}
pub fn llh<S: Debug, M: Debug>(source: S, message: M) {
    eprintln!("[{:?} 𝜏{:?}] {:?}", source, now(), message)
}

pub fn flip_case(x: &str) -> String {
    let mut y = String::new();
    for c in x.chars() {
        if c.is_uppercase() {
            y.push_str(c.to_lowercase().to_string().as_str())
        } else {
            y.push_str(c.to_uppercase().to_string().as_str())
        }
    }
    y
}

pub fn a_to_bitmap<T: TryFrom<isize>>(x: &str) -> T {
    match isize::from_str_radix(x, 2).unwrap().try_into() {
        Ok(y) => y,
        Err(_) => panic!("Failed to convert"),
    }
}

pub fn bitmap_to_a<T: Binary>(x: T) -> String {
    format!("{:b}", x)
}

pub fn alphabet() -> String {
    "abcdefghijklmnopqrstuvwxyz".to_string()
}

pub fn itou<T: TryInto<usize>>(x: T) -> usize {
    match x.try_into() {
        Ok(y) => return y,
        Err(_) => panic!("Oops"),
    }
}

pub fn nth_char(x: &String, n: usize) -> char {
    x.chars().nth(n).unwrap()
}

pub fn i_to_bit(x: u8) -> u128 {
    2 ^ (x as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_case_with_specials() {
        assert_eq!("ЩаЧлО! 228", flip_case("щАчЛо! 228").as_str());
    }

    #[test]
    fn test_can_get_nth_letter_of_the_alphabet() {
        let mut cbabc = Vec::new();
        for n in (-3 + 1)..3 {
            cbabc.push(nth_char(&alphabet(), itou((n as i32).abs())))
        }
        assert_eq!(vec!['c', 'b', 'a', 'b', 'c'], cbabc);
    }

    #[test]
    fn test_bitmaps_trip_to_strings() {
        assert_eq!(
            "101100".to_string(),
            bitmap_to_a(a_to_bitmap::<u32>("101100"))
        );
    }
}
