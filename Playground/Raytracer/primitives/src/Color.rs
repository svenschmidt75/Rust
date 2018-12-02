#[derive(Copy, Clone)]

pub struct Color {
	pub r: f64,
	pub g: f64,
	pub b: f64,
}

impl Color {
	pub fn new(r: f64, g: f64, b: f64) -> Self {
		Color{r, g, b }
	}
}
