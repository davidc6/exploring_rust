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
}
