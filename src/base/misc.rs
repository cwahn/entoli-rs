#[inline(always)]
pub fn in_place<A, F>(f: F) -> impl Fn(&mut A) -> ()
where
    F: Fn(A) -> A,
{
    // ! Make shure f does not panic
    move |value: &mut A| unsafe {
        *value = f(std::ptr::read(value));
    }
}
