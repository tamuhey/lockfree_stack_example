#![feature(test)]
mod leak;
mod lock;
mod treiber;

extern crate test;

trait StackOp {
    type Item;
    fn pop(&self) -> Option<Self::Item>;
    fn push(&self, t: Self::Item);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;
    use test::Bencher;
    const n: usize = 100000;

    fn push_pop(stack: &impl StackOp<Item = usize>) {
        for i in 0..n {
            stack.push(i);
        }
        for _ in 0..n {
            stack.pop();
        }
    }

    #[bench]
    fn leak(b: &mut Bencher) {
        let s = Arc::new(leak::Stack::new());
        b.iter(|| {
            let s2 = Arc::clone(&s);
            let th = thread::spawn(move || push_pop(&*s2));
            push_pop(&*s);
            th.join().unwrap();
            assert_eq!(s.pop(), None);
        });
    }
    #[bench]
    fn lock(b: &mut Bencher) {
        let s = Arc::new(lock::Stack::new());
        b.iter(|| {
            let s2 = Arc::clone(&s);
            let th = thread::spawn(move || push_pop(&*s2));
            push_pop(&*s);
            th.join().unwrap();
            assert_eq!(s.pop(), None);
        });
    }
    #[bench]
    fn treiber(b: &mut Bencher) {
        let s = Arc::new(treiber::Stack::new());
        b.iter(|| {
            let s2 = Arc::clone(&s);
            let th = thread::spawn(move || push_pop(&*s2));
            push_pop(&*s);
            th.join().unwrap();
            assert!(s.pop().is_none());
        });
    }
}
