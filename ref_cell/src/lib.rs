trait Messenger {
    fn send(&self, msg: &str) {}
}

struct Limiter<'a, T: Messenger> {
    messenger: &'a T,
    // value: usize,
    max: usize
}

impl<'a, T> Limiter<'a, T>
where 
    T: Messenger
{
    fn new(messenger: &T, max_value: usize) -> Limiter<T> {
        Limiter {
            messenger,
            // value: 0,
            max: max_value
        }
    }

    fn set_value(&self, value: usize) {
        let percentage_of_max = value as f64 / self.max as f64;

        match percentage_of_max {
            x if x >= 1.0 => self.messenger.send("over quota"),
            x if x >= 0.9 => self.messenger.send("90%"),
            x if x >= 0.75 => self.messenger.send("75%"),
            _ => self.messenger.send("within quota")
        }

    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }


    impl MockMessenger {
        fn new() -> Self {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            // self.sent_messages.borrow_mut().push(String::from(msg));
            let mut one_mut_borrow = self.sent_messages.borrow_mut();
            let mut second_mut_borrow = self.sent_messages.borrow_mut();

            one_mut_borrow.push(String::from(msg));
            second_mut_borrow.push(String::from(msg));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let limiter = Limiter::new(&mock_messenger, 100);

        limiter.set_value(100);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
