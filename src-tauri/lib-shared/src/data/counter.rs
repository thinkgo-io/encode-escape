#[derive(Clone, Default, Debug)]
pub struct Counter {
    pub value: isize,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { value: 0 }
    }
}

impl From<i32> for Counter {
    fn from(value: i32) -> Self {
        Counter {
            value: value as isize,
        }
    }
}

impl Counter {
    pub fn decrement(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
    }
    pub fn increment(&mut self) {
        self.value += 1;
    }
    pub fn reset(&mut self) {
        self.value = 0;
    }
}

mod tests {

    #[test]
    fn new_counter_is_0() {
        let counter = super::Counter::new();
        assert_eq!(counter.value, 0);
    }

    #[test]
    fn decrement_counter_is_negative_1() {
        let mut counter = super::Counter::new();
        counter.decrement();
        assert_eq!(counter.value, -1);
    }

    #[test]
    fn increment_counter_is_1() {
        let mut counter = super::Counter::new();
        counter.increment();
        assert_eq!(counter.value, 1);
    }
}
