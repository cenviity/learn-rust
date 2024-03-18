fn main() {
    let s = String::from("hello world");

    println!("{}", first_word(&s));
}

fn first_word(s: &String) -> usize {
    for (i, item) in s.bytes().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
