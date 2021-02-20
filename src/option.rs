use crate::kinded::Kind;

pub struct OptionT<K: Kind, T>(K::Cons<Option<T>>);

impl<K: Kind, T> OptionT<K, T> {
    pub fn of(t: K::Cons<Option<T>>) -> Self {
        OptionT(t)
    }

    pub fn lift(t: K::Cons<T>) -> Self {
        todo!()
    }

    pub fn from_option(t: Option<T>) -> Self
// todo: where
        // K: ApplicativeK,
    {
        todo!()
    }
}

pub mod syntax {
    trait OptionSyntax: Sized {
        fn some(self) -> Option<Self>;
    }

    impl<T> OptionSyntax for T {
        fn some(self) -> Option<Self> {
            Some(self)
        }
    }
}
