use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match get_resistor_color_by_value(value) {
        Some(col) => col.to_string(),
        None => "value out of range".to_string(),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    (0..=9).map(|val| get_resistor_color_by_value(val).unwrap()).collect()
}


fn get_resistor_color_by_value(value: usize) -> Option<ResistorColor> {
    match value {
        0 => Some(ResistorColor::Black),
        1 => Some(ResistorColor::Brown),
        2 => Some(ResistorColor::Red),
        3 => Some(ResistorColor::Orange),
        4 => Some(ResistorColor::Yellow),
        5 => Some(ResistorColor::Green),
        6 => Some(ResistorColor::Blue),
        7 => Some(ResistorColor::Violet),
        8 => Some(ResistorColor::Grey),
        9 => Some(ResistorColor::White),
        _ => None,
    }
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ResistorColor::Black => write!(f, "Black"),
            ResistorColor::Brown => write!(f, "Brown"),
            ResistorColor::Red => write!(f, "Red"),
            ResistorColor::Orange => write!(f, "Orange"),
            ResistorColor::Yellow => write!(f, "Yellow"),
            ResistorColor::Green => write!(f, "Green"),
            ResistorColor::Blue => write!(f, "Blue"),
            ResistorColor::Violet => write!(f, "Violet"),
            ResistorColor::Grey => write!(f, "Grey"),
            ResistorColor::White => write!(f, "White"),
        }
    }
}