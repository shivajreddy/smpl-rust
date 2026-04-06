pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
        }
    }
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = value as f64 / self.max as f64;
        if percentage_of_max >= 1.0 {
            self.messenger.send("ERROR: You are out of quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("WARNING: You used up 90%");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("WARNING: You used up 75%");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> Self {
            Self {
                sent_messages: RefCell::new(Vec::new()),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn test_foo() {
        let mock_messenger = MockMessenger::new();
        let max_size: usize = 100;
        let mut limit_tracker = LimitTracker::new(&mock_messenger, max_size);
        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
