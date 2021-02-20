use std::ops::{Deref, DerefMut};

use rats::core::{
    applicative::{ApplicativeInstance, ApplicativeTy},
    prelude::{
        ApplyInstance, ApplyTy, FlatMapInstance, FlatMapTy, FunctorInstance, FunctorTy,
        MonadInstance, MonadTy,
    },
};

use crate::kinded::Kind;

#[derive(Clone, Debug, PartialEq)]
pub struct Id<T>(pub T);

impl<T> Deref for Id<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Id<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Id<T> {
    #[inline]
    pub fn into_value(self) -> T {
        self.0
    }
}

impl<T> From<T> for Id<T> {
    fn from(value: T) -> Self {
        Id(value)
    }
}

pub struct IdKind;

impl Kind for IdKind {
    type Cons<T> = Id<T>;
}

impl FunctorTy for IdKind {
    type Cons<T> = Id<T>;
}

impl<T> FunctorInstance<T> for Id<T> {
    type Kind = IdKind;

    fn fmap<B>(self, f: impl Fn(&T) -> B) -> Id<B> {
        Id(f(&self))
    }
}

impl ApplyTy for IdKind {
    type Cons<T> = Id<T>;
}

impl<T> ApplyInstance<T> for Id<T> {
    type Kind = IdKind;

    fn apply<B, F>(self, f: Id<F>) -> Id<B>
    where
        F: Fn(&T) -> B,
    {
        Id(f.0(&self))
    }
}

impl ApplicativeTy for IdKind {
    type Cons<T> = Id<T>;
}

impl<T> ApplicativeInstance<T> for Id<T> {
    type Kind = IdKind;

    fn pure<A>(value: A) -> Id<A> {
        Id(value)
    }
}

impl FlatMapTy for IdKind {
    type Cons<T> = Id<T>;
}

impl<T> FlatMapInstance<T> for Id<T> {
    type Kind = IdKind;

    fn flat_map<B>(self, f: impl Fn(&T) -> Id<B>) -> Id<B> {
        f(&self)
    }
}

impl MonadTy for IdKind {
    type Cons<T> = Id<T>;
}

impl<T> MonadInstance<T> for Id<T> {
    type Kind = IdKind;
}

#[cfg(test)]
impl<T: quickcheck::Arbitrary + Clone> quickcheck::Arbitrary for Id<T> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Id(T::arbitrary(g))
    }
}
