use std::ops::Range;

fn main() {
    let mut v: Vec<i32> = vec![1, 2];
    let mut iter: Range<usize> = 0..v.len();
    let i1: usize = iter.next().unwrap();
    let n1: &i32 = &v[i1];
}
