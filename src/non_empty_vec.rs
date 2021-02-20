use rats::core::prelude::VecKind;

use crate::kinded::Kind;

use super::one_and::*;

pub struct NonEmptyVec<T>(pub(crate) OneAnd<VecKind, T>);

impl<T> NonEmptyVec<T> {
    pub fn one(t: T) -> Self {
        NonEmptyVec(OneAnd::of(t, Vec::new()))
    }

    pub fn of(head: T, tail: Vec<T>) -> Self {
        NonEmptyVec(OneAnd::of(head, tail))
    }

    pub fn head(&self) -> &T {
        &self.0.head()
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct NonEmptyVecKind;

impl Kind for NonEmptyVecKind {
    type Cons<T> = NonEmptyVec<T>;
}

pub mod syntax {
    use super::*;

    pub trait NelSyntax<T> {
        fn into_nel(self) -> Option<NonEmptyVec<T>>;
    }

    impl<T> NelSyntax<T> for Vec<T> {
        fn into_nel(mut self) -> Option<NonEmptyVec<T>> {
            if self.is_empty() {
                None
            } else {
                let head = self.remove(0);
                Some(NonEmptyVec(OneAnd::of(head, Vec::new())))
            }
        }
    }
}
