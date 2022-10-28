#[derive(Clone, Default, Debug)]
pub struct RotatingCounter {
    pub value: usize,
    pub max: usize,
}

impl RotatingCounter {
    pub fn new(max: usize) -> Self {
        RotatingCounter { value: 0, max }
    }

    pub fn increment(&mut self) {
        self.value += 1;
        if self.value > self.max {
            self.value = 0;
        }
    }
}

mod tests {
    #[test]
    fn new_counter_is_0() {
        let counter = super::RotatingCounter::new(10);
        assert_eq!(counter.value, 0);
    }

    #[test]
    fn increment_counter_is_1() {
        let mut counter = super::RotatingCounter::new(10);
        counter.increment();
        assert_eq!(counter.value, 1);
    }

    #[test]
    fn increment_counter_is_0() {
        let mut counter = super::RotatingCounter::new(10);
        for _ in 0..10 {
            counter.increment();
        }
        assert_eq!(counter.value, 0);
    }
}
