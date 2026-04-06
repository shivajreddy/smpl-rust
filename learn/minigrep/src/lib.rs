pub fn read_file_contents(file_name: &str) -> Result<String, std::io::Error> {
    let data = std::fs::read_to_string(file_name)?;
    Ok(data)
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut res: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            res.push(line);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let query = "are";
        let contents = r#"
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.
How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
        "#;
        let res = search(query, contents);
        let expected = vec!["I'm nobody! Who are you?", "Are you nobody, too?"];
        assert_eq!(res, expected);
    }
}
