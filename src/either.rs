use std::marker::PhantomData;

use rats::kernel::Kind;

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub struct LeftEitherKind<B>(PhantomData<B>);

impl<B> Kind for LeftEitherKind<B> {
    type Ty<T> = Either<T, B>;
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
