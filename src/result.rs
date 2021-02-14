pub mod syntax {
    pub trait ResultSyntax: Sized {
        fn ok<E>(self) -> Result<Self, E> {
            Ok(self)
        }

        fn err<T>(self) -> Result<T, Self> {
            Err(self)
        }
    }

    impl<T> ResultSyntax for T {}
}
