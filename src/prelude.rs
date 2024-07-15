use crate::{
    base::hkt::Hkt1,
    data::{
        functor::Functor,
        // monad::Monad
    },
    impl_hkt1,
};

// Tuples

/// O(1)
/// Extract the first element of a tuple
pub fn fst<A, B>((a, _): (A, B)) -> A {
    a
}

/// O(1)
/// Extract the second element of a tuple
pub fn snd<A, B>((_, b): (A, B)) -> B {
    b
}

// todo curry, uncurry

// pub fn curry<A, B, C>(f: impl Fn(A, B) -> C) -> impl Fn(A) -> impl Fn(B) -> C {
//     move |a| move |b| f(a, b)
// }

//  Folds and traversals

/// O(n) Lazy
///
/// Left-associative fold of a structure.
pub fn foldl<A, B, F>(f: F, acc: B, xs: impl IntoIterator<Item = A>) -> B
where
    F: Fn(B, A) -> B,
{
    xs.into_iter().fold(acc, move |acc, a| f(acc, a))
}

pub fn foldr<A, B, F>(
    f: F,
    acc: B,
    xs: impl IntoIterator<Item = A, IntoIter: DoubleEndedIterator<Item = A>>,
) -> B
where
    F: Fn(A, B) -> B,
{
    xs.into_iter().rfold(acc, move |acc, a| f(a, acc))
}

/// O(n)
/// Determines whether given element is in the iterable.
/// Short-circuits on first match.
pub fn elem<A>(x: A, xs: impl IntoIterator<Item = A>) -> bool
where
    A: PartialEq,
{
    xs.into_iter().any(|a| a == x)
}

// maximum, minimum, sum, product, any, all

/// O(n)
/// The largest element of a non-empty structure.
/// Returns None for empty structures.
pub fn maximum<A>(xs: impl IntoIterator<Item = A>) -> Option<A>
where
    A: Ord,
{
    xs.into_iter().max()
}

/// O(n)
/// The smallest element of a non-empty structure.
/// Returns None for empty structures.
pub fn minimum<A>(xs: impl IntoIterator<Item = A>) -> Option<A>
where
    A: Ord,
{
    xs.into_iter().min()
}

// todo Monid

/// O(n) Lazy
///
/// The sum function computes the sum of the numbers of a structure.
/// Returns 0 for empty structures.
pub fn sum<A>(xs: impl IntoIterator<Item = A>) -> A
where
    A: std::ops::Add<Output = A> + Default,
{
    xs.into_iter().fold(Default::default(), |acc, a| acc + a)
}

// todo Monid
// ! Need to implement unit for Monid with Mul
// pub fn product<A>(xs: impl IntoIterator<Item = A>) -> A
// where
//     A: std::ops::Mul<Output = A> + Default,
// {
//     xs.into_iter().fold(1 as A, |acc, a| acc * a)
// }

/// O(n)
/// Determines whether any element of the structure satisfies the predicate.
/// Short-circuits on first match.
pub fn any<A, F>(f: F, xs: impl IntoIterator<Item = A>) -> bool
where
    F: Fn(A) -> bool,
{
    xs.into_iter().any(|a| f(a))
}

/// O(n)
/// Determines whether all elements of the structure satisfy the predicate.
/// Short-circuits on first non-match.
pub fn all<A, F>(f: F, xs: impl IntoIterator<Item = A>) -> bool
where
    F: Fn(A) -> bool,
{
    xs.into_iter().all(|a| f(a))
}

// Miscellaneous functions

/// O(1)
/// The identity function.

#[inline(always)]
pub fn id<A>(a: A) -> A {
    a
}

// List operations

/// O(n) Lazy
///
/// Map a function over all values in the iterable.
#[inline(always)]
pub fn map<A, As, B, F>(f: F, xs: As) -> std::iter::Map<<As as IntoIterator>::IntoIter, F>
where
    As: IntoIterator<Item = A>,
    F: Fn(A) -> B,
{
    xs.into_iter().map(f)
}

// pub fn append<A>(
//     xs: impl IntoIterator<Item = A>,
//     ys: impl IntoIterator<Item = A>,
// ) -> impl IntoIterator<Item = A> {
//     xs.into_iter().chain(ys.into_iter())
// }

