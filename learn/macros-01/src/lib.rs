#![allow(unused)]

macro_rules! renamei32 {
    ($arg1: ident) => {
        type $arg1 = i32;
    };
}

renamei32!(Wo);
// type wo = i32;

enum Color {
    Red,
    Green,
    Custom(u8, u8, u8),
}

fn foo() {
    let c = Color::Custom(10, 20, 30);

    if let Color::Custom(a, b, c) = c {}
    match c {
        Color::Red => {}
        Color::Green => {}
        Color::Custom(a, b, c) => {
            a + b + c;
        }
        _ => {}
    }
}
