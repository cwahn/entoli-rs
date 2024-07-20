pub trait Hkt1 {
    type HktArg1;
}

pub trait Hkt2 {
    type HktArg1;
    type HktArg2;
}

#[macro_export]
macro_rules! impl_hkt1 {
    ($type_constructor:ident) => {
        impl<T1> Hkt1 for $type_constructor<T1> {
            type HktArg1 = T1;
        }
    };
}

#[macro_export]
macro_rules! impl_hkt2 {
    ($type_constructor:ident) => {
        impl<T1, T2> Hkt2 for $type_constructor<T1, T2> {
            type HktArg1 = T1;
            type HktArg2 = T2;
        }
    };
}
