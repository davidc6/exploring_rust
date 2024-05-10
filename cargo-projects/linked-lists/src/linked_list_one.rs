use std::mem;

#[derive(Debug)]
pub enum List {
    Empty,
    // To make List representable, we need to insert indirection (i.e. Box<List>).
    // This means storing a pointer to a value instead of the value itself.
    // Box allocates value on the heap but the pointer itself lives on the stack.
    // This way we know the size of Box.
    Elem(i32, Box<List>),
}

// Since ListThree is a single struct,
// the size of the struct is the same as the field
#[derive(Debug)]
pub struct ListThree {
    head: ListTwo,
}

// :: is the namespace operator which allows us to choose enum variant
impl ListThree {
    pub fn new() -> Self {
        ListThree {
            head: ListTwo::Empty,
        }
    }
}

impl ListThree {
    pub fn push(&mut self, value: i32) {
        // Move source (ListTwo::Empty) into destination (self.head)
        // and return previous destination. Here self.head temporarily gets set to ListTwo::Empty.
        let node = ListNode {
            elem: value,
            next_elem: std::mem::replace(&mut self.head, ListTwo::Empty),
        };

        // Set head to Filled list with new node. We replace the previously set self.head with the new "head".
        self.head = ListTwo::Filled(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<i32> {
        match std::mem::replace(&mut self.head, ListTwo::Empty) {
            ListTwo::Filled(node) => {
                self.head = node.next_elem;
                Some(node.elem)
            }
            ListTwo::Empty => None,
        }
    }
}

#[derive(Debug)]
enum ListTwo {
    Empty,
    Filled(Box<ListNode>),
}

#[derive(Debug)]
struct ListNode {
    elem: i32,
    next_elem: ListTwo,
}

#[cfg(test)]
mod linked_list_one_tests {
    use crate::linked_list_one::{List, ListThree};

    use super::{ListNode, ListTwo};

    #[test]
    fn list_constructed_correctly() {
        let ll = List::Elem(1, Box::new(List::Elem(2, Box::new(List::Empty))));

        assert!(matches!(ll, List::Elem(1, _)));
    }

    #[test]
    fn list_two_constructed_correctly() {
        let ll = ListThree {
            head: ListTwo::Filled(Box::new(ListNode {
                elem: 1,
                next_elem: ListTwo::Empty,
            })),
        };

        assert!(matches!(ll, ListThree { head: _ }));
    }

    #[test]
    fn popping_list_two_returns_valid_value() {
        let mut ll = ListThree {
            head: ListTwo::Filled(Box::new(ListNode {
                elem: 1,
                next_elem: ListTwo::Empty,
            })),
        };

        ll.push(2);

        assert_eq!(ll.pop(), Some(2));
    }
}
