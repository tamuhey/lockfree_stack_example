#![feature(test)]
mod leak;
mod lock;

extern crate test;

trait StackOp {
    type Item;
    fn pop(&mut self) -> Option<Self::Item>;
    fn push(&mut self, t: Self::Item);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    const n: usize = 100000;

    fn push_pop(stack: &mut impl StackOp<Item = usize>) {
        for i in 0..n {
            stack.push(i);
        }
        for _ in 0..n {
            stack.pop();
        }
    }
    #[bench]
    fn leak(b: &mut Bencher) {
        let mut s = leak::Stack::new();
        b.iter(|| push_pop(&mut s));
    }
}
