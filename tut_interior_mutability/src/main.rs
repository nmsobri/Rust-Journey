pub trait Messenger {
    fn send(&self, msg: &str);
}

struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    fn new(messenger: &'a T, max: usize) -> Self {
        Self {
            messenger,
            value: 0,
            max,
        }
    }

    fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 0.75 && percentage_of_max < 0.9 {
            self.messenger.send("Warning: You've used up over 75% of yourquota!");
        } else if percentage_of_max >= 0.9 && percentage_of_max < 1.0 {
            self.messenger.send("Urgent warning: You've used up over 90% ofyour quota!");
        } else if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        }
    }
}

impl<'a, T> Messenger for LimitTracker<'a, T> where T: Messenger {
    fn send(&self, msg: &str) {
        self.messenger.send(msg);
    }
}


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::*;
    use std::cell::RefCell;

    struct MockMessenger {
        messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> Self {
            Self {
                messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            let mut amut = self.messages.borrow_mut();
            let mut bmut = self.messages.borrow_mut();

            amut.push(msg.to_string());
            bmut.push(msg.to_string());
//            self.messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn send_over_75_percent_message() {
        use crate::*;

        let mock = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock, 100);

        limit_tracker.set_value(80);
        assert_eq!(1, mock.messages.borrow().len());
    }
}


