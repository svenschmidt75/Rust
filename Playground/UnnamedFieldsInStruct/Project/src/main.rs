pub struct Result(Vec<u32>);

/*
	equivalent to

	pub struct Result {
		result: Vec<u32>
	}

	but the field 'result' is not named.

*/

impl Result {
	pub fn iter(&self) -> std::slice::Iter<u32> {
		// the unnamed field is accessed via 0
		self.0.iter()
	}
}


fn main() {
	let v = Result(vec![0, 1, 2]);
	for x in v.iter() {
		println!("{}", x);
	}

    println!("Hello, world!");
}
