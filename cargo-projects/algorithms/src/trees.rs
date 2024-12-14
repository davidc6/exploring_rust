#[derive(Debug, Default)]
struct SubNode<T: Ord>(Option<Box<TreeNode<T>>>);

impl<T: Ord> SubNode<T> {
    pub fn new() -> Self {
        Self(None)
    }
}

#[derive(Debug, Default)]
struct TreeNode<T: Ord> {
    val: T,
    left: SubNode<T>,
    right: SubNode<T>,
}

struct BinaryTree<T: Ord> {
    root: SubNode<T>,
    len: usize,
}

impl<T: Ord> BinaryTree<T> {
    pub fn new() -> Self {
        Self {
            root: SubNode::new(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

#[cfg(test)]
mod trees_tests {
    use super::BinaryTree;

    #[test]
    fn trees_works() {
        let bt: BinaryTree<_> = BinaryTree::<usize>::new();

        assert_eq!(bt.len(), 0);
    }
}
