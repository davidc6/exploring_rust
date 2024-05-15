use std::mem;

// #[derive(Debug)]
// enum ListNodeConnection {
//     Empty,
//     Elem(i32, Box<ListNodeConnection>),
// }

// A connection between nodes in the list
#[derive(Debug)]
enum ListNodeConnection {
    Empty,
    // To make ListNodeConnection representable, we need to insert indirection (i.e. Box<ListNodeConnection>).
    // This means storing a pointer to a value instead of the value itself.
    // Box allocates value on the heap but the pointer itself lives on the stack.
    // This way we know the size of Box.
    Filled(Box<ListNode>),
}

// Node
#[derive(Debug)]
struct ListNode {
    elem: i32,
    next_elem: ListNodeConnection,
}

// Since LinkedList is a single struct,
// the size of the struct is the same as the field
#[derive(Debug)]
pub struct LinkedList {
    head: ListNodeConnection,
}

impl Default for LinkedList {
    fn default() -> Self {
        Self::new()
    }
}

// :: is the namespace operator which allows us to choose enum variant
impl LinkedList {
    pub fn new() -> Self {
        LinkedList {
            head: ListNodeConnection::Empty,
        }
    }
}

impl LinkedList {
    pub fn push(&mut self, value: i32) {
        // std::mem::replace - moves src (second argument) into the references dest (first argument) and returns previous dest value
        // Move source (ListNodeConnection::Empty) into destination (self.head)
        // and return previous destination. Here self.head temporarily gets set to ListNodeConnection::Empty.
        let node = ListNode {
            elem: value,
            next_elem: mem::replace(&mut self.head, ListNodeConnection::Empty),
        };

        // Set head to Filled list with new node. We replace the previously set self.head with the new "head".
        self.head = ListNodeConnection::Filled(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, ListNodeConnection::Empty) {
            ListNodeConnection::Filled(node) => {
                self.head = node.next_elem;
                Some(node.elem)
            }
            ListNodeConnection::Empty => None,
        }
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut current = std::mem::replace(&mut self.head, ListNodeConnection::Empty);

        // Lift ListNodes out of their Boxes
        while let ListNodeConnection::Filled(mut node) = current {
            current = std::mem::replace(&mut node.next_elem, ListNodeConnection::Empty);
        }
    }
}

#[cfg(test)]
mod linked_list_one_tests {
    use super::*;

    #[test]
    fn linked_list_is_constructed_correctly() {
        let ll = LinkedList {
            head: ListNodeConnection::Filled(Box::new(ListNode {
                elem: 1,
                next_elem: ListNodeConnection::Empty,
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

        // Empty linked list
        assert_eq!(ll.pop(), None);
    }
}
