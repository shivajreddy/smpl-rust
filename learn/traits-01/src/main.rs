#![allow(unused)]

use std::ops::Deref;

// define Describable trait
trait Describable {
    fn describe(&self) -> String;
    fn shout(&self) -> String {
        self.describe().to_uppercase()
    }
}

// define Countable
trait Countable {
    fn count(&self) -> u32;
}

struct Animal {
    name: String,
    legs: u8,
}

impl Animal {
    fn new(name: String, legs: u8) -> Self {
        Animal { name, legs }
    }
}

impl Describable for Animal {
    fn describe(&self) -> String {
        format!("Name:{} Legs:{}", self.name, self.legs)
    }
}
impl Countable for Animal {
    fn count(&self) -> u32 {
        self.legs as u32
    }
}
impl Printable for Animal {
    fn print(&self) {
        self.describe();
    }
}

struct Plant<'x> {
    name: &'x str,
    is_flowering: bool,
}

impl<'t> Plant<'t> {
    fn new(name: &'t str, is_flowering: bool) -> Self {
        Plant { name, is_flowering }
    }
}

impl<'x> Describable for Plant<'x> {
    fn describe(&self) -> String {
        format!("Name:{} is_flowering:{}", self.name, self.is_flowering)
    }
}
impl<'x> Countable for Plant<'x> {
    fn count(&self) -> u32 {
        if self.is_flowering { 1 } else { 0 }
    }
}

fn take_dyn(x: &dyn Describable) {}

// fn print_descriptable(item: &impl Describable) {
// fn print_descriptable<T: Describable>(item: &T) {
fn print_descriptable<T>(item: &T)
where
    T: Describable,
{
    println!("{}", item.describe());
}

fn describe_and_count<T>(item: &T)
where
    T: Describable + Countable,
{
    println!("{} count:{}", item.describe(), item.count());
}

fn give() -> impl Describable + Countable {
    let a = Animal::new(String::from("doggo"), 4);
    a
}

fn give2() -> impl Describable {
    let _a = Animal::new(String::from("doggo"), 4);
    let _p = Plant::new("sokoia", false);
    _a
}

fn give3() -> Box<dyn Describable> {
    let a = Animal::new(String::from("doggo"), 4);
    let p = Plant::new("sokoia", false);
    Box::new(p)
}

fn give4() -> impl Countable {
    let a = Animal::new(String::from("dogggggooo"), 4);
    let p = Plant::new("alsdjkfjsdf", true);
    p
    // Box::new(p)
}

// fn give5() -> impl Countable { // will not compile
fn give5() -> Box<dyn Countable> {
    let a = Animal::new(String::from("dogggggooo"), 4);
    let a = Box::new(a);
    let p = Plant::new("alsdjkfjsdf", true);
    let p = Box::new(p);
    if p.is_flowering { p } else { a }
}

// super traits
trait Printable: Describable {
    fn print(&self);
}

fn main() {
    println!("Hello, traits!");

    let a = Animal::new(String::from("doggo"), 4);
    let p = Plant::new("sokoia", false);

    println!("{}", a.describe());
    println!("{}", a.shout());
    print_descriptable(&a);
    println!("{}", p.describe());
    println!("{}", p.shout());
    print_descriptable(&p);

    describe_and_count(&a);
    describe_and_count(&p);

    // vector that holds items that implement a trait
    // let v = vec![a, p]; // wont compile, macro expands to set the type of collection to the type of the first item
    // let v: Vec<Box<dyn Describable>> = vec![Box(a)];
    let box_a = Box::new(a);
    let box_p = Box::new(p);
    let v: Vec<Box<dyn Describable>> = vec![box_a, box_p];

    for item in v {
        println!("{}", item.describe());
        // describe_and_count(item.deref());
        // describe_and_count(&item);
        // let a = item.shout();
        // take_dyn(item.deref());
        take_dyn(&*item);
    }

    //
}

struct Rock {
    weight: i32,
}

// if rock doesnt implement Describable then would get trait bound `Rock: Describable` is not satisfied
impl Printable for Rock {
    fn print(&self) {
        println!("{}", self.describe());
    }
}

impl Describable for Rock {
    fn describe(&self) -> String {
        format!("weight{}", self.weight)
    }
}
