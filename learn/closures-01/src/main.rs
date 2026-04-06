#![allow(unused)]

fn apply_twice(val: u32, f: impl Fn(u32) -> u32) -> u32 {
    0
}

fn apply_twice(val: u32, f: &dyn Fn(u32) -> u32) -> u32 {
    f(f(val))
}

fn apply_twice3(val: u32, f: impl Fn(u32) -> u32) -> u32 {
    f(f(val))
}

fn make_adder(val: u32) -> impl Fn(u32) -> u32 {
    let res = move |n: u32| n + val;
    res
}

fn make_adder_original(x: i32) -> impl Fn(i32) -> i32 {
    let res = move |n: i32| (n + x);
    println!("trying to use {x}");
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_twice() {
        // apply_twice(10, |20| 22);
        assert_eq!(apply_twice3(5, &|x| x + 1), 7);
        assert_eq!(apply_twice3(5, &|x| x * 2), 20);
    }

    #[test]
    fn test_make_adder() {
        let add5 = make_adder(5);
        assert_eq!(add5(3), 8);
        assert_eq!(add5(10), 15);
    }
}

// write a fn that expects a closure

struct MyVec<T> {
    values: Vec<T>,
}

impl<T> MyVec<T> {
    fn new() -> Self {
        Self { values: Vec::new() }
    }

    fn push(&mut self, val: T) {
        self.values.push(val);
    }

    fn add_to_items(&self, f: impl Fn(&T)) {
        for val in &self.values {
            f(val)
        }
    }

    fn count_2<F>(self, f: F) -> T
    where
        F: Fn() -> T,
    {
        f()
    }
}

fn main() {
    println!("CLOSUERS");

    let mut v = MyVec::<u8>::new();
    v.push(10);
    v.push(20);
    v.push(30);

    v.add_to_items(move |item| println!("{}", item + 5 as u8));

    let c = move |item: &u8| println!("{}", item + 5);
    v.add_to_items(c);
}
