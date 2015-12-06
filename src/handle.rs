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
    fn base_handle_idx() {
        let invalid_handle = BaseHandle::invalid();
        let handle = BaseHandle::new(3);
        assert!(!invalid_handle.is_valid());
        assert!(handle.is_valid());
    }
}
