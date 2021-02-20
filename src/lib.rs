#![allow(incomplete_features)]
#![feature(generic_associated_types)]

pub mod const_data;
pub mod either;
pub mod eval;
pub mod id;
pub mod ior;
pub mod lazy;
pub mod non_empty_vec;
pub mod one_and;
pub mod option;
pub mod result;
pub mod validated;

pub mod kinded {
    use rats::core::std_kinds;

    pub trait Kind {
        type Cons<T>;
    }

    impl Kind for std_kinds::VecKind {
        type Cons<T> = Vec<T>;
    }

    impl Kind for std_kinds::OptionKind {
        type Cons<T> = Option<T>;
    }

    impl<E> Kind for std_kinds::ResultKindOk<E> {
        type Cons<T> = Result<T, E>;
    }
}

pub mod prelude {
    pub use crate::const_data::*;
    pub use crate::either::*;
    pub use crate::eval::*;
    pub use crate::id::*;
    pub use crate::ior::*;
    pub use crate::non_empty_vec::*;
    pub use crate::one_and::*;
    pub use crate::validated::*;
}

pub mod syntaxes {
    pub use crate::const_data::syntax::*;
    pub use crate::either::syntax::*;
    pub use crate::ior::syntax::*;
    pub use crate::non_empty_vec::syntax::*;
    pub use crate::option::syntax::*;
    pub use crate::result::syntax::*;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
