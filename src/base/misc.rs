/// Return in-place mutating function from a homogeneous consuming function.
///
/// (a -> a) -> (&mut a -> ())
/// Input function should not panic, otherwise UB.

#[inline(always)]
pub fn in_place<A, F>(f: F) -> impl Fn(&mut A) -> ()
where
    F: Fn(A) -> A,
{
    move |value: &mut A| unsafe {
        *value = f(std::ptr::read(value));
    }
}
