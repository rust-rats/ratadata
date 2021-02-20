use crate::prelude::NonEmptyVec;

pub enum Validated<S, E> {
    Valid(S),
    Invalid(E),
}

impl<S, E> Validated<S, E> {
    pub fn invalid(err: E) -> Validated<S, E> {
        Validated::Invalid(err)
    }

    pub fn invalid_nev(err: E) -> Validated<S, NonEmptyVec<E>> {
        Validated::Invalid(NonEmptyVec::one(err))
    }

    pub fn valid(val: S) -> Validated<S, E> {
        Validated::Valid(val)
    }

    pub fn validated_nev(val: S) -> Validated<S, NonEmptyVec<E>> {
        Validated::Valid(val)
    }

    pub fn cond(
        condition: bool,
        success: impl FnOnce() -> S,
        failure: impl FnOnce() -> E,
    ) -> Validated<S, E> {
        if condition {
            Validated::Valid(success())
        } else {
            Validated::Invalid(failure())
        }
    }

    pub fn cond_nev(
        condition: bool,
        success: impl FnOnce() -> S,
        failure: impl FnOnce() -> NonEmptyVec<E>,
    ) -> Validated<S, NonEmptyVec<E>> {
        if condition {
            Validated::Valid(success())
        } else {
            Validated::Invalid(failure())
        }
    }
}
