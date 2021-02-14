#![allow(incomplete_features)]
#![feature(generic_associated_types)]
#![feature(never_type)]

pub mod const_data;
pub mod either;
pub mod eval;
pub mod ior;
pub mod non_empty_vec;
pub mod one_and;
pub mod option;
pub mod result;

pub mod prelude {
    pub use crate::const_data::*;
    pub use crate::either::*;
    pub use crate::eval::*;
    pub use crate::ior::*;
    pub use crate::non_empty_vec::*;
    pub use crate::one_and::*;
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
