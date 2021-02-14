use std::cell::RefCell;
use super::either::*;

pub enum Eval<T> {
    Lazy(RefCell<Either<T, Box<dyn Fn() -> T>>>),
    Always(Box<dyn Fn() -> T>),
    Now(T),
}

impl<T: Clone> Eval<T> {
    pub fn now(f: impl Fn() -> T) -> Self {
        Eval::Now(f())
    }

    pub fn lazy(f: impl Fn() -> T + 'static) -> Self {
        Eval::Lazy(RefCell::new(Either::Right(Box::new(f))))
    }

    pub fn always(f: impl Fn() -> T + 'static) -> Self {
        Eval::Always(Box::new(f))
    }

    pub fn eval(&self) -> T {
        match self {
            Eval::Now(v) => v.clone(),
            Eval::Always(f) => f(),
            Eval::Lazy(cell) => {
                let mut content = cell.borrow_mut();
                let result = match &*content {
                    Either::Left(val) => val.clone(),
                    Either::Right(f) => {
                        let tmp = f();
                        *&mut *content = Either::Left(tmp.clone());
                        tmp
                    }
                };
                result
            }
        }
    }
}
