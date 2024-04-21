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
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let max_percentage = self.value as f64 / self.max as f64;

        if max_percentage >= 1.0 {
            self.messenger.send("Error: Over quota!");
        } else if max_percentage >= 0.9 {
            self.messenger.send("Warning: Over 90% quota");
        } else if max_percentage >= 0.75 {
            self.messenger.send("Warning: Over 75% quota");
        }
    }
}

#[cfg(test)]
mod messenger_tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        send_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> Self {
            MockMessenger {
                // send_messages: vec![], -> this won't work since we want to mutate send_messages
                send_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.send_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn send_75_percentage_warning_sent() {
        let mock = MockMessenger::new();

        let mut tracker = LimitTracker::new(&mock, 100);

        tracker.set_value(80);
        let messages = mock.send_messages.into_inner();
        assert_eq!(messages.len(), 1);
    }
}
