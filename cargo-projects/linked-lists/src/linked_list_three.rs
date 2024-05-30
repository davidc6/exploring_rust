use std::rc::Rc;

pub struct LinkedList<T> {
    head: LinkedListNodeConnection<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

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
}

struct LinkedListNode<T> {
    current_node: T,
    next_node: LinkedListNodeConnection<T>,
}

type LinkedListNodeConnection<T> = Option<Rc<LinkedListNode<T>>>;
