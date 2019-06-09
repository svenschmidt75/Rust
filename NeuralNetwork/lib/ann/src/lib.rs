#[macro_use]
extern crate serde_derive;
extern crate bincode;

mod ann;
mod la;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
