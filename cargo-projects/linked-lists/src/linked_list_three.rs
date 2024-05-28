use std::rc::Rc;

pub struct List<T> {
    head: ListNodeConnection<T>,
}

struct ListNode<T> {
    val: T,
    next_val: ListNodeConnection<T>,
}

type ListNodeConnection<T> = Option<Rc<ListNode<T>>>;
