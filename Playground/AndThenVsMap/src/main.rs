use std::io;


// What is the difference between Map and AndThen on the Option type?
// The difference is that map cannot return  Option, whereas end_then
// will. Hence, we do not wrap what and_then returns in an option,
// as happens in map.

fn main() {
    works();
    fails();
}

fn works() {
    let result = try_read_int().and_then(|value1| {
        try_read_int().map(|value2| {
            value1 + value2 
        })
    });

    // prints Some(value1 + value2)
    println!("{:?}", result)
}

fn fails() {
    let result = try_read_int().map(|value1| {
        try_read_int().map(|value2| {
            value1 + value2
        })
    });

    // prints Some(Some(value1 + value2))
    println!("{:?}", result)
}

fn try_read_int() -> Option<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok();
    input.trim().parse().ok()
}
