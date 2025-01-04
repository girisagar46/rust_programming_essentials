// Crate-scope attribute
#![allow(dead_code)]

pub enum Color {
    Red,
    Green,
    Blue,
    Unknown,
}

pub enum HouseLocation {
    Number(i32),
    Name(String),
    Unknown,
}
