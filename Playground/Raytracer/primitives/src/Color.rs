use std::ops::AddAssign;
use std::ops::DivAssign;
use std::ops::Mul;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        *self = Color { r: self.r + other.r, g: self.g + other.g, b: self.b + other.b }
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        *self = Color { r: self.r / rhs, g: self.g / rhs, b: self.b / rhs }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self * rhs.r, self * rhs.g, self * rhs.b)
    }
}