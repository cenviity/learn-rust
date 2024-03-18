fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    for (i, item) in s.bytes().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