/// O(n) Lazy
///
/// Append two iterables.
/// Consumes both iterables.
#[inline(always)]
pub fn append<A, As1, As2>(
    xs: As1,
    ys: As2,
) -> std::iter::Chain<<As1 as IntoIterator>::IntoIter, <As2 as IntoIterator>::IntoIter>
where
    As1: IntoIterator<Item = A>,
    As2: IntoIterator<Item = A>,
{
    xs.into_iter().chain(ys.into_iter())
}

// #[inline(always)]
// pub fn filter<A, F>(f: F, xs: impl IntoIterator<Item = A>) -> impl Iterator<Item = A>
// where
//     F: Fn(&A) -> bool,
// {
//     xs.into_iter().filter(f)
// }

/// O(n) Lazy
///     
/// Filter elements of a structure based on a predicate.

#[inline(always)]
pub fn filter<A, As, F>(f: F, xs: As) -> std::iter::Filter<<As as IntoIterator>::IntoIter, F>
where
    As: IntoIterator<Item = A>,
    F: Fn(&A) -> bool,
{
    xs.into_iter().filter(f)
}

/// O(1)
///
/// Extract the first element of a iterable, if it exists.
/// Panics if the iterable is empty.
#[inline(always)]
pub fn head<A>(xs: impl IntoIterator<Item = A>) -> A {
    xs.into_iter().next().unwrap()
}

/// O(n)
///
/// Extract the last element of a iterable, if it exists consuming the iterable.
/// Panics if the iterable is empty.
#[inline(always)]
pub fn last<A>(xs: impl IntoIterator<Item = A>) -> A {
    xs.into_iter().last().unwrap()
}

/// O(1) Lazy
///     
/// Extract the elements after the head of a iterable, if it exists.
#[inline(always)]
pub fn tail<A, As>(xs: As) -> std::iter::Skip<<As as IntoIterator>::IntoIter>
where
    As: IntoIterator<Item = A>,
{
    xs.into_iter().skip(1)
}

/// O(n) Lazy
///
/// Extract the elements before the last element of a iterable, if it exists.
/// Consumes the iterable.
/// Panics if the iterable is empty.
#[inline(always)]
pub fn init<A, As>(xs: As) -> std::iter::FromFn<impl FnMut() -> Option<A>>
where
    As: IntoIterator<Item = A>,
{
    let mut iter = xs.into_iter().peekable();

    std::iter::from_fn(move || {
        let next = iter.next();
        if iter.peek().is_some() {
            next
        } else {
            None
        }
    })
}

#[inline(always)]
pub fn nth<A>(n: usize, xs: impl IntoIterator<Item = A>) -> Option<A> {
    xs.into_iter().nth(n)
}

#[inline(always)]
pub fn length<A>(xs: impl IntoIterator<Item = A>) -> usize {
    xs.into_iter().count()
}

#[inline(always)]
pub fn reverse<A>(xs: impl IntoIterator<Item = A, IntoIter: DoubleEndedIterator>) -> Vec<A> {
    xs.into_iter().rev().collect()
}

#[inline(always)]
pub fn concat<A, As>(xss: As) -> std::iter::Flatten<<As as IntoIterator>::IntoIter>
where
    As: IntoIterator,
    As::Item: IntoIterator<Item = A>,
{
    xss.into_iter().flatten()
}

#[inline(always)]
pub fn concat_map<A, As, Bs, F>(
    f: F,
    xs: As,
) -> std::iter::FlatMap<<As as IntoIterator>::IntoIter, Bs, F>
where
    As: IntoIterator<Item = A>,
    Bs: IntoIterator,
    F: Fn(A) -> Bs,
{
    xs.into_iter().flat_map(f)
}

// Building lists

pub fn scanl<A, B, F>(
    f: F,
    init: B,
    xs: impl IntoIterator<Item = A>,
) -> std::iter::FromFn<impl FnMut() -> Option<B>>
where
    F: Fn(&B, A) -> B,
    B: Clone,
{
    let mut acc = Some(init.clone());
    let mut iter = xs.into_iter();

    std::iter::from_fn(move || {
        if let Some(b) = acc.take() {
            acc = iter.next().map(|x| f(&b, x));
            Some(b)
        } else {
            None
        }
    })
}

