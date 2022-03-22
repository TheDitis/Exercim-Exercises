use std::fmt::Display;
use std::iter::FromIterator;
use std::mem;

pub struct SimpleLinkedList<T: Clone + Display> {
    head: Option<Box<Node<T>>>,
}

#[derive(Clone)]
pub struct Node<T: Clone + Display> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Clone + Display> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut cur = &self.head;
        while cur.is_some() {
            count += 1;
            cur = &cur.as_ref().unwrap().next;
        }
        count
    }

    pub fn push(&mut self, item: T) {
        let mut cur = &mut self.head;
        if cur.is_none() {
            self.head = Some(Box::new(Node { data: item, next: None }));
            return;
        }
        // until cur has no next
        while cur.as_ref().unwrap().next.is_some() {
            cur = &mut cur.as_mut().unwrap().next;
        }
        // replace the last item's 'next' with new node with new value
        let old = mem::replace(&mut cur.as_mut().unwrap().next, Some(Box::new(Node { data: item, next: None })));
        mem::forget(old);
    }

    pub fn pop(&mut self) -> Option<T> {
        match &mut self.head {
            Some(_) => {
                // if head isn't none, set the current at the head
                let mut cur = &mut self.head;
                // while cur has a next and that next has a next, advance cur to the next node
                while cur.as_ref().unwrap().next.is_some() && cur.as_ref().unwrap().next.as_ref().unwrap().next.is_some() {
                    cur = &mut cur.as_mut().unwrap().next
                }
                let last_val: T;
                // if cur has a next (length is greater than one) get val from cur.next and clear node
                if cur.as_ref().unwrap().next.as_ref().is_some() {
                    last_val = cur.as_ref().unwrap().next.as_ref().unwrap().data.clone();
                    let old_mem = mem::replace(&mut cur.as_mut().unwrap().next, None);
                    mem::forget(old_mem);
                }
                // otherwise (length of 1) get val from self.head and clear self.head
                else {
                    last_val = cur.as_ref().unwrap().data.clone();
                    let old_mem = mem::replace(&mut self.head, None);
                    mem::forget(old_mem);
                }
                // return the value found
                Some(last_val)
            },
            // if head is empty, return None
            None => None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        let mut cur = &self.head;
        if cur.as_ref().is_some() {
            while cur.as_ref().unwrap().next.as_ref().is_some() {
                cur = &cur.as_ref().unwrap().next;
            }
            return cur.as_ref().map(|node| &node.data)
        }
        None
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut cur = self.head.clone();
        let mut prev = None;
        // while cur is some, set cur.next to prev and advance both cur & prev
        while let Some(mut cur_node) = cur.take() {
            let next = cur_node.next.take();
            cur_node.next = prev.take();
            prev = Some(cur_node);
            cur = next;
        }
        SimpleLinkedList { head: prev }
    }
}

impl<T: Clone + Display> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for data in iter {
            list.push(data);
        }
        list
    }
}

impl<T: Clone + Display> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        let mut output = Vec::new();
        while let Some(val) = _linked_list.pop() {
            output.push(val);
        }
        output.reverse();
        output
    }
}
