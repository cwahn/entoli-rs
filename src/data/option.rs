// pub trait HktOption {
//     type Arg1;

//     fn fmap_or_else<B, D, F>(self, default: &D, f: &F) -> B
//     where
//         D: FnOnce() -> B,
//         F: FnOnce(Self::Arg1) -> B;
// }

// impl<A> HktOption for Option<A> {
//     type Arg1 = A;

//     #[inline(always)]
//     fn fmap_or_else<B, D, F>(self, default: D, f: F) -> B
//     where
//         D: FnOnce() -> B,
//         F: FnOnce(A) -> B,
//     {
//         self.map(f).unwrap_or_else(default)
//     }
// }


trait SomeTrait{
    type AccType : SomeTrait;
}

impl SomeTrait for i32{
    type AccType = impl SomeTrait<AccType = i32>;
}