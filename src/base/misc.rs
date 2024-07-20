#[inline(always)]
pub fn in_place<A, F>(f: F) -> impl Fn(&mut A) -> ()
where
    F: Fn(A) -> A,
{
    move |value: &mut A| unsafe {
        std::ptr::write(value, f(std::ptr::read(value)));
    }
}
