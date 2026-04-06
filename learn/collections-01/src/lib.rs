#![allow(unused)]

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn create_hashmap() {
        let mut hm: HashMap<String, u32> = HashMap::new();
        let s = "hello world hello rust hello world";
        for word in s.split(' ') {
            *hm.entry(word.to_string()).or_insert(0) += 1;
        }

        // for (k, v) in hm.iter() {
        //     println!("{}:{}", k, v);
        // }

        let mut expected = HashMap::new();
        expected.insert("hello".to_string(), 3);
        expected.insert("world".to_string(), 2);
        expected.insert("rust".to_string(), 1);

        assert_eq!(hm, expected);
    }
}
