use crate::base::hkt::Hkt1;

pub trait Io: Sized {
    type Output;

    fn run(self) -> Self::Output;

    fn map<B, F>(self, f: F) -> IoMap<Self, F>
    where
        F: FnOnce(Self::Output) -> B,
    {
        IoMap { io: self, f }
    }

    fn pure<T>(t: T) -> IoPure<T> {
        IoPure { io: t }
    }

    fn and_then<B, F>(self, f: F) -> IoBind<Self, F>
    where
        F: FnOnce(Self::Output) -> B,
    {
        IoBind { io: self, f }
    }

    fn then<Mb>(self, mb: Mb) -> IoBind<Self, impl FnOnce(Self::Output) -> Mb>
    where
        Mb: Io,
    {
        IoBind {
            io: self,
            f: |_| mb,
        }
    }
}

pub struct IoMap<I, F> {
    io: I,
    f: F,
}

impl<A, F, B> Io for IoMap<A, F>
where
    A: Io,
    F: FnOnce(A::Output) -> B,
{
    type Output = B;

    fn run(self) -> Self::Output {
        (self.f)(self.io.run())
    }
}

// impl<A, B, F> Hkt1 for IoMap<A, F>
// where
//     A: Io,
//     F: FnOnce(A::Output) -> B,
// {
//     type HktArg1 = B;
// }

// impl<A, F, B> Functor for IoMap<A, F>
// where
//     A: Io,
//     F: FnOnce(A::Output) -> B,
// {
//     type Map<T, G> = IoMap<Self, G>
//     where
//         G: Fn(Self::HktArg1) -> T + Clone;

//     fn fmap<C, G>(self, g: G) -> Self::Map<C, G>
//     where
//         G: Fn(Self::HktArg1) -> C + Clone,
//     {
//         IoMap { io: self, f: g }
//     }

//     fn fmap1<G>(self, g: G) -> Self::Map<Self::HktArg1, G>
//     where
//         G: Fn(Self::HktArg1) -> Self::HktArg1 + Clone,
//     {
//         IoMap { io: self, f: g }
//     }
// }

// impl<A, F, B> Monad for IoMap<A, F>
// where
//     A: Io,
//     F: Fn(A::Output) -> B,
//     // Mb: Monad<HktArg1 = B> + Io<Output = B>,
// {
//     type Pure<T> = IoPure<T>;

//     type M<B_> = impl Monad<HktArg1 = B_> + Io<Output = B_>; // There is no way to know if the type exists

//     type Bind<B_, Mf_> = IoBind<Self, Mf_>
//     where
//         Mf_: Fn(Self::HktArg1) -> Self::M<B_> + Clone;

//     fn pure<T>(t: T) -> Self::Pure<T> {
//         IoPure { io: t }
//     }

//     fn bind<C, Mc, Mㅁf>(self, mf: Mf) -> Self::Bind<C, Mf>
//     where
//         // Mf: Fn(Self::HktArg1) -> Self::M<C> + Clone,
//         Mf: FnOnce(Self::HktArg1) -> Mc + Clone,
//         Mc: Monad<HktArg1 = C> + Io<Output = C>,
//     {
//         IoBind { io: self, f: mf }
//     }
// }

pub struct IoPure<A> {
    io: A,
}

impl<A> Io for IoPure<A> {
    type Output = A;

    fn run(self) -> Self::Output {
        self.io
    }
}

// impl<A> Hkt1 for IoPure<A> {
//     type HktArg1 = A;
// }

// impl<A> Functor for IoPure<A> {
//     type Map<T, F> = IoMap<Self, F>
//     where
//         F: Fn(Self::HktArg1) -> T + Clone;

//     fn fmap<B, F>(self, f: F) -> Self::Map<B, F>
//     where
//         F: Fn(Self::HktArg1) -> B + Clone,
//     {
//         IoMap { io: self, f }
//     }

//     fn fmap1<F>(self, f: F) -> Self::Map<Self::HktArg1, F>
//     where
//         F: Fn(Self::HktArg1) -> Self::HktArg1 + Clone,
//     {
//         IoMap { io: self, f }
//     }
// }

// impl<A> Monad for IoPure<A> {
//     type Pure<T> = IoPure<T>;

//     type Bind<B, F> = IoBind<Self, F>
//     where
//         F: FnOnce(Self::HktArg1) -> B + Clone,
//         B: Monad;

//     fn pure<T>(t: T) -> Self::Pure<T> {
//         IoPure { io: t }
//     }

//     fn bind<B, F>(self, f: F) -> Self::Bind<B, F>
//     where
//         F: Fn(Self::HktArg1) -> B + Clone,
//         B: Monad,
//     {
//         IoBind { io: self, f }
//     }
// }

pub struct IoBind<A, F> {
    io: A,
    f: F,
}

impl<A, B, F> Io for IoBind<A, F>
where
    A: Io,
    B: Io,
    F: FnOnce(A::Output) -> B,
{
    type Output = B::Output;

    fn run(self) -> Self::Output {
        (self.f)(self.io.run()).run()
    }
}

// impl<A, B, F> Hkt1 for IoBind<A, F>
// where
//     A: Io,
//     F: FnOnce(A::Output) -> B,
//     B: Io,
// {
//     type HktArg1 = <B as Io>::Output;
// }

// impl<A, B, F> Functor for IoBind<A, F>
// where
//     A: Io,
//     F: FnOnce(A::Output) -> B,
//     B: Io,
// {
//     type Map<T, G> = IoMap<Self, G>
//     where
//         G: Fn(Self::HktArg1) -> T + Clone;

//     fn fmap<C, G>(self, g: G) -> Self::Map<C, G>
//     where
//         G: Fn(Self::HktArg1) -> C + Clone,
//     {
//         IoMap { io: self, f: g }
//     }

//     fn fmap1<G>(self, g: G) -> Self::Map<Self::HktArg1, G>
//     where
//         G: Fn(Self::HktArg1) -> Self::HktArg1 + Clone,
//     {
//         IoMap { io: self, f: g }
//     }
// }

// impl<A, B, F> Monad for IoBind<A, F>
// where
//     A: Io,
//     F: FnOnce(A::Output) -> B,
//     B: Io,
// {
//     type Pure<T> = IoPure<T>;

//     type Bind<C, G> = IoBind<Self, G>
//     where
//         G: Fn(Self::HktArg1) -> C + Clone,
//         C: Monad + Io;

//     fn pure<T>(t: T) -> Self::Pure<T> {
//         IoPure { io: t }
//     }

//     fn bind<C, G>(self, g: G) -> Self::Bind<C, G>
//     where
//         G: Fn(Self::HktArg1) -> C + Clone,
//         C: Monad,
//     {
//         IoBind { io: self, f: g }
//     }
// }
