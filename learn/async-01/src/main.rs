#![allow(unused)]

fn main() {
    println!("Hello, world!");
    let res1 = foo1();
    let res2 = foo2();
    // println!("{:?}", res);
}

async fn foo1() -> usize {
    println!("foo1");
    0
}

fn foo2() -> impl Future<Output = usize> {
    async {
        println!("foo2");
        0
    }
}
