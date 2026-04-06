fn main() {
    // with out iterator
    let words = vec!["hello", "world", "foo", "bar"];
    let mut result = Vec::new();
    for word in &words {
        if word.len() > 3 {
            result.push(word.to_uppercase());
        }
    }
    println!("{result:?}");

    // with iterator
    let res: Vec<_> = words
        .iter()
        .filter(|word| word.len() > 3)
        .map(|word| word.to_uppercase())
        .collect();
    println!("{res:?}");
}

struct Countdown {
    val: u32,
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.val == 0 {
            return None;
        }
        let val = self.val;
        self.val -= 1;
        return Some(val);
    }
}

#[test]
fn test_countdown() {
    let countdown = Countdown { val: 5 };
    let res: Vec<_> = countdown.into_iter().collect();
    assert_eq!(vec![5, 4, 3, 2, 1], res);

    let countdown = Countdown { val: 9 };
    let res: Vec<_> = countdown.into_iter().collect();
    assert_eq!(vec![9, 8, 7, 6, 5, 4, 3, 2, 1], res);
}
