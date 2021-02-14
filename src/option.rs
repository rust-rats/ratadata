pub mod syntax {
    trait OptionSyntax
    where
        Self: Sized,
    {
        fn some(self) -> Option<Self>;
    }

    impl<T> OptionSyntax for T {
        fn some(self) -> Option<Self> {
            Some(self)
        }
    }
}
