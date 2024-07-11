use crate::{base::hkt::Hkt1, data::functor::Functor, misc::in_place};

#[derive(Debug, PartialEq, Clone)]
pub struct Tree<T> {
    pub value: T,
    pub children: Vec<Tree<T>>,
}

// unfold :: (b -> (a, [b])) -> b -> Tree a

impl<T> Tree<T> {
    pub fn unfold<F, B>(f: &F, b: &B) -> Tree<T>
    where
        F: Fn(&B) -> (T, Vec<B>) + Clone,
    {
        let (value, children) = f(&b);
        let children = children.into_iter().map(|b| Tree::unfold(f, &b)).collect();
        Tree { value, children }
    }

    pub fn zip<B>(self, other: Tree<B>) -> Tree<(T, B)>
    where
        T: Clone,
        B: Clone,
    {
        let new_value = (self.value, other.value);
        let new_children = self
            .children
            .into_iter()
            .zip(other.children.into_iter())
            .map(|(child1, child2)| child1.zip(child2))
            .collect();

        Tree {
            value: new_value,
            children: new_children,
        }
    }

    pub fn iter(&self) -> TreeIter<T> {
        TreeIter { stack: vec![self] }
    }

    pub fn update<F: Fn(&mut T)>(&mut self, f_mut: &F) {
        f_mut(&mut self.value);
        self.children.iter_mut().for_each(|c| c.update(f_mut));
    }

    fn fmap_ref<B, F>(self, f: &F) -> Tree<B>
    where
        F: Fn(T) -> B + Clone,
    {
        Tree {
            value: f(self.value),
            children: self
                .children
                .into_iter()
                .map(|child| child.fmap_ref(f))
                .collect(),
        }
    }
}

// Implement Iterator for TreeIter

pub struct TreeIter<'a, T> {
    stack: Vec<&'a Tree<T>>,
}

impl<'a, T> Iterator for TreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        self.stack.extend(node.children.iter().rev());

        Some(&node.value)
    }
}

// Implement IntoIterator for Tree

pub struct TreeIntoIter<T> {
    stack: Vec<Tree<T>>,
}

impl<T> Iterator for TreeIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;

        self.stack.extend(node.children.into_iter().rev());

        Some(node.value)
    }
}

impl<T> IntoIterator for Tree<T> {
    type Item = T;
    type IntoIter = TreeIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        TreeIntoIter { stack: vec![self] }
    }
}

// Implement Hkt1 for Tree

impl<T> Hkt1 for Tree<T> {
    type HktOf1 = T;

    // type Of<W1> = Tree<W1>;
}

// impl<A> Functor<A> for Tree<A> {
//     type Map<B, F> = Tree<B>
//     where
//         F: Fn(A) -> B + Clone;

//     fn fmap<B, F>(self, f: F) -> Tree<B>
//     where
//         F: Fn(A) -> B + Clone,
//     {
//         self.fmap_ref(&f)
//     }

//     fn fmap1<F>(mut self, f: F) -> Self
//     where
//         F: Fn(A) -> A,
//     {
//         self.update(&in_place(f));
//         self
//     }
// }

impl<T> Functor for Tree<T> {
    type Map<B, F> = Tree<B>
    where
        F: Fn(T) -> B + Clone;

    fn fmap<B, F>(self, f: F) -> Tree<B>
    where
        F: Fn(T) -> B + Clone,
    {
        self.fmap_ref(&f)
    }

    fn fmap1<F>(mut self, f: F) -> Self
    where
        F: Fn(T) -> T,
    {
        self.update(&in_place(f));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unfold() {
        let f = |n: &u32| {
            if *n == 0 {
                (0, vec![])
            } else {
                (*n, vec![n - 1])
            }
        };
        let tree = Tree::unfold(&f, &3);

        assert_eq!(
            tree,
            Tree {
                value: 3,
                children: vec![Tree {
                    value: 2,
                    children: vec![Tree {
                        value: 1,
                        children: vec![Tree {
                            value: 0,
                            children: vec![]
                        }]
                    }]
                }]
            }
        );
    }

    #[test]
    fn test_zip() {
        let tree1 = Tree {
            value: 1,
            children: vec![
                Tree {
                    value: 11,
                    children: vec![Tree {
                        value: 111,
                        children: vec![],
                    }],
                },
                Tree {
                    value: 12,
                    children: vec![
                        Tree {
                            value: 121,
                            children: vec![],
                        },
                        Tree {
                            value: 122,
                            children: vec![],
                        },
                    ],
                },
            ],
        };

        let tree2 = Tree {
            value: 'a',
            children: vec![
                Tree {
                    value: 'b',
                    children: vec![Tree {
                        value: 'c',
                        children: vec![],
                    }],
                },
                Tree {
                    value: 'd',
                    children: vec![
                        Tree {
                            value: 'e',
                            children: vec![],
                        },
                        Tree {
                            value: 'f',
                            children: vec![],
                        },
                    ],
                },
            ],
        };

        let zipped = tree1.zip(tree2);

        assert_eq!(
            zipped,
            Tree {
                value: (1, 'a'),
                children: vec![
                    Tree {
                        value: (11, 'b'),
                        children: vec![Tree {
                            value: (111, 'c'),
                            children: vec![]
                        }]
                    },
                    Tree {
                        value: (12, 'd'),
                        children: vec![
                            Tree {
                                value: (121, 'e'),
                                children: vec![]
                            },
                            Tree {
                                value: (122, 'f'),
                                children: vec![]
                            }
                        ]
                    }
                ]
            }
        );
    }

    #[test]
    fn test_fmap() {
        let tree1 = Tree {
            value: 1,
            children: vec![
                Tree {
                    value: 11,
                    children: vec![Tree {
                        value: 111,
                        children: vec![],
                    }],
                },
                Tree {
                    value: 12,
                    children: vec![
                        Tree {
                            value: 121,
                            children: vec![],
                        },
                        Tree {
                            value: 122,
                            children: vec![],
                        },
                    ],
                },
            ],
        };

        let f = |n: i32| n.to_string();

        let mapped = tree1.fmap(&f);

        assert_eq!(
            mapped,
            Tree {
                value: "1".to_string(),
                children: vec![
                    Tree {
                        value: "11".to_string(),
                        children: vec![Tree {
                            value: "111".to_string(),
                            children: vec![]
                        }]
                    },
                    Tree {
                        value: "12".to_string(),
                        children: vec![
                            Tree {
                                value: "121".to_string(),
                                children: vec![]
                            },
                            Tree {
                                value: "122".to_string(),
                                children: vec![]
                            }
                        ]
                    }
                ]
            }
        );
    }

    #[test]
    fn test_into_iter_0() {
        let tree = Tree {
            value: 1,
            children: vec![],
        };

        let flattened = tree.into_iter();

        assert_eq!(flattened.collect::<Vec<_>>(), vec![1]);
    }

    #[test]
    fn test_flatten_1() {
        let tree = Tree {
            value: 1,
            children: vec![
                Tree {
                    value: 11,
                    children: vec![Tree {
                        value: 111,
                        children: vec![],
                    }],
                },
                Tree {
                    value: 12,
                    children: vec![
                        Tree {
                            value: 121,
                            children: vec![],
                        },
                        Tree {
                            value: 122,
                            children: vec![],
                        },
                    ],
                },
            ],
        };

        let flattened: Vec<i32> = tree.into_iter().collect();

        assert_eq!(flattened, vec![1, 11, 111, 12, 121, 122]);
    }
}
