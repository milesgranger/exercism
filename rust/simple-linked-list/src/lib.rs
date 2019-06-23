
pub struct Node<T> {
    next: Option<Box<Node<T>>>,
    data: T
}

impl<T: Copy> Node<T> {
    pub fn new(element: T) -> Self {
        Node { next: None, data: element }
    }
    pub fn push(&mut self, node: Node<T>) {
        match &mut self.next {
            Some(next) => next.push(node),
            None => self.next = Some(Box::new(node))
        }
    }
    pub fn pop(&mut self) -> Option<T> {

        let mut nn_node_has_node = self.next
            .as_ref()
            .map_or(false, |node| {
                node.next.is_some()
            });

        if nn_node_has_node {
            let val = self.next
                .as_ref()
                .unwrap()
                .data;
            self.next = None;
            Some(val)
        } else {
            None
        }


    }
}


pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T: Copy> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        unimplemented!()
    }

    /// Modify the node to link to a new node
    pub fn push(&mut self, element: T) {
        match &mut self.head {
            Some(ref mut head) => {
                head.push(Node::new(element))
            },
            None => self.head = Some(Box::new(Node::new(element)))
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match &mut self.head {
            Some(head) => head.pop(),
            None => None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        unimplemented!()
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        unimplemented!()
    }
}
