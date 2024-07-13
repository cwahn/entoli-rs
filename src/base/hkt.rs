use chrono::format::Item;

pub trait Hkt1 {
    type HktArg1;

    // type Of<W1>: Hkt1<HktArg1 = W1> + Hkt1<Of<Self::HktArg1> = Self> + Hkt1<Of<W1> = Self::Of<W1>>;
}

pub trait Hkt2 {
    type HktArg1;
    type HktArg2;

    type Of<W1, W2>: Hkt2<HktArg1 = W1, HktArg2 = W2>
        + Hkt2<Of<Self::HktArg1, Self::HktArg2> = Self>
        + Hkt2<Of<W1, W2> = Self::Of<W1, W2>>;
}

#[macro_export]
macro_rules! impl_hkt1 {
    ($type_constructor:ident) => {
        impl<T1> Hkt1 for $type_constructor<T1> {
            type HktArg1 = T1;

            // type Of<W1> = $type_constructor<W1>;
        }
    };
}

#[macro_export]
macro_rules! impl_hkt2 {
    ($type_constructor:ident) => {
        impl<T1, T2> Hkt2 for $type_constructor<T1, T2> {
            type HktArg1 = T1;
            type HktArg2 = T2;

            type Of<W1, W2> = $type_constructor<W1, W2>;
        }
    };
}

// Blanket implementations

// Implement Hkt1 for Iterator

// ! Associated type position impl trait (ATPIT) is unstable
// impl<T> Hkt1 for T
// where
//     T: Iterator,
// {
//     type HktArg1 = T::Item;

//     type Of<W1> = impl Iterator<Item = W1>;
// }

pub trait HktIter: Iterator {}
// pub trait HktOption<A> {
//     fn bimap<B, F, G>(self, f: F, g: G) -> Option<B>
//     where
//         F: Fn(A) -> B,
//         G: Fn() -> B;
// }

// impl<A> HktOption<A> for Option<A> {
//     fn bimap<B, F, G>(self, f: F, g: G) -> Option<B>
//     where
//         F: Fn(A) -> B,
//         G: Fn() -> B,
//     {
//         match self {
//             Some(x) => Some(f(x)),
//             None => Some(g()),
//         }
//     }
// }