pub fn scanl1<A, F>(
    f: F,
    xs: impl IntoIterator<Item = A>,
) -> std::iter::FromFn<impl FnMut() -> Option<A>>
where
    F: Fn(&A, A) -> A,
    A: Clone,
{
    let mut iter = xs.into_iter();
    let mut acc = iter.next();

    std::iter::from_fn(move || {
        if let Some(b) = acc.take() {
            acc = iter.next().map(|x| f(&b, x));
            Some(b)
        } else {
            None
        }
    })
}

// todo scanr, scanr1

// Infinite lists

//  iterate, repeat, replicate, cycle

pub fn iterate<A, F>(f: F, a: A) -> std::iter::FromFn<impl FnMut() -> Option<A>>
where
    F: Fn(&A) -> A,
    A: Clone,
{
    let mut acc = Some(a);

    std::iter::from_fn(move || {
        let a = acc.take()?;
        acc = Some(f(&a));
        Some(a)
    })
}

#[inline(always)]
pub fn repeat<A>(a: A) -> std::iter::FromFn<impl FnMut() -> Option<A>>
where
    A: Clone,
{
    std::iter::from_fn(move || Some(a.clone()))
}

pub fn replicate<A>(n: usize, a: A) -> std::iter::FromFn<impl FnMut() -> Option<A>>
where
    A: Clone,
{
    let mut i = 0;
    std::iter::from_fn(move || {
        if i < n {
            i += 1;
            Some(a.clone())
        } else {
            None
        }
    })
}

#[inline(always)]
pub fn cycle<As>(xs: As) -> std::iter::Cycle<<As as IntoIterator>::IntoIter>
where
    As: IntoIterator,
    <As as IntoIterator>::IntoIter: Clone,
{
    xs.into_iter().cycle()
}

// Sublists

// todo take, drop, split_at, take_while, drop_while, span

#[inline(always)]
pub fn take<A, As>(n: usize, xs: As) -> std::iter::Take<<As as IntoIterator>::IntoIter>
where
    As: IntoIterator<Item = A>,
{
    xs.into_iter().take(n)
}

#[inline(always)]
pub fn drop<A, As>(n: usize, xs: As) -> std::iter::Skip<<As as IntoIterator>::IntoIter>
where
    As: IntoIterator<Item = A>,
{
    xs.into_iter().skip(n)
}

#[inline(always)]
pub fn take_while<A, As, F>(f: F, xs: As) -> std::iter::TakeWhile<<As as IntoIterator>::IntoIter, F>
where
    As: IntoIterator<Item = A>,
    F: Fn(&A) -> bool,
{
    xs.into_iter().take_while(f)
}

#[inline(always)]
pub fn drop_while<A, As, F>(f: F, xs: As) -> std::iter::SkipWhile<<As as IntoIterator>::IntoIter, F>
where
    As: IntoIterator<Item = A>,
    F: Fn(&A) -> bool,
{
    xs.into_iter().skip_while(f)
}

#[inline(always)]
pub fn span<A, As, F>(
    f: F,
    xs: As,
) -> (
    std::iter::TakeWhile<<As as IntoIterator>::IntoIter, F>,
    std::iter::SkipWhile<<As as IntoIterator>::IntoIter, F>,
)
where
    As: IntoIterator<Item = A>,
    <As as IntoIterator>::IntoIter: Clone,
    F: Fn(&A) -> bool + Clone,
{
    let iter = xs.into_iter();
    (iter.clone().take_while(f.clone()), iter.skip_while(f))
}

#[inline(always)]
pub fn split_at<A, As>(
    n: usize,
    xs: As,
) -> (
    std::iter::Take<<As as IntoIterator>::IntoIter>,
    std::iter::Skip<<As as IntoIterator>::IntoIter>,
)
where
    As: IntoIterator<Item = A>,
    <As as IntoIterator>::IntoIter: Clone,
{
    let iter = xs.into_iter();
    (iter.clone().take(n), iter.skip(n))
}

// No break since it is a keyword

// Zipping and unzipping lists

#[inline(always)]
pub fn zip<A, B, As, Bs>(
    xs: As,
    ys: Bs,
) -> std::iter::Zip<<As as IntoIterator>::IntoIter, <Bs as IntoIterator>::IntoIter>
where
    As: IntoIterator<Item = A>,
    Bs: IntoIterator<Item = B>,
{
    xs.into_iter().zip(ys)
}

