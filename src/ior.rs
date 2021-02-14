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

pub mod syntax {
    use super::*;

    pub trait IorSyntax1
    where
        Self: Sized,
    {
        fn left_ior(self) -> Ior<Self, !>;
        fn right_ior(self) -> Ior<!, Self>;
    }

    pub trait IorSyntax2<A, B>
    where
        Self: Sized,
    {
        fn both_ior(self) -> Ior<A, B>;
    }

    impl<T> IorSyntax1 for T {
        fn left_ior(self) -> Ior<T, !> {
            Ior::Left(self)
        }

        fn right_ior(self) -> Ior<!, T> {
            Ior::Right(self)
        }
    }

    impl<T1, T2> IorSyntax2<T1, T2> for (T1, T2) {
        fn both_ior(self) -> Ior<T1, T2> {
            Ior::Both(self.0, self.1)
        }
    }
}
