use std::marker::PhantomData;

use crate::kinded::Kind;

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

impl<A, B> Either<A, B> {
    pub fn try_left(&self) -> Option<&A> {
        match self {
            Either::Left(val) => Some(val),
            _ => None,
        }
    }

    pub fn try_right(&self) -> Option<&B> {
        match self {
            Either::Right(val) => Some(val),
            _ => None,
        }
    }

    pub fn try_left_mut(&mut self) -> Option<&mut A> {
        match self {
            Either::Left(val) => Some(val),
            _ => None,
        }
    }

    pub fn try_right_mut(&mut self) -> Option<&mut B> {
        match self {
            Either::Right(val) => Some(val),
            _ => None,
        }
    }

    pub fn is_left(&self) -> bool {
        matches!(self, Either::Left(_))
    }

    pub fn is_right(&self) -> bool {
        matches!(self, Either::Right(_))
    }
}

#[derive(Copy, Clone, Default, Debug)]
pub struct LeftEitherKind<B>(PhantomData<B>);

impl<B> Kind for LeftEitherKind<B> {
    type Cons<T> = Either<T, B>;
}

pub mod syntax {
    use super::*;

    trait EitherSyntax: Sized {
        fn left<R>(self) -> Either<Self, R> {
            Either::Left(self)
        }
        fn right<L>(self) -> Either<L, Self> {
            Either::Right(self)
        }
    }

    impl<T> EitherSyntax for T {}
}
