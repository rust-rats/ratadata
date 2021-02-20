use std::marker::PhantomData;

use crate::kinded::Kind;

pub struct Const<A, B>(A, PhantomData<B>);

impl<A, B> Const<A, B> {
    fn get_value(&self) -> &A {
        &self.0
    }

    fn get_value_mut(&mut self) -> &mut A {
        &mut self.0
    }

    fn take(self) -> A {
        self.0
    }

    fn new(a: A) -> Self {
        Const(a, PhantomData)
    }
}

pub struct ConstKind<B>(PhantomData<B>);

impl<B> Kind for ConstKind<B> {
    type Cons<T> = Const<T, B>;
}

pub mod syntax {
    use super::*;

    pub trait ConstSyntax: Sized {
        fn to_const<B>(self) -> Const<Self, B> {
            Const::new(self)
        }
    }

    impl<T> ConstSyntax for T {}
}
