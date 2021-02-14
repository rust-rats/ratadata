use rats::kernel::prelude::VecKind;

use super::one_and::*;

pub struct NonEmptyVec<T>(OneAnd<VecKind, T>);

impl<T> NonEmptyVec<T> {
    pub fn one(t: T) -> Self {
        NonEmptyVec(OneAnd::of(t, Vec::new()))
    }

    pub fn of(head: T, tail: Vec<T>) -> Self {
        NonEmptyVec(OneAnd::of(head, tail))
    }
}

impl<T> Into<OneAnd<VecKind, T>> for NonEmptyVec<T> {
    fn into(self) -> OneAnd<VecKind, T> {
        self.0
    }
}

pub mod syntax {
    use super::*;

    pub trait NelSyntax<T> {
        fn to_nel(self) -> Option<NonEmptyVec<T>>;
    }

    impl<T> NelSyntax<T> for Vec<T> {
        fn to_nel(mut self) -> Option<NonEmptyVec<T>> {
            if self.is_empty() {
                None
            } else {
                let head = self.remove(0);
                Some(NonEmptyVec(OneAnd::of(head, Vec::new())))
            }
        }
    }
}
