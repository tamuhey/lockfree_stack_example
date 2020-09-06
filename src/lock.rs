use super::StackOp;
use std::mem;
use std::sync::{Arc, Mutex};

pub struct Stack<T> {
    head: Mutex<Option<Box<Node<T>>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            head: Mutex::new(None),
        }
    }
}

impl<T> StackOp for Stack<T> {
    type Item = T;
    fn pop(&self) -> Option<T> {
        let mut head = self.head.lock().unwrap();
        let Node { data, next } = match head.take() {
            None => return None,
            Some(node) => *node,
        };
        if let Some(next) = next {
            head.replace(next);
        }
        Some(data)
    }
    fn push(&self, t: T) {
        let mut head = self.head.lock().unwrap();
        let next = head.take();
        head.replace(Box::new(Node { data: t, next }));
    }
}
