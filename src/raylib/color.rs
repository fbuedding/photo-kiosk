#![allow(dead_code)]
use super::RColor;

#[derive(Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self {
            r: (value >> 24) as u8,
            g: ((value >> 16) & 0xff) as u8,
            b: ((value >> 8) & 0xff) as u8,
            a: (value & 0xff) as u8,
        }
    }
}

impl From<Color> for RColor {
    fn from(value: Color) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]

pub const LIGHTGRAY: Color = Color {
    r: 200,
    g: 200,
    b: 200,
    a: 255,
}; // Light Gray
pub const GRAY: Color = Color {
    r: 130,
    g: 130,
    b: 130,
    a: 255,
}; // Gray
pub const DARKGRAY: Color = Color {
    r: 80,
    g: 80,
    b: 80,
    a: 255,
}; // Dark Gray
pub const YELLOW: Color = Color {
    r: 253,
    g: 249,
    b: 0,
    a: 255,
}; // Yellow
pub const GOLD: Color = Color {
    r: 255,
    g: 203,
    b: 0,
    a: 255,
}; // Gold
pub const ORANGE: Color = Color {
    r: 255,
    g: 161,
    b: 0,
    a: 255,
}; // Orange
pub const PINK: Color = Color {
    r: 255,
    g: 109,
    b: 194,
    a: 255,
}; // Pink
pub const RED: Color = Color {
    r: 230,
    g: 41,
    b: 55,
    a: 255,
}; // Red
pub const MAROON: Color = Color {
    r: 190,
    g: 33,
    b: 55,
    a: 255,
}; // Maroon
pub const GREEN: Color = Color {
    r: 0,
    g: 228,
    b: 48,
    a: 255,
}; // Green
pub const LIME: Color = Color {
    r: 0,
    g: 158,
    b: 47,
    a: 255,
}; // Lime
pub const DARKGREEN: Color = Color {
    r: 0,
    g: 117,
    b: 44,
    a: 255,
}; // Dark Green
pub const SKYBLUE: Color = Color {
    r: 102,
    g: 191,
    b: 255,
    a: 255,
}; // Sky Blue
pub const BLUE: Color = Color {
    r: 0,
    g: 121,
    b: 241,
    a: 255,
}; // Blue
pub const DARKBLUE: Color = Color {
    r: 0,
    g: 82,
    b: 172,
    a: 255,
}; // Dark Blue
pub const PURPLE: Color = Color {
    r: 200,
    g: 122,
    b: 255,
    a: 255,
}; // Purple
pub const VIOLET: Color = Color {
    r: 135,
    g: 60,
    b: 190,
    a: 255,
}; // Violet
pub const DARKPURPLE: Color = Color {
    r: 112,
    g: 31,
    b: 126,
    a: 255,
}; // Dark Purple
pub const BEIGE: Color = Color {
    r: 211,
    g: 176,
    b: 131,
    a: 255,
}; // Beige
pub const BROWN: Color = Color {
    r: 127,
    g: 106,
    b: 79,
    a: 255,
}; // Brown
pub const DARKBROWN: Color = Color {
    r: 76,
    g: 63,
    b: 47,
    a: 255,
}; // Dark Brown
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
}; // White
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255,
}; // Black
pub const BLANK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
}; // Blank (Transparent)
pub const MAGENTA: Color = Color {
    r: 255,
    g: 0,
    b: 255,
    a: 255,
}; // Magenta
pub const RAYWHITE: Color = Color {
    r: 245,
    g: 245,
    b: 245,
    a: 255,
}; // My own White (raylib logo)
