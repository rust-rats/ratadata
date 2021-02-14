use std::{cell::RefCell, mem};

use crate::prelude::Either;

pub struct Lazy<T>(RefCell<Either<T, Option<Box<dyn FnOnce() -> T>>>>);

impl<T> Lazy<T> {
    pub fn new(f: impl Fn() -> T + 'static) -> Self {
        Lazy(RefCell::new(Either::Right(Some(Box::new(f)))))
    }
}

impl<T: 'static> Lazy<T> {
    pub fn from_const(t: T) -> Self {
        Lazy(RefCell::new(Either::Right(Some(Box::new(|| t)))))
    }
}

impl<T: Clone> Lazy<T> {
    pub fn eval(&self) -> T {
        self.force();
        let borrow = &*self.0.borrow();
        // because of force this can't fail
        borrow.try_left().cloned().unwrap()
    }

    fn force(&self) {
        let mut content = self.0.borrow_mut();
        if content.is_right() {
            let ptr = content.try_right_mut().unwrap();
            let interior = ptr.take();
            let eval = interior.unwrap()();
            *content = Either::Left(eval);
        }
    }
}
