pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub mod syntax {
    use super::*;

    trait EitherSyntax
    where
        Self: Sized,
    {
        fn left(self) -> Either<Self, !>;
        fn right(self) -> Either<!, Self>;
    }

    impl<T> EitherSyntax for T {
        fn left(self) -> Either<T, !> {
            Either::Left(self)
        }

        fn right(self) -> Either<!, T> {
            Either::Right(self)
        }
    }
}
