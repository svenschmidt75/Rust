use std::ops::Deref;

#[allow(dead_code)]
enum COW<'a, T: ?Sized> {
    Borrowed(&'a T),
    Owned(T)
}

#[allow(dead_code)]
impl<'a, T: ?Sized> COW<'a, T> {

    pub fn to_mut(&'a mut self) -> &'a mut T {
        match *self {
            COW::Borrowed(borrowed) => {
                *self = COW::Owned(borrowed.clone());
                match *self {
                    COW::Owned(owned) => {
                        let a: u8 = owned;
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

impl<'a, T: ?Sized + 'a> Deref for COW<'a, T> {
    type Target = T;

    fn deref(&self) -> &<Self as Deref>::Target {
        match self {
            COW::Borrowed(val) => {
                val
            },
            COW::Owned(val) => {
                val
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

        // Act
        let cow = COW::Borrowed(&"test");

        // Assert
        assert_eq!("test", *cow);
    }

    #[test]
    fn deref_to_mut_borrowed() {
        // Arrange
        let value = 1;
        let mut cow: COW<i32> = COW::Borrowed(&value);
        let mut owned_cow = cow.to_mut();

        // Act
//        owned_cow.push('2');
//        String::push(&mut owned_cow, 'c');
        owned_cow = &mut &2;

        // Assert
        assert_eq!(1, value);
//        assert_eq!(&mut &2, owned_cow);
    }

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
        assert_eq!("test", value);
    }

}
