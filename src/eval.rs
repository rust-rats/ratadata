use crate::lazy::Lazy;

use super::either::*;
use std::cell::RefCell;

pub enum Eval<T> {
    Lazy(Lazy<T>),
    Always(Box<dyn Fn() -> T>),
    Now(T),
}

impl<T: Clone> Eval<T> {
    pub fn now(f: impl Fn() -> T) -> Self {
        Eval::Now(f())
    }

    pub fn lazy(f: impl Fn() -> T + 'static) -> Self {
        Eval::Lazy(Lazy::new(f))
    }

    pub fn always(f: impl Fn() -> T + 'static) -> Self {
        Eval::Always(Box::new(f))
    }

    pub fn eval(&self) -> T {
        match self {
            Eval::Now(v) => v.clone(),
            Eval::Always(f) => f(),
            Eval::Lazy(lazy) => lazy.eval(),
        }
    }
}
