use std::ops::{Mul, MulAssign, Add, AddAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r: r as u32, g: g as u32, b: b as u32 }
    }
}

impl From<Color> for image::Rgb<u8> {
    fn from(color: Color) -> Self {
        image::Rgb([color.r as u8, color.g as u8, color.b as u8])
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, rhs: Color) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color::new((self.r as f64 * rhs) as u8, (self.g as f64 * rhs) as u8, (self.b as f64 * rhs) as u8)
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        self.r = (self.r as f64 * rhs) as u32;
        self.g = (self.g as f64 * rhs) as u32;
        self.b = (self.b as f64 * rhs) as u32;
    }
}