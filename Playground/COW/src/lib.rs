use std::ops::Deref;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Error;

#[allow(dead_code)]
enum COW<'a> {
    Borrowed(&'a str),
    Owned(String)
}

#[allow(dead_code)]
impl<'a> COW<'a> {

    pub fn to_mut(&mut self) -> &mut String {
        match *self {
            COW::Borrowed(borrowed) => {
                *self = COW::Owned(borrowed.to_string());
                match *self {
                    COW::Owned(ref mut owned) => {
                        owned
                    },
                    COW::Borrowed(..) => unreachable!()
                }
            },
            COW::Owned(ref mut owned) => {
                owned
            }
        }
    }

}

impl<'a> Deref for COW<'a> {
    type Target = str;

    fn deref(&self) -> &<Self as Deref>::Target {
        match *self {
            COW::Borrowed(ref val) => {
                val
            },
            COW::Owned(ref val) => {
                val
            }
        }
    }
}

impl<'a> PartialEq<COW<'a>> for &str {

    fn eq(&self, other: &COW<'a>) -> bool {
        match *other {
            COW::Borrowed(ref val) => {
                self == val
            },
            COW::Owned(ref val) => {
                self == val
            }
        }
    }
}

impl <'a> Debug for COW<'a> {

    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            COW::Borrowed(ref val) => {
                write!(f, "{}", val)
            },
            COW::Owned(ref val) => {
                write!(f, "{}", val)
            }
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Arrange

        // Act
        let cow = COW::Borrowed(&"test");

        // Assert
    }

    #[test]
    fn deref_borrowed() {
        // Arrange

        let mut cow = COW::Borrowed("test");                 // ---------+-- 'a
        {                                                    //          |
            let mut owned_cow: &mut String = cow.to_mut();   // -+-- 'b  |
                                                             //  |       |
            // Act                                           //  |       |
            String::push(&mut owned_cow, 'c');  //  |       |
        }                                                    // -|       |
                                                             //         |
        // Assert                                            //         |
        assert_eq!("testc", cow);                            //         |
    }                                                        // --------+

    #[test]
    fn deref_to_mut_borrowed3() {
        use std::borrow::Cow;

        // Arrange
        let value = 1;
        let mut cow: Cow<i32> = Cow::Borrowed(&value);
        let mut owned_cow: &mut i32 = cow.to_mut();

        // Act
        owned_cow = &mut 2;

        // Assert
        assert_eq!(2, value);
    }

    #[test]
    fn deref_to_mut_borrowed2() {
        use std::borrow::Cow;

        // Arrange
        let value = "test".to_string();
        let mut cow: Cow<str> = Cow::from(&value);
        let mut owned_cow: &mut String = cow.to_mut();

        // Act
//        owned_cow.push('2');
        String::push(&mut owned_cow, 'c');

        // Assert
        assert_eq!("testc", cow);
    }

}
