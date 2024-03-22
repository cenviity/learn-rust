fn main() {}

fn dup_in_place(v: &mut Vec<i32>) {
    for n_ref in v.iter() {
        v.push(*n_ref);
    }
}
