use std::ops::{Mul, MulAssign, Add, AddAssign};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
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

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        Color::new((self.r as f32 * rhs) as u8, (self.g as f32 * rhs) as u8, (self.b as f32 * rhs) as u8)
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r = (self.r as f32 * rhs) as u8;
        self.g = (self.g as f32 * rhs) as u8;
        self.b = (self.b as f32 * rhs) as u8;
    }
}