#[inline(always)]
pub fn zip_with<A, B, C, As, Bs, F>(
    f: F,
    xs: As,
    ys: Bs,
) -> std::iter::Map<
    std::iter::Zip<<As as IntoIterator>::IntoIter, <Bs as IntoIterator>::IntoIter>,
    impl FnMut((A, B)) -> C,
>
where
    As: IntoIterator<Item = A>,
    Bs: IntoIterator<Item = B>,
    F: Fn(A, B) -> C,
{
    xs.into_iter().zip(ys).map(move |(a, b)| f(a, b))
}

#[inline(always)]
pub fn unzip<A, B, FromA, FromB>(xs: impl IntoIterator<Item = (A, B)>) -> (FromA, FromB)
where
    FromA: Default + Extend<A>,
    FromB: Default + Extend<B>,
{
    let mut as_ = FromA::default();
    let mut bs = FromB::default();

    for (a, b) in xs {
        as_.extend(Some(a));
        bs.extend(Some(b));
    }

    (as_, bs)
}

// Functions on strings

#[inline(always)]
pub fn lines(s: &str) -> std::str::Lines {
    s.lines()
}

#[inline(always)]
pub fn words(s: &str) -> std::str::SplitWhitespace {
    s.split_whitespace()
}

pub fn unlines<I>(lines: I) -> String
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut result = lines.into_iter().fold(String::new(), |mut acc, line| {
        acc.push_str(line.as_ref());
        acc.push('\n');
        acc
    });

    result.pop(); // Remove the trailing newline

    result
}

pub fn unwords<I>(words: I) -> String
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut result = words.into_iter().fold(String::new(), |mut acc, word| {
        acc.push_str(word.as_ref());
        acc.push(' ');
        acc
    });

    result.pop(); // Remove the trailing space

    result
}

// Io

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
        F: FnOnce(Self::Output) -> B + Clone,
    {
        IoBind { io: self, f }
    }

    fn then<Mb>(self, mb: Mb) -> IoBind<Self, impl FnOnce(Self::Output) -> Mb + Clone>
    where
        Mb: Io + Clone,
    {
        IoBind {
            io: self,
            f: |_| mb,
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct IoPure<A> {
    io: A,
}

impl<A> Io for IoPure<A> {
    type Output = A;

    fn run(self) -> Self::Output {
        self.io
    }
}

#[derive(Clone)]
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

// pub struct PutStr<'a>(&'a str);

// impl<'a> crate::prelude::Io for PutStr<'a> {
//     type Output = ();

//     fn run(self) {
//         print!("{}", self.0);
//     }
// }

// pub fn put_str(s: &str) -> PutStr {
//     PutStr(s)
// }

#[derive(Clone)]
pub struct PutStr(std::string::String);

impl crate::prelude::Io for PutStr {
    type Output = ();

    fn run(self) {
        print!("{}", self.0);
    }
}

pub fn put_str<S>(s: S) -> PutStr
where
    S: Into<String>,
{
    PutStr(s.into())
}

// pub struct PutStrLn<'a>(&'a str);

// impl<'a> crate::prelude::Io for PutStrLn<'a> {
//     type Output = ();

//     fn run(self) {
//         println!("{}", self.0);
//     }
// }

// pub fn put_strln(s: &str) -> PutStrLn {
//     PutStrLn(s)
// }

#[derive(Clone)]
pub struct PutStrLn(std::string::String);

impl crate::prelude::Io for PutStrLn {
    type Output = ();

    fn run(self) {
        println!("{}", self.0);
    }
}

pub fn put_str_ln<S>(s: S) -> PutStrLn
where
    S: Into<String>,
{
    PutStrLn(s.into())
}

/// Get a line from standard input.
/// The last newline character(/n or /r/n on Windows) will not returned as part of output.
#[allow(non_camel_case_types)]
#[derive(Clone)]
pub struct get_line;

impl crate::prelude::Io for get_line {
    type Output = String;

    fn run(self) -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();

        // Check if the last character is a newline and remove it
        if s.ends_with('\n') {
            s.pop();
        }
        // Check for Windows-style newline (\r\n)
        if s.ends_with('\r') {
            s.pop();
        }

        s
    }
}

// Additional functions

pub fn filter_map<A, B, As, F>(
    f: F,
    xs: As,
) -> std::iter::FilterMap<<As as IntoIterator>::IntoIter, F>
where
    As: IntoIterator<Item = A>,
    F: Fn(A) -> Option<B>,
{
    xs.into_iter().filter_map(f)
}

pub fn find<A, F>(f: F, xs: impl IntoIterator<Item = A>) -> Option<A>
where
    F: Fn(&A) -> bool,
{
    xs.into_iter().find(f)
}

pub fn elem_index<A>(x: A, xs: impl IntoIterator<Item = A>) -> Option<usize>
where
    A: PartialEq,
{
    xs.into_iter().position(|a| a == x)
}

pub fn find_index<A, F>(f: F, xs: impl IntoIterator<Item = A>) -> Option<usize>
where
    F: Fn(&A) -> bool,
{
    xs.into_iter().position(|a| f(&a))
}

pub fn elem_indecies<A>(x: A, xs: impl IntoIterator<Item = A>) -> Vec<usize>
where
    A: PartialEq,
{
    xs.into_iter()
        .enumerate()
        .filter_map(|(i, a)| if a == x { Some(i) } else { None })
        .collect()
}

pub fn find_indecies<A, F>(f: F, xs: impl IntoIterator<Item = A>) -> Vec<usize>
where
    F: Fn(&A) -> bool,
{
    xs.into_iter()
        .enumerate()
        .filter_map(|(i, a)| if f(&a) { Some(i) } else { None })
        .collect()
}

// todo intersparse, intercallate

pub fn sort<A: Ord, As: IntoIterator<Item = A>>(xs: As) -> Vec<A> {
    let mut xs: Vec<A> = xs.into_iter().collect();
    xs.sort();
    xs
}

pub fn sort_on<A, B: Ord, As: IntoIterator<Item = A>, F>(f: F, xs: As) -> Vec<A>
where
    F: Fn(&A) -> B,
{
    let mut xs: Vec<A> = xs.into_iter().collect();
    xs.sort_by_key(f);
    xs
}

pub fn is_prefix_of<A>(prefix: impl IntoIterator<Item = A>, xs: impl IntoIterator<Item = A>) -> bool
where
    A: PartialEq,
{
    let mut prefix = prefix.into_iter();
    let mut xs = xs.into_iter();

    loop {
        match (prefix.next(), xs.next()) {
            (Some(a), Some(b)) if a == b => continue,
            (None, _) => return true,
            _ => return false,
        }
    }
}

pub fn is_suffix_of<A>(
    suffix: impl IntoIterator<Item = A, IntoIter: DoubleEndedIterator<Item = A>>,
    xs: impl IntoIterator<Item = A, IntoIter: DoubleEndedIterator<Item = A>>,
) -> bool
where
    A: PartialEq,
{
    let mut suffix = suffix.into_iter().rev();
    let mut xs = xs.into_iter().rev();

    loop {
        match (suffix.next(), xs.next()) {
            (Some(a), Some(b)) if a == b => continue,
            (None, _) => return true,
            _ => return false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fst() {
        assert_eq!(fst((1, 2)), 1);
    }

    #[test]
    fn test_snd() {
        assert_eq!(snd((1, 2)), 2);
    }

    // Fold and traverse

    #[test]
    fn test_foldl() {
        assert_eq!(foldl(|acc, x| acc + x, 0, Vec::<i32>::new()), 0);

        assert_eq!(foldl(|acc, x| acc + x, 0, vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_foldr() {
        assert_eq!(foldr(|x, acc| acc + x, 0, Vec::<i32>::new()), 0);

        assert_eq!(foldr(|x, acc| acc + x, 0, vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_elem() {
        assert_eq!(elem(1, Vec::<i32>::new()), false);

        assert_eq!(elem(1, vec![2, 3, 4]), false);
        assert_eq!(elem(1, vec![1, 2, 3]), true);
    }

    #[test]
    fn test_maximum() {
        assert_eq!(maximum(Vec::<i32>::new()), None);

        assert_eq!(maximum(vec![1, 2, 3, 4, 5]), Some(5));
    }

    #[test]
    fn test_minimum() {
        assert_eq!(minimum(Vec::<i32>::new()), None);

        assert_eq!(minimum(vec![1, 2, 3, 4, 5]), Some(1));
    }

    #[test]
    fn test_sum() {
        assert_eq!(sum(Vec::<i32>::new()), 0);

        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    // #[test]
    // fn test_product() {
    //     assert_eq!(product(Vec::<i32>::new()), 1);

    //     assert_eq!(product(vec![1, 2, 3, 4, 5]), 120);
    // }

    #[test]
    fn test_any() {
        assert_eq!(any(|x| x > 0, Vec::<i32>::new()), false);

        assert_eq!(any(|x| x > 0, vec![-1, -2, -3]), false);
        assert_eq!(any(|x| x > 0, vec![-1, 2, -3]), true);
    }

    #[test]
    fn test_all() {
        assert_eq!(all(|x| x > 0, Vec::<i32>::new()), true);

        assert_eq!(all(|x| x > 0, vec![-1, -2, -3]), false);
        assert_eq!(all(|x| x > 0, vec![1, 2, 3]), true);
    }

    // Miscellaneous functions

    #[test]
    fn test_id() {
        assert_eq!(id(1), 1);
    }

    // List operations

    #[test]
    fn test_map() {
        assert_eq!(
            map(|x| x + 1, Vec::<i32>::new()).collect::<Vec<_>>(),
            Vec::new()
        );

        assert_eq!(
            map(|x| x + 1, vec![1, 2, 3]).collect::<Vec<_>>(),
            vec![2, 3, 4]
        );
    }

    #[test]
    fn test_append() {
        assert_eq!(
            append(Vec::<i32>::new(), Vec::<i32>::new())
                .into_iter()
                .collect::<Vec<_>>(),
            Vec::new()
        );

        assert_eq!(
            append(vec![1, 2, 3], vec![4, 5, 6])
                .into_iter()
                .collect::<Vec<_>>(),
            vec![1, 2, 3, 4, 5, 6]
        );
    }

    #[test]
    fn test_filter() {
        assert_eq!(
            filter(|x| x > &0, Vec::<i32>::new()).collect::<Vec<_>>(),
            Vec::new()
        );

        assert_eq!(
            filter(|x| x > &0, vec![-1, -2, -3]).collect::<Vec<_>>(),
            Vec::new()
        );
        assert_eq!(
            filter(|x| x > &0, vec![-1, 2, -3]).collect::<Vec<_>>(),
            vec![2]
        );
    }

    #[test]
    fn test_head() {
        // assert_eq!(head(Vec::<i32>::new()), None);

        assert_eq!(head(vec![1, 2, 3]), 1);
    }

    #[test]
    fn test_last() {
        // assert_eq!(last(Vec::<i32>::new()), None);

        assert_eq!(last(vec![1, 2, 3]), 3);
    }

    #[test]
    fn test_tail() {
        assert_eq!(tail(Vec::<i32>::new()).collect::<Vec<_>>(), Vec::new());

        assert_eq!(tail(vec![1, 2, 3]).collect::<Vec<_>>(), vec![2, 3]);
    }

    #[test]
    fn test_init() {
        assert_eq!(init(Vec::<i32>::new()).collect::<Vec<_>>(), Vec::new());

        assert_eq!(init(vec![1, 2, 3]).collect::<Vec<_>>(), vec![1, 2]);
    }

    #[test]
    fn test_nth() {
        assert_eq!(nth(0, Vec::<i32>::new()), None);

        assert_eq!(nth(0, vec![1, 2, 3]), Some(1));
    }

    #[test]
    fn test_length() {
        assert_eq!(length(Vec::<i32>::new()), 0);

        assert_eq!(length(vec![1, 2, 3]), 3);
    }

    #[test]
    fn test_reverse() {
        assert_eq!(reverse(Vec::<i32>::new()), Vec::<i32>::new());

        assert_eq!(reverse(vec![1, 2, 3]), vec![3, 2, 1]);
    }

    #[test]
    fn test_concat() {
        assert_eq!(
            concat(Vec::<Vec<i32>>::new()).collect::<Vec<_>>(),
            Vec::<i32>::new()
        );

        assert_eq!(
            concat(vec![vec![1, 2], vec![3, 4]]).collect::<Vec<_>>(),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_concat_map() {
        assert_eq!(
            concat_map(|x| vec![x, x], Vec::<i32>::new()).collect::<Vec<_>>(),
            Vec::new()
        );
        assert_eq!(
            concat_map(|x| vec![x, x], vec![1, 2, 3]).collect::<Vec<_>>(),
            vec![1, 1, 2, 2, 3, 3]
        );
    }

    // Building lists

    #[test]
    fn test_scanl() {
        assert_eq!(
            scanl(|acc, x| *acc && (x % 2 == 0), true, Vec::<i32>::new()).collect::<Vec<_>>(),
            vec![true]
        );

        assert_eq!(
            // Take until value is all even
            scanl(|acc, x| *acc && (x % 2 == 0), true, vec![0, 2, 4, 5, 7]).collect::<Vec<_>>(),
            vec![
                true, true,  // 0
                true,  // 0 && 2
                true,  // 0 && 2 && 4
                false, // 0 && 2 && 4 && 5
                false, // 0 && 2 && 4 && 5 && 7
            ]
        );
    }

    #[test]
    fn test_scanl1() {
        assert_eq!(
            scanl1(|acc, x| acc + x, Vec::<i32>::new()).collect::<Vec<_>>(),
            Vec::new()
        );

        assert_eq!(
            scanl1(|acc, x| acc + x, vec![1, 2, 3, 4, 5]).collect::<Vec<_>>(),
            vec![1, 3, 6, 10, 15]
        );
    }

    // Infinite lists

    #[test]
    fn test_iterate() {
        assert_eq!(
            iterate(|x| x + 1, 0).take(5).collect::<Vec<_>>(),
            vec![0, 1, 2, 3, 4]
        );
    }

    #[test]
    fn test_repeat() {
        assert_eq!(repeat(1).take(5).collect::<Vec<_>>(), vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_replicate() {
        assert_eq!(replicate(5, 1).collect::<Vec<_>>(), vec![1, 1, 1, 1, 1]);
    }

    #[test]
    fn test_cycle() {
        assert_eq!(
            cycle(vec![1, 2, 3]).take(7).collect::<Vec<_>>(),
            vec![1, 2, 3, 1, 2, 3, 1]
        );
    }

    // Sublists

    #[test]
    fn test_take() {
        assert_eq!(take(0, Vec::<i32>::new()).collect::<Vec<_>>(), Vec::new());

        assert_eq!(take(2, vec![1, 2, 3]).collect::<Vec<_>>(), vec![1, 2]);
    }

    #[test]
    fn test_drop() {
        assert_eq!(drop(0, Vec::<i32>::new()).collect::<Vec<_>>(), Vec::new());

        assert_eq!(drop(2, vec![1, 2, 3]).collect::<Vec<_>>(), vec![3]);
    }

    #[test]
    fn test_take_while() {
        assert_eq!(
            take_while(|x| x < &3, Vec::<i32>::new()).collect::<Vec<_>>(),
            Vec::new()
        );

        assert_eq!(
            take_while(|x| x < &3, vec![1, 2, 3, 4, 5]).collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    #[test]
    fn test_drop_while() {
        assert_eq!(
            drop_while(|x| x < &3, Vec::<i32>::new()).collect::<Vec<_>>(),
            Vec::new()
        );

        assert_eq!(
            drop_while(|x| x < &3, vec![1, 2, 3, 4, 5]).collect::<Vec<_>>(),
            vec![3, 4, 5]
        );
    }

    #[test]
    fn test_span() {
        let (xs, ys) = span(|x| x < &3, vec![1, 2, 3, 4, 5]);

        assert_eq!(xs.collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(ys.collect::<Vec<_>>(), vec![3, 4, 5]);
    }

    #[test]
    fn test_split_at() {
        let (xs, ys) = split_at(2, vec![1, 2, 3, 4, 5]);

        assert_eq!(xs.collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(ys.collect::<Vec<_>>(), vec![3, 4, 5]);
    }

    #[test]
    fn test_zip() {
        assert_eq!(
            zip(vec![1, 2, 3], vec![4, 5, 6]).collect::<Vec<_>>(),
            vec![(1, 4), (2, 5), (3, 6)]
        );
    }

    #[test]
    fn test_zip_with() {
        assert_eq!(
            zip_with(|x, y| x + y, vec![1, 2, 3], vec![4, 5, 6]).collect::<Vec<_>>(),
            vec![5, 7, 9]
        );
    }

    #[test]
    fn test_unzip() {
        let (xs, ys): (Vec<_>, Vec<_>) = unzip(vec![(1, 4), (2, 5), (3, 6)]);

        assert_eq!(xs, vec![1, 2, 3]);
        assert_eq!(ys, vec![4, 5, 6]);
    }

    // Functions on strings

    #[test]
    fn test_lines() {
        assert_eq!(lines("").collect::<Vec<_>>(), Vec::<&str>::new());

        assert_eq!(lines("a\nb\nc").collect::<Vec<_>>(), vec!["a", "b", "c"]);

        assert_eq!(
            lines(&"".to_string()).collect::<Vec<_>>(),
            Vec::<&str>::new()
        );

        assert_eq!(
            lines(&"a\nb\nc".to_string()).collect::<Vec<_>>(),
            vec!["a", "b", "c"]
        );
    }

    #[test]
    fn test_words() {
        assert_eq!(words("").collect::<Vec<_>>(), Vec::<&str>::new());

        assert_eq!(words("a b c").collect::<Vec<_>>(), vec!["a", "b", "c"]);
    }

    #[test]
    fn test_unlines() {
        assert_eq!(unlines(Vec::<&str>::new()), "");

        assert_eq!(unlines(vec!["a", "b", "c"]), "a\nb\nc");
    }

    #[test]
    fn test_unwords() {
        assert_eq!(unwords(Vec::<&str>::new()), "");

        assert_eq!(unwords(vec!["a", "b", "c"]), "a b c");
    }

    // No break since it is a keyword

    // Additional functions

    #[test]
    fn test_filter_map() {
        assert_eq!(
            filter_map(
                |x| if x % 2 == 0 { Some(x) } else { None },
                vec![1, 2, 3, 4, 5]
            )
            .collect::<Vec<_>>(),
            vec![2, 4]
        );
    }

    #[test]
    fn test_find() {
        assert_eq!(find(|x| *x == 1, vec![1, 2, 3]), Some(1));
        assert_eq!(find(|x| *x == 1, vec![2, 3, 4]), None);
    }

    #[test]
    fn test_elem_index() {
        assert_eq!(elem_index(1, vec![1, 2, 3]), Some(0));
        assert_eq!(elem_index(1, vec![2, 3, 4]), None);
    }

    #[test]
    fn test_find_index() {
        assert_eq!(find_index(|x| *x == 1, vec![1, 2, 3]), Some(0));
        assert_eq!(find_index(|x| *x == 1, vec![2, 3, 4]), None);
    }

    #[test]
    fn test_elem_indecies() {
        assert_eq!(find_indecies(|x| *x == 1, vec![1, 2, 3]), vec![0]);
        assert_eq!(
            find_indecies(|x| *x == 1, vec![2, 3, 4]),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn test_find_indecies() {
        assert_eq!(find_indecies(|x| *x == 1, vec![1, 2, 3]), vec![0]);
        assert_eq!(
            find_indecies(|x| *x == 1, vec![2, 3, 4]),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn test_sort() {
        assert_eq!(sort(vec![3, 2, 1]), vec![1, 2, 3]);
    }

    #[test]
    fn test_sort_on() {
        assert_eq!(sort_on(|x| -x, vec![3, 2, 1]), vec![3, 2, 1]);
    }

    #[test]
    fn test_is_prefix_of() {
        assert_eq!(is_prefix_of(vec![1, 2], vec![1, 2, 3]), true);
        assert_eq!(is_prefix_of(vec![1, 2], vec![1, 3, 4]), false);

        assert_eq!(is_prefix_of("ab".chars(), "abc".chars()), true);
        assert_eq!(is_prefix_of("ab".chars(), "acb".chars()), false);
    }

    #[test]
    fn test_is_suffix_of() {
        assert_eq!(is_suffix_of(vec![2, 3], vec![1, 2, 3]), true);
        assert_eq!(is_suffix_of(vec![2, 3], vec![1, 3, 4]), false);

        assert_eq!(is_suffix_of("bc".chars(), "abc".chars()), true);
        assert_eq!(is_suffix_of("bc".chars(), "acb".chars()), false);
    }
}
