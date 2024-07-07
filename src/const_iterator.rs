// pub trait ConstIterable {
//     type Item;
//     fn const_iter(self) -> impl ConstIterator<Item = Self::Item>;
// }

// pub trait ConstIterator {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item>;
// }

// impl<T> ConstIterable for T
// where
//     T: ConstIterator,
// {
//     type Item = T::Item;

//     fn const_iter(self) -> impl ConstIterator<Item = Self::Item> {
//         self
//     }
// }

// impl<'a, T> ConstIterator for std::slice::Iter<'a, T> {
//     type Item = &'a T;

//     fn next(&mut self) -> Option<Self::Item> {
//         Iterator::next(self)
//     }
// }

// impl<'a, T> ConstIterator for std::vec::IntoIter<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {
//         Iterator::next(self)
//     }
// }

// // Impl ConstItrable for Vec<T>

// impl<T> ConstIterable for Vec<T> {
//     type Item = T;

//     fn const_iter(self) -> impl ConstIterator<Item = Self::Item> {
//         self.into_iter()
//     }
// }
