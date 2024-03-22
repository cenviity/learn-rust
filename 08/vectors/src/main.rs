use std::slice::Iter;

fn main() {
    let mut v: Vec<i32> = vec![1, 2];

    let mut iter: Iter<'_, i32> = v.iter();

    let n1: &i32 = iter.next().unwrap();
    let n2: &i32 = iter.next().unwrap();
    let end: Option<&i32> = iter.next();
}
