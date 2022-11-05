pub struct First {
    first: bool,
}

impl First {
    pub fn new() -> Self {
        First { first: true }
    }
    pub fn is_first(&mut self) -> bool {
        if !self.first {
            return false;
        }

        self.first = false;
        return true;
    }

    pub fn not_first(&mut self) -> bool {
        return !self.is_first();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn new_first_is_first_should_be_true() {
        let mut first = super::First::new();
        assert!(&first.is_first());
    }
    #[test]
    fn new_first_not_first_should_be_false() {
        let mut first = super::First::new();
        assert!(!&first.not_first());
    }
    #[test]
    fn after_first_is_first_then_is_first_should_be_false() {
        let mut first = super::First::new();
        first.is_first();
        assert!(!&first.is_first());
    }
    #[test]
    fn after_first_not_first_then_is_first_should_be_true() {
        let mut first = super::First::new();
        first.is_first();
        assert!(&first.not_first());
    }
}