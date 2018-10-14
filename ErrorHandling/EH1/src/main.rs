fn find(haystack: &str, needle: char) -> Option<usize> {
	for (offset, c) in haystack.char_indices() {
		if c == needle{
			return Some(offset);
		}
	}
	None
}


fn main() {
	let filename = "foobar.rs";
	match find(filename, '.') {
		Some(i) => println!("File extension: {}", &filename[i+1..]),
		None    => println!("File extension not found"),
	}
}
