use super::StackOp;
use std::{
    ptr,
    sync::atomic::{AtomicPtr, Ordering::*},
};
pub struct Stack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }
}

impl<T> StackOp for Stack<T> {
    type Item = T;
    fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Acquire);
            if head == ptr::null_mut() {
                return None;
            } else {
                let next = unsafe { (*head).next };
                if self.head.compare_and_swap(head, next, Release) == head {
                    // Leak!!
                    return Some(unsafe { ptr::read(&(*head).data) });
                }
            }
        }
    }
    fn push(&self, t: T) {
        let n = Box::into_raw(Box::new(Node {
            data: t,
            next: ptr::null_mut(),
        }));
        loop {
            let head = self.head.load(Relaxed);
            unsafe {
                (*n).next = head;
            }
            if self.head.compare_and_swap(head, n, Release) == head {
                break;
            }
        }
    }
}
