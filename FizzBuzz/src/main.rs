use std::fmt::{Display, Error, Formatter};

fn fizz_buzz(i: u32) -> String {
	let mut text = String::new();
	if i % 3 == 0 {
		text = String::from("fizz");
	}
	if i % 5 == 0 {
		text = text + &"buzz";
	}
	if i % 3 != 0 && i % 5 != 0 {
		text = i.to_string();
	}
	text
}

fn fizz_buzz2(i: u32) -> String {
	match i {
		i if i % 3 == 0 && i % 5 == 0 => String::from("fizz buzz"),
		i if i % 3 == 0               => String::from("fizz"),
		i if i % 5 == 0               => String::from("buzz"),
		i                             => i.to_string(),
	}
}



enum FizzBuzzValue {
	Value(u32),
	Fizz(u32),
	Buzz(u32),
	FizzBuzz(u32),
}

impl Display for FizzBuzzValue {
	fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
		match self {
			&FizzBuzzValue::FizzBuzz(_) => write!(f, "FizzBuzz"),
			&FizzBuzzValue::Fizz(_)     => write!(f, "Fizz"),
			&FizzBuzzValue::Buzz(_)     => write!(f, "Buzz"),
			&FizzBuzzValue::Value(n)    => write!(f, "{}", n),
		}
	}
}

fn to_fzbz(n: u32) -> FizzBuzzValue {
	match n {
		n if n % 15 == 0 => FizzBuzzValue::FizzBuzz(n),
		n if n % 3 == 0  => FizzBuzzValue::Fizz(n),
		n if n % 5 == 0  => FizzBuzzValue::Buzz(n),
		n                => FizzBuzzValue::Value(n),
	}
}

fn main() {
	for i in 1..101 {
		println!("{} {} {}", fizz_buzz(i), fizz_buzz2(i), to_fzbz(i));
	}
}
