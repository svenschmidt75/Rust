
#[derive(Copy, Clone)]
pub struct Color {
	r: f64,
	g: f64,
	b: f64,
}

impl Color {
	pub fn new(r: f64, g: f64, b: f64) -> Self {
		Color{r: r, g: g, b: b}
	}
}