#![allow(dead_code)]

use gui::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
    }
}

fn main() {
    let components = vec![
        Box::new(SelectBox {
            width: 75,
            height: 10,
            options: vec![
                String::from("Yes"),
                String::from("Maybe"),
                String::from("No"),
            ],
        }) as Box<dyn Draw>,
        Box::new(Button {
            width: 50,
            height: 10,
            label: String::from("OK"),
        }),
    ];
    let screen = Screen { components };

    screen.run();
}
