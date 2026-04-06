use minigrep::{read_file_contents, search};
use std::env;

fn main() {
    println!("Minigrep");

    let mut args = env::args();
    args.next(); // skip the program name

    let search_query = args.next().expect("Search Query can't be empty");
    let file_name = args.next().expect("File name can't be empty");

    println!("Given Search-query:{search_query}");
    println!("Given File-name:{file_name}");

    // Check if file is valid
    let data;
    match read_file_contents(&file_name) {
        Ok(d) => data = d,
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => println!("{file_name} is not found"),
                _ => println!("{e}"),
            }
            return;
        }
    }

    let res = search(&search_query, &data);
    println!("res: {res:?}");
}
