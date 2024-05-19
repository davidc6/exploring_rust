// Represents a connection between nodes in the list.
//
// To make ListNodeConnection representable, we need to insert indirection (i.e. Box<ListNodeConnection>).
// This means storing a pointer to a value instead of the value itself.
// Box allocates value on the heap but the pointer itself lives on the stack.
// This way we know the size of Box.
type ListNodeConnection = Option<Box<ListNode>>;

// Linked List's Node that is holding a value and link to the next node (if any)
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
            head: ListNodeConnection::None,
        }
    }
}

impl LinkedList {
    pub fn push(&mut self, value: i32) {
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

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next_elem;
            node.elem
        })
    }
}

impl Drop for LinkedList {
    fn drop(&mut self) {
        let mut current = self.head.take();

        // Lift ListNodes out of their Boxes
        while let ListNodeConnection::Some(mut node) = current {
            current = node.next_elem.take();
        }
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
}
