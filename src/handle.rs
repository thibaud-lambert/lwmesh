pub struct BaseHandle {
    index_ : Option<u32>,
}

impl BaseHandle {
    pub fn invalid() -> BaseHandle {
        BaseHandle {
            index_ : None,
        }
    }

     pub fn new(idx : u32) -> BaseHandle {
            BaseHandle {
                index_ : Some(idx),
        }
    }

    pub fn idx(&self) -> Option<u32> {
        self.index_
    }

    pub fn reset(&mut self) {
        self.index_ = None;
    }

    pub fn is_valid(&self) -> bool {
        self.index_.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_handle_invalid() {
        let handle = BaseHandle::invalid();
        assert!(!handle.is_valid());
        assert!(handle.idx().is_none());
    }

    #[test]
    fn base_handle_idx() {
        let idx = 42;
        let handle = BaseHandle::new(idx);
        assert!(handle.is_valid());

        assert!(handle.idx().is_some());
        assert!(handle.idx().unwrap() == idx);
    }

    #[test]
    fn base_handle_reset() {
        let idx = 42;
        let mut handle = BaseHandle::new(idx);
        assert!(handle.is_valid());

        handle.reset();
        assert!(!handle.is_valid());
    }
}
