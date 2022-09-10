#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    // Cons List
    use crate::List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("{:?}", list);

    // Custom Box
    #[derive(Debug)]
    struct Node {
        data: i32,
        next: Option<Box<Node>>,
    }

    impl Node {
        fn new(num: i32) -> Node {
            Node {
                data: num,
                next: None,
            }
        }
        
        fn next_node(&mut self, node: Node) {
            self.next = Some(Box::new(node));
        }
    }

    let mut node_one = Node::new(1);
    let node_two = Node::new(2);
    node_one.next_node(node_two);
    println!("{:?}", node_one);
}
