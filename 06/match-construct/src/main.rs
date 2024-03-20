fn main() {
    let opt: Option<String> = Some(String::from("Hello world"));

    match &opt {
        Some(s) => println!("Some: {}", s),
        None => println!("None!"),
    }

    println!("{:?}", opt);
}
