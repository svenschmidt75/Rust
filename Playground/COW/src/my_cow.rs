use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::Deref;

#[allow(dead_code)]
trait MyToOwned {
    type Owned;

    fn to_owned(&self) -> Self::Owned;
}

impl MyToOwned for str {
    type Owned = String;

    fn to_owned(&self) -> Self::Owned {
        ToOwned::to_owned(self)
    }
}

trait MyToBorrow {
    type Borrowed;

    fn borrow(&self) -> &Self::Borrowed;
}

#[allow(dead_code)]
enum MyCow<'a, T: ?Sized + MyToOwned> {
    Borrowed(&'a T),
    Owned(<T as MyToOwned>::Owned)
}

impl<'a, T: ?Sized + MyToOwned> MyCow<'a, T> {
    fn to_mut(&mut self) -> &mut <T as MyToOwned>::Owned {
        match *self {
            MyCow::Borrowed(borrowed) => {
                *self = MyCow::Owned(MyToOwned::to_owned(borrowed));
                match *self {
                    MyCow::Owned(ref mut owned) => {
                        owned
                    },
                    MyCow::Borrowed(..) => unreachable!()
                }
            },
            MyCow::Owned(ref mut owned) => {
                owned
            }
        }
    }
}

impl<'a, T> Deref for MyCow<'a, T>
    where T: MyToOwned + MyToBorrow<Borrowed=T>, <T as MyToOwned>::Owned: MyToBorrow<Borrowed=T>
{
    type Target = <T as MyToBorrow>::Borrowed;

    fn deref(&self) -> &<Self as Deref>::Target {
        match *self {
            MyCow::Borrowed(borrowed) => {
                borrowed
            },
            MyCow::Owned(ref owned) => {
                MyToBorrow::borrow(owned)
            }
        }
    }
}

impl<'a> PartialEq<MyCow<'a, str>> for &str {
    fn eq(&self, other: &MyCow<'a, str>) -> bool {
        match *other {
            MyCow::Borrowed(ref val) => {
                self == val
            },
            MyCow::Owned(ref val) => {
                self == val
            }
        }
    }
}

impl<'a, T> Debug for MyCow<'a, T>
    where T: ?Sized + MyToOwned + Display, <T as MyToOwned>::Owned: Display
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            MyCow::Borrowed(ref val) => {
                write!(f, "{}", val)
            },
            MyCow::Owned(ref val) => {
                write!(f, "{}", val)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_owned() {
        // Arrange
        let string_literal = "test";

        // Act
        let mut owned = MyToOwned::to_owned(string_literal);
        owned.push_str(" append");

        // Assert
        assert_eq!("test append", owned)
    }

    #[test]
    fn deref_borrowed() {
        // Arrange
        let mut cow = MyCow::Borrowed("test");
        let mut owned_cow: &mut String = cow.to_mut();

        // Act
        String::push(&mut owned_cow, 'c');

        // Assert
        assert_eq!("testc", cow);
    }
}
