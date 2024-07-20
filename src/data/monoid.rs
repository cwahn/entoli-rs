pub trait Monoid {
    type A;

    fn mempty() -> Self::A;

    fn mappend(lhs: Self::A, rhs: Self::A) -> Self::A;
}
