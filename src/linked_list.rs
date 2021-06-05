use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<LinkedListNode<T>>>>,
    tail: Option<Rc<RefCell<LinkedListNode<T>>>>,
}

#[derive(Debug)]
pub struct LinkedListNode<T> {
    pub value: T,
    prev: Option<Weak<RefCell<LinkedListNode<T>>>>,
    next: Option<Rc<RefCell<LinkedListNode<T>>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    fn set_last_node(&mut self, node: &Rc<RefCell<LinkedListNode<T>>>) {
        if let Some(tail) = self.tail.as_ref() {
            node.borrow_mut().prev = Some(Rc::downgrade(tail));
            tail.borrow_mut().next = Some(Rc::clone(node));
        } else if let None = self.head {
            self.head = Some(Rc::clone(node));
        }
        self.tail = Some(Rc::clone(node));
    }

    pub fn push(&mut self, value: T) -> Rc<RefCell<LinkedListNode<T>>> {
        let node = Rc::new(RefCell::new(LinkedListNode {
            value,
            prev: None,
            next: None,
        }));
        self.set_last_node(&node);
        node
    }

    pub fn move_to_back(&mut self, node: &Rc<RefCell<LinkedListNode<T>>>) {
        let prev = node.borrow_mut().prev.take();
        let next = node.borrow_mut().next.take();
        match (prev, next) {
            (Some(prev), Some(next)) => {
                prev.upgrade().unwrap().borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());
                self.set_last_node(node);
            }
            (Some(_), None) => {
                // do nothing
            }
            (None, Some(next)) => {
                next.borrow_mut().prev = None;
                self.head = Some(next);
                self.set_last_node(node);
            }
            (_, _) => (),
        }
    }

    pub fn remove_front(&mut self) -> Option<Rc<RefCell<LinkedListNode<T>>>> {
        if let Some(head) = self.head.take() {
            if let Some(next) = head.borrow().next.as_ref() {
                next.borrow_mut().prev = None;
            } else {
                self.tail = None;
            }
            self.head = head.borrow().next.clone();
            Some(head)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use super::*;

    #[test]
    fn test_linked_list() {
        let mut list = LinkedList::new();
        let rc1 = list.push(1);
        let rc2 = list.push(2);
        let rc3 = list.push(3);
        let rc4 = list.push(4);
        let rc5 = list.push(5);
        list.move_to_back(&rc3);
        list.move_to_back(&rc1);
        list.push(6);
        assert_eq!(
            list.remove_front().map(|rc| Rc::as_ptr(&rc)),
            Some(Rc::as_ptr(&rc2))
        );
        assert_eq!(
            list.remove_front().map(|rc| Rc::as_ptr(&rc)),
            Some(Rc::as_ptr(&rc4))
        );
        assert_eq!(
            list.remove_front().map(|rc| Rc::as_ptr(&rc)),
            Some(Rc::as_ptr(&rc5))
        );
        assert_eq!(
            list.remove_front().map(|rc| Rc::as_ptr(&rc)),
            Some(Rc::as_ptr(&rc3))
        );
        assert_eq!(
            list.remove_front().map(|rc| Rc::as_ptr(&rc)),
            Some(Rc::as_ptr(&rc1))
        );
        if let Some(node) = list.remove_front() {
            assert_eq!(node.borrow().value, 6);
        }
        assert_eq!(list.remove_front().map(|rc| Rc::as_ptr(&rc)), None);
    }
}
