use std::marker::PhantomData;

use crate::kinded::Kind;

pub enum Ior<A, B> {
    Left(A),
    Right(B),
    Both(A, B),
}

impl<A, B> Ior<A, B> {
    pub fn left(a: A) -> Self {
        Ior::<A, B>::Left(a)
    }

    pub fn right(b: B) -> Self {
        Ior::<A, B>::Right(b)
    }

    pub fn both(a: A, b: B) -> Self {
        Ior::<A, B>::Both(a, b)
    }
}

pub struct LeftIorKind<B>(PhantomData<B>);

impl<B> Kind for LeftIorKind<B> {
    type Cons<T> = Ior<T, B>;
}

pub mod syntax {
    use super::*;

    pub trait IorSyntax1: Sized {
        fn left_ior<R>(self) -> Ior<Self, R> {
            Ior::Left(self)
        }
        fn right_ior<L>(self) -> Ior<L, Self> {
            Ior::Right(self)
        }
    }

    pub trait IorSyntax2<A, B>: Sized {
        fn both_ior(self) -> Ior<A, B>;
    }

    impl<T> IorSyntax1 for T {}

    impl<T1, T2> IorSyntax2<T1, T2> for (T1, T2) {
        fn both_ior(self) -> Ior<T1, T2> {
            Ior::Both(self.0, self.1)
        }
    }
}
