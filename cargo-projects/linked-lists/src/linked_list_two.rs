// Represents a connection between nodes in the list.
//
// To make ListNodeConnection representable, we need to insert indirection (i.e. Box<ListNodeConnection>).
// This means storing a pointer to a value instead of the value itself.
// Box allocates value on the heap but the pointer itself lives on the stack.
// This way we know the size of Box.
type ListNodeConnection<T> = Option<Box<ListNode<T>>>;

// Linked List's Node that is holding a value and link to the next node (if any)
#[derive(Debug)]
struct ListNode<T> {
    elem: T,
    next_elem: ListNodeConnection<T>,
}

// Since LinkedList is a single struct,
// the size of the struct is the same as the field
#[derive(Debug)]
pub struct LinkedList<T> {
    head: ListNodeConnection<T>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    // :: is the namespace operator which allows us to choose enum variant
    pub fn new() -> Self {
        LinkedList {
            head: ListNodeConnection::None,
        }
    }

    pub fn peek(&self) -> Option<&T> {
        // map() takes self by value
        // by using as_ref(), it provides an Option to a ref to its internals which is what in required here.
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        // map() takes self by value
        // by using as_ref(), it provides an Option to a ref to its internals which is what in required here.
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn push(&mut self, value: T) {
        // std::mem::replace - moves src (second argument) into the references dest (first argument) and returns previous dest value
        // Move source (ListNodeConnection::None) into destination (self.head)
        // and return previous destination. Here self.head temporarily gets set to ListNodeConnection::None.
        let node = ListNode {
            elem: value,
            // Since mem::replace(&mut option, None) is a very common idiom, Option has a method for it: Option.take()
            // next_elem: mem::replace(&mut self.head, ListNodeConnection::None),
            next_elem: self.head.take(),
        };

        // Set head to Some list with new node. We replace the previously set self.head with the new "head".
        self.head = ListNodeConnection::Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next_elem;
            node.elem
        })
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();

        // Lift ListNodes out of their Boxes
        while let ListNodeConnection::Some(mut node) = current {
            current = node.next_elem.take();
        }
    }
}

// Using tuple struct here to wrap around LinkedList type
pub struct LinkedListIter<T>(LinkedList<T>);

// Implement Iterator on the wrapped type
impl<T> Iterator for LinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        // 0 is essentially the wrapped type i.e. LinkedList<T>
        self.0.pop()
    }
}

// Implementing into_iter method on LinkedList type
// to define how to create an iterator
impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = LinkedListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        LinkedListIter(self)
    }
}

// Iter struct is required to implement Iterator on.
pub struct Iter<'a, T> {
    next: Option<&'a ListNode<T>>,
}

// iter() return Iter which implements Iterator.
impl<T> LinkedList<T> {
    pub fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

// Actual Iterator implementation.
impl<'a, T> Iterator for Iter<'a, T> {
    // Item type is a reference to T item
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next_elem.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut ListNode<T>>,
}

// iter() return Iter which implements Iterator.
impl<T> LinkedList<T> {
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next_elem.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod linked_list_two_tests {
    use super::*;

    #[test]
    fn linked_list_is_constructed_correctly() {
        let ll = LinkedList {
            head: ListNodeConnection::Some(Box::new(ListNode {
                elem: 1,
                next_elem: ListNodeConnection::None,
            })),
        };

        assert!(matches!(ll, LinkedList { head: _ }));
    }

    #[test]
    fn linked_list_pop_method_returns_valid_value() {
        // Initialise new LinkedList
        let mut ll = LinkedList::new();

        // Initially the list should be empty
        assert_eq!(ll.pop(), None);

        // Push values to populate the list
        ll.push(2);
        ll.push(4);
        ll.push(6);

        // Test order and values
        assert_eq!(ll.pop(), Some(6));
        assert_eq!(ll.pop(), Some(4));
        assert_eq!(ll.pop(), Some(2));

        // Push more
        ll.push(8);
        ll.push(10);

        // Test some more values since previous once were popped
        assert_eq!(ll.pop(), Some(10));
        assert_eq!(ll.pop(), Some(8));

        // None linked list
        assert_eq!(ll.pop(), None);
    }

    #[test]
    fn linked_list_peek_works() {
        let mut ll = LinkedList::new();
        assert_eq!(ll.peek(), None);
        assert_eq!(ll.peek_mut(), None);

        // Push some values
        ll.push(1);
        ll.push(2);
        ll.push(3);

        assert_eq!(ll.peek(), Some(&3));
        assert_eq!(ll.peek_mut(), Some(&mut 3));
    }

    #[test]
    fn linked_list_peek_mut_allows_mutation() {
        let mut ll = LinkedList::new();

        assert_eq!(ll.peek(), None);
        assert_eq!(ll.peek_mut(), None);

        // Push some values
        ll.push(1);
        ll.push(2);
        ll.push(3);

        if let Some(val) = ll.peek_mut() {
            *val = 100;
        }

        assert_eq!(ll.peek(), Some(&100));
        assert_eq!(ll.pop(), Some(100));
    }

    #[test]
    fn linked_list_iterator_next() {
        let mut ll = LinkedList::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);

        let mut ll_iter = ll.into_iter();
        assert_eq!(ll_iter.next(), Some(3));
        assert_eq!(ll_iter.next(), Some(2));
        assert_eq!(ll_iter.next(), Some(1));
        assert_eq!(ll_iter.next(), None);
    }

    #[test]
    fn linked_list_iterator_next_loop() {
        let mut ll = LinkedList::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);

        let mut expected_val = 3;

        for val in ll {
            assert_eq!(val, expected_val);
            expected_val -= 1;
        }
    }

    #[test]
    fn linked_list_iterator_iter() {
        let mut ll = LinkedList::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);

        let mut iter = ll.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn linked_list_iterator_iter_mut() {
        let mut ll = LinkedList::new();

        ll.push(1);
        ll.push(2);
        ll.push(3);

        let mut iter_mut = ll.iter_mut();
        assert_eq!(iter_mut.next(), Some(&mut 3));
        assert_eq!(iter_mut.next(), Some(&mut 2));
        assert_eq!(iter_mut.next(), Some(&mut 1));
        assert_eq!(iter_mut.next(), None);
    }
}
