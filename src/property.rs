use handle::*;
use std::ops::Index;

pub struct Property<D> {
    data_ : Vec<D>,
}

impl<T,D> Index<BaseHandle<T> > for Property<D> {
    type Output = D;

    fn index<'a>(&'a self, _index: BaseHandle<T>) -> &'a D {
        return & self.data_[_index.idx().unwrap()];
    }
}

impl<D : Clone> Property<D> {
    pub fn reserve(&mut self, size : usize) {
        self.data_.reserve(size);
    }

    pub fn resize(&mut self, size : usize, value : D) {
        //self.data_.resize(size,value);
    }
}
