use std::rc::Rc;

pub struct LinkedList<T> {
    head: LinkedListNodeConnection<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    /// Prepends new node to the list and return s a new list.
    ///
    /// Current head becomes next node of the new node.
    ///
    /// Example:
    ///
    /// (before prepend)
    /// [Current head] -> [Next node (if any)]
    ///
    /// (after prepend)
    /// [New (prepended) Node] -> [Previous Head] -> [Next node (if any)]
    pub fn prepend(&self, node: T) -> LinkedList<T> {
        LinkedList {
            head: Some(Rc::new(LinkedListNode {
                current_node: node,
                next_node: self.head.clone(),
            })),
        }
    }

    pub fn tail(&self) -> LinkedList<T> {
        LinkedList {
            head: self.head.as_ref().and_then(|node| node.next_node.clone()),
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.current_node)
    }
}

struct LinkedListNode<T> {
    current_node: T,
    next_node: LinkedListNodeConnection<T>,
}

type LinkedListNodeConnection<T> = Option<Rc<LinkedListNode<T>>>;

// Iterator - returns each value
// impl<T> Iterator for LinkedList<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<Self::Item> {}
// }

#[cfg(test)]
mod linked_list_three_tests {
    use super::*;

    #[test]
    fn linked_list_operations_perform_as_expected() {
        let ll = LinkedList::new();
        assert_eq!(ll.head(), None);

        let ll = ll.prepend(1).prepend(2);
        assert_eq!(ll.head(), Some(&2));

        let ll = ll.tail();
        assert_eq!(ll.head(), Some(&1));

        let ll = ll.tail();
        assert_eq!(ll.head(), None);

        assert_eq!(ll.head(), None);
    }
}
