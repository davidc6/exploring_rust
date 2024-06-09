use std::rc::Rc;

pub struct LinkedList<T> {
    head: LinkedListNodeConnection<T>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList::new()
    }
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

impl<T> LinkedList<T> {
    /// iter() method instantiates IteratorState struct,
    /// which allows us to then iterate over the linked list.
    /// '_ - the compiler infers the lifetime
    pub fn iter(&self) -> IteratorState<'_, T> {
        IteratorState {
            // as_deref() leaves the original option in-place,
            // and creates a new one with the reference to the original one.
            next: self.head.as_deref(),
        }
    }
}

struct LinkedListNode<T> {
    current_node: T,
    next_node: LinkedListNodeConnection<T>,
}

type LinkedListNodeConnection<T> = Option<Rc<LinkedListNode<T>>>;

pub struct IteratorState<'a, T> {
    next: Option<&'a LinkedListNode<T>>,
}

impl<'a, T> Iterator for IteratorState<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            // Creates a reference to the next Option LinkedListNode (keeping it in-place),
            // coercing the contents via Deref trait.
            self.next = node.next_node.as_deref();
            // Return a reference to the current LinkedListNode
            &node.current_node
        })
    }
}

///
impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // take() - take value of head (Option) and leave None in place
        let mut head = self.head.take();

        // While head is a valid Node (i.e. it is Some and not None),
        // set head to the next Node (until we can't hoist out of node anymore)
        while let Some(node) = head {
            // If we know that we are the last list that knows about this node,
            // we can move LinkedListNode out of Rc
            if let Ok(mut node) = Rc::try_unwrap(node) {
                // take next Node and replace with None
                head = node.next_node.take();
            } else {
                break;
            }
        }
    }
}

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

    #[test]
    fn linked_list_iter_method_iterates_over_reference_of_type_t() {
        let ll = LinkedList::new().prepend(1).prepend(2).prepend(3);
        let mut iter = ll.iter();

        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }
}
