use crate::{hkt::Hkt1, impl_hkt1};

pub trait Functor: Hkt1 {
    fn fmap<B, F>(self, f: &F) -> Self::Of<B>
    where
        F: Fn(Self::HktOf1) -> B;
}

// pub trait FunctorMut: Functor {
//     fn fmap_mut<B, F>(self, f: &mut F) -> Self::Of<B>
//     where
//         F: for<'a> FnMut(&mut Self::HktOf1) -> B;
// }

pub trait FunctorOnce: Functor {
    fn fmap_once<B, F>(self, f: F) -> Self::Of<B>
    where
        F: FnOnce(Self::HktOf1) -> B;
}

impl_hkt1!(Option);

impl<T> Functor for Option<T> {
    fn fmap<B, F>(self, f: &F) -> Self::Of<B>
    where
        F: Fn(T) -> B,
    {
        match self {
            Some(x) => Some(f(x)),
            None => None,
        }
    }
}

// impl<T> FunctorMut for Option<T> {
//     fn fmap_mut<B, F>(self, f: &mut F) -> Self::Of<B>
//     where
//         F: for<'a> FnMut(&mut T) -> B,
//     {
//         match self {
//             Some(mut x) => Some(f(&mut x)),
//             None => None,
//         }
//     }
// }

impl<T> FunctorOnce for Option<T> {
    fn fmap_once<B, F>(self, f: F) -> Self::Of<B>
    where
        F: FnOnce(T) -> B,
    {
        match self {
            Some(x) => Some(f(x)),
            None => None,
        }
    }
}

// impl<T> Hkt1 for Iterator
// where
//     T: Iterator,
// {
//     type HktOf1 = T;

//     type With<W1> = Self<W1>;
// }

// impl<T> Hkt1 for Iterator<Item = T> {
//     type HktOf1 = T;

//     type With<W1> = Iterator<Item = W1>;
// }

// impl_hkt1!(Map);

// impl<T, F> Hkt1 for Map<T, F> {
//     type HktOf1 = T;
//     type With<W1> = Map<W1, F>;
// }

// impl<T, F> Functor for Map<T, F> {
//     fn fmap<B, G>(&self, g: G) -> Self::With<B>
//     where
//         G: for<'a> Fn(&'a Self::HktOf1) -> B,
//     {

//     }
// }

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_option_functor() {
        let x = Some(1);
        let f = |x: i32| x + 1;
        let y = x.fmap(&f);
        assert_eq!(y, Some(2));
    }

    // #[test]
    // fn test_option_functor_mut() {
    //     let x = Some(1);
    //     let mut f = |x: &mut i32| {
    //         *x += 1;
    //         *x
    //     };
    //     let y = x.fmap_mut(&mut f);
    //     assert_eq!(y, Some(2));
    // }

    #[test]
    fn test_option_functor_once() {
        let x = Some(1);
        let f = |x: i32| x + 1;
        let y = x.fmap_once(f);
        assert_eq!(y, Some(2));
    }
}
