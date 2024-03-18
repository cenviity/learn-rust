fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);

    s.clear();
}

fn first_word(s: &str) -> &str {
    for (i, item) in s.bytes().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s
}
