fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("the first word is: {}", &s[..word]);
}

fn first_word(s: &String) -> usize {
    for (i, item) in s.bytes().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
