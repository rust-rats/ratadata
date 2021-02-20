use crate::kinded::Kind;

#[derive(Clone, Debug, PartialEq)]
pub struct Id<T>(pub T);

impl<T> Id<T> {
    #[inline]
    pub fn into_value(self) -> T {
        self.0
    }
}

impl<T> From<T> for Id<T> {
    fn from(value: T) -> Self {
        Id(value)
    }
}

pub struct IdKind;

impl Kind for IdKind {
    type Cons<T> = T;
}

#[cfg(test)]
impl<T: quickcheck::Arbitrary + Clone> quickcheck::Arbitrary for Id<T> {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        Id(T::arbitrary(g))
    }
}
