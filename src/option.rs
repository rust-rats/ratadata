use rats::core::{
    applicative::{self, ApplicativeTy},
    functor,
    prelude::FunctorTy,
};

use crate::kinded::Kind;

pub struct OptionT<K: Kind, T>(K::Cons<Option<T>>);

impl<K: Kind, T> OptionT<K, T> {
    pub fn of(t: K::Cons<Option<T>>) -> Self {
        OptionT(t)
    }

    #[rustfmt::skip]
    pub fn lift<G: Kind>(kind: G, t: <G as FunctorTy>::Cons<T>) -> Self
    where
        G: FunctorTy<Cons<Option<T>> = K::Cons<Option<T>>>,
        T: Clone
    {
        OptionT(functor::fmap(kind, t, |t| Some(t.clone())))
    }

    #[rustfmt::skip]
    pub fn from_option<G: Kind>(kind: G, t: Option<T>) -> Self
    where
        G: ApplicativeTy<Cons<Option<T>> = K::Cons<Option<T>>>,
    {
         OptionT(applicative::pure(kind, t))
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
