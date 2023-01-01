pub mod grids;
pub mod indices;

pub mod iterators {
    pub trait IteratorExt: Iterator {
        fn take_while_and_one<P>(self, f: P) -> TakeWhileAndOne<Self, P>
        where
            Self: Sized,
            P: FnMut(&Self::Item) -> bool,
        {
            TakeWhileAndOne {
                iter: self,
                flag: false,
                predicate: f,
            }
        }
    }
    impl<T> IteratorExt for T where T: Iterator {}

    #[derive(Debug, Clone)]
    pub struct TakeWhileAndOne<I, P> {
        iter: I,
        flag: bool,
        predicate: P,
    }

    impl<I: Iterator, P> Iterator for TakeWhileAndOne<I, P>
    where
        P: FnMut(&I::Item) -> bool,
    {
        type Item = I::Item;

        fn next(&mut self) -> Option<I::Item> {
            if self.flag {
                None
            } else {
                let x = self.iter.next()?;
                self.flag = !(self.predicate)(&x);
                Some(x)
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn take_while_and_one_return_one_more() {
            let a = [2, 6, 8, 9, 10];
            let mut b = a.into_iter().take_while_and_one(|i| i % 2 == 0);
            assert_eq!(b.next(), Some(2));
            assert_eq!(b.next(), Some(6));
            assert_eq!(b.next(), Some(8));
            assert_eq!(b.next(), Some(9));
            assert_eq!(b.next(), None);
        }

        #[test]
        fn take_while_and_one_return_no_more() {
            let a = [2, 6, 8];
            let mut b = a.into_iter().take_while_and_one(|i| i % 2 == 0);
            assert_eq!(b.next(), Some(2));
            assert_eq!(b.next(), Some(6));
            assert_eq!(b.next(), Some(8));
            assert_eq!(b.next(), None);
        }

        #[test]
        fn take_while_and_one_return_only_more() {
            let a = [1, 4];
            let mut b = a.into_iter().take_while_and_one(|i| i % 2 == 0);
            assert_eq!(b.next(), Some(1));
            assert_eq!(b.next(), None);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TestCase {
    pub input: &'static str,
    pub out1: &'static str,
    pub out2: &'static str,
}

impl From<[&'static str; 3]> for TestCase {
    fn from(value: [&'static str; 3]) -> Self {
        Self::from(&value)
    }
}

impl From<&[&'static str; 3]> for TestCase {
    fn from(value: &[&'static str; 3]) -> Self {
        Self {
            input: value[0],
            out1: value[1],
            out2: value[2],
        }
    }
}
