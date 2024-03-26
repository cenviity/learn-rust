fn make_a_cloner(s_ref: &str) -> impl Fn() -> String {
    move || s_ref.to_string()
}

fn main() {
    let s_own = String::from("Hello world");
    let cloner = make_a_cloner(&s_own);
    drop(s_own);
    cloner();
}
