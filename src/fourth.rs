use std::cell::RefCell;
use std::rc::Rc;

pub struct List<T>{
    head: Link<T>,
    tail: Link<T>
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T>{
    elem: T,
    prev: Link<T>,
    next: Link<T>
}

impl<T> Node<T>{
    fn new(elem: T) -> Rc<RefCell<Node<T>>>{
        Rc::new(
            RefCell::new(
                Node{
                    prev: None,
                    next: None,
                    elem: elem
                }
            )
        )
    }
}

impl<T> List<T>{
    fn new() ->Self{
        List{
            head: None,
            tail: None
        }
    }

    pub fn push_front(&mut self, elem: T){
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                old_head.prev = Some(new_head.clone());
                new_head.next = Some(old_head);
                self.head = Some(new_head)
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
}