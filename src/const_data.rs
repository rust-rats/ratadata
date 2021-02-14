use std::marker::PhantomData;

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

pub mod syntax {
    use super::*;

    pub trait ConstSyntax: Sized {
        fn to_const(self) -> Const<Self, !> {
            Const::new(self)
        }
    }

    impl<T> ConstSyntax for T {}
}
