use crate::{functor::Functor, hkt::Hkt1, impl_hkt1};

#[derive(Debug, PartialEq)]
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

    pub fn flatten(&self) -> Vec<&T> {
        let mut result = vec![&self.value];

        for child in &self.children {
            result.extend(child.flatten());
        }

        result
    }
}

// impl_hkt1!(Tree);

impl<T> Hkt1 for Tree<T> {
    type HktOf1 = T;

    type Of<W1> = Tree<W1>;
}

impl<T> Functor for Tree<T> {
    fn fmap<B, F>(self, f: &F) -> Tree<B>
    where
        F: for<'a> Fn(&'a T) -> B,
    {
        Tree {
            value: f(&self.value),
            children: self
                .children
                .into_iter()
                .map(|child| child.fmap(f))
                .collect(),
        }
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

        let f = |n: &i32| n.to_string();

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
    fn test_flatten_0() {
        let tree = Tree {
            value: 1,
            children: vec![],
        };

        let flattened = tree.flatten();

        assert_eq!(flattened, vec![&1]);
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

        let flattened = tree.flatten();

        assert_eq!(flattened, vec![&1, &11, &111, &12, &121, &122]);
    }
}
