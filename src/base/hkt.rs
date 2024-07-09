use chrono::format::Item;

pub trait Hkt1 {
    type HktOf1;

    type Of<W1>: Hkt1<HktOf1 = W1> + Hkt1<Of<Self::HktOf1> = Self> + Hkt1<Of<W1> = Self::Of<W1>>;
}

pub trait Hkt2 {
    type HktOf1;
    type HktOf2;

    type Of<W1, W2>: Hkt2<HktOf1 = W1, HktOf2 = W2>
        + Hkt2<Of<Self::HktOf1, Self::HktOf2> = Self>
        + Hkt2<Of<W1, W2> = Self::Of<W1, W2>>;
}

#[macro_export]
macro_rules! impl_hkt1 {
    ($type_constructor:ident) => {
        impl<T1> Hkt1 for $type_constructor<T1> {
            type HktOf1 = T1;

            type Of<W1> = $type_constructor<W1>;
        }
    };
}

#[macro_export]
macro_rules! impl_hkt2 {
    ($type_constructor:ident) => {
        impl<T1, T2> Hkt2 for $type_constructor<T1, T2> {
            type HktOf1 = T1;
            type HktOf2 = T2;

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
//     type HktOf1 = T::Item;

//     type Of<W1> = impl Iterator<Item = W1>;
// }
