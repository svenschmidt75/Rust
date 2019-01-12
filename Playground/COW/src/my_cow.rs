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

#[allow(dead_code)]
enum MyCow<'a, T: MyToOwned> {
    Borrowed(&'a T),
    Owned(T::Owned)
}

impl<'a, T: MyToOwned> MyToOwned for MyCow<'a, T> {
    type Owned = T::Owned;

    fn to_owned(&self) -> <Self as MyToOwned>::Owned {
        MyToOwned::to_owned(self)
    }
}


impl <'a, T: MyToOwned + Clone> MyCow<'a, T> {

    pub fn to_mut(&mut self) -> &mut <Self as MyToOwned>::Owned {
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
    fn it_works() {
        // Arrange

        // Act
//        let _ = COW<>::Borrowed(&"test");

        // Assert
    }
}
