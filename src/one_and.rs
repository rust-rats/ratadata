use std::marker::PhantomData;

use rats::core::prelude::VecKind;

use crate::{kinded::Kind, prelude::NonEmptyVec};

pub struct OneAnd<K: Kind, T> {
    head: T,
    tail: K::Cons<T>,
}

impl<K: Kind, T> OneAnd<K, T> {
    pub fn of(head: T, tail: K::Cons<T>) -> Self {
        OneAnd { head, tail }
    }

    pub fn head(&self) -> &T {
        &self.head
    }
}

impl<T> From<NonEmptyVec<T>> for OneAnd<VecKind, T> {
    fn from(nev: NonEmptyVec<T>) -> Self {
        nev.0
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct OneAndKind<K: Kind>(PhantomData<K>);

impl<K: Kind> Kind for OneAndKind<K> {
    type Cons<T> = OneAnd<K, T>;
}
