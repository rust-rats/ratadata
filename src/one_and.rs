use std::marker::PhantomData;

use crate::kinded::Kind;

pub struct OneAnd<K: Kind, T> {
    head: T,
    tail: K::Cons<T>,
}

impl<K: Kind, T> OneAnd<K, T> {
    pub fn of(head: T, tail: K::Cons<T>) -> Self {
        OneAnd { head, tail }
    }
}

pub struct OneAndKind<K: Kind>(PhantomData<K>);
impl<K: Kind> Kind for OneAndKind<K> {
    type Cons<T> = OneAnd<K, T>;
}
