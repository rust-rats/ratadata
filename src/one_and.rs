use std::marker::PhantomData;

use rats::kernel::Kind;

pub struct OneAnd<K: Kind, T> {
    head: T,
    tail: K::Ty<T>,
}

impl<K: Kind, T> OneAnd<K, T> {
    pub fn of(head: T, tail: K::Ty<T>) -> Self {
        OneAnd { head, tail }
    }
}

pub struct OneAndKind<K: Kind>(PhantomData<K>);
impl<K: Kind> Kind for OneAndKind<K> {
    type Ty<T> = OneAnd<K, T>;
}
