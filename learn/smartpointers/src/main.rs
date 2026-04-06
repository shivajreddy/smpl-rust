#![allow(unused)]

/*
ref:
 - 1 mutable ref
 - infinite immutabe ref


Box<T>
 - owner: single
 - borrow: immutable, mutable
 - borrow check: compile time
Rc<T>
 - owner: multiple
 - borrow(data inside Rc): immutable
 - borrow check: compile time
RefCell<T>
 - owner: single
 - borrow(data inside RefCell): immutable, mutable
 - borrow check: run time
*/

use std::rc::Rc;
use std::{cell::RefCell, rc};

enum Node {
    Yes(i32, Rc<Node>),
    No,
}

use Node::{No, Yes};

struct Person {}

fn main() {
    let mut p1 = Person {};
    let mut p2 = Person {};
    let mut x = &mut p1;
    x = &mut p2;
    *x = p1;
    // so the moment we use deref, we are not talking about the left side
    // variable, we are talking about the right side variable, in this case
    // it is p2. so this line essentialy is saying p2 = p1;

    let mut a = Rc::new(Yes(3, Rc::new(Yes(2, Rc::new(No)))));
    let b = Yes(4, Rc::clone(&a));
    let c = Yes(4, Rc::clone(&a));

    let mut x = &a;
    let mut y = &mut a;
    *y = Rc::new(No);

    let mut x = 20;
    let y = &mut x;
    *y = 21;

    let state = State::new(String::from("something cool"));
    let rc_state = Rc::new(RefCell::new(state));
    println!("{:?} {}", rc_state, Rc::strong_count(&rc_state));
    let owner1 = Rc::clone(&rc_state);
    println!("{:?} {}", rc_state, Rc::strong_count(&rc_state));
    let owner2 = Rc::clone(&rc_state);
    println!("{:?} {}", rc_state, Rc::strong_count(&rc_state));

    owner1.borrow_mut().data = String::from("woaw");
    println!("{:?} {}", rc_state, Rc::strong_count(&rc_state));
    owner2.borrow_mut().data = String::from("this is crazy cool");
    println!("{:?} {}", rc_state, Rc::strong_count(&rc_state));

    let mut borrow1 = owner1.borrow_mut();
    drop(borrow1);
    let mut borrow2 = owner1.borrow_mut();

    let mut heapdata = Box::new(String::from("heap"));
    // heapdata = String::from("new box value");
    // let mut x = &mut heapdata;
    // *x =
}

#[derive(Debug)]
struct State {
    data: String,
}
impl State {
    fn new(data: String) -> Self {
        Self { data }
    }
}
