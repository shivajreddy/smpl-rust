#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> String {
    "Hello, world!".to_string()
}

use tokio::time::{sleep, Duration};

#[get("/")]
fn test() -> String {
    println!("hello");
    sleep(Duration::from_secs(5));
    println!("hello, again");
    // do some async await stuff
    "test".to_string()
}

#[get("/")]
fn sql() -> String {
    // conncect to psql database
    "success".to_string()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/test", routes![test])
        .mount("/sql", routes![sql])
}
