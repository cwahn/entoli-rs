#[inline(always)]
pub fn in_place<A, F>(f: F) -> impl Fn(&mut A) -> ()
where
    F: Fn(A) -> A,
{
    move |x: &mut A| unsafe {
        std::ptr::write(x, f(std::ptr::read(x)));
    }
}
