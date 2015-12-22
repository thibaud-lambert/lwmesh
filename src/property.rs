use std::marker::PhantomData;
use std::ops::Index;
use std::ops::IndexMut;
use handle::*;

pub struct Property<H,D> {
    type_ : PhantomData<H>,
    data_ : Vec<D>,
}

impl<T,D> Index<BaseHandle<T> > for Property<BaseHandle<T>,D> {
    type Output = D;

    fn index<'a>(&'a self, _index: BaseHandle<T>) -> &'a D {
        return & self.data_[_index.idx().unwrap()];
    }
}

impl<T,D> IndexMut<BaseHandle<T> > for Property<BaseHandle<T> ,D> {
    fn index_mut<'a>(&'a mut self, _index: BaseHandle<T>) -> &'a mut D {
        return &mut self.data_[_index.idx().unwrap()];
    }
}

impl<H, D : Clone> Property<H,D> {
    pub fn new() -> Property<H,D>{
        Property {
            type_ : PhantomData,
            data_ : Vec::new()
        }
    }

    pub fn reserve(&mut self, size : usize) {
        self.data_.reserve(size);
    }

    pub fn capacity(& self) -> usize {
        self.data_.capacity()
    }

    pub fn resize(&mut self, size : usize, value : D) {
        let n = self.data_.len();
        if n < size {
            self.data_.reserve(size);
            let m = size-n;
            for _ in 0..m {
                self.data_.push(value.clone());
            }
        } else {
            let m = n-size;
            for _ in 0..m {
                self.data_.pop();
            }
        }
    }

    pub fn len(&self) -> usize() {
        self.data_.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use handle::*;

    #[test]
    fn reserve() {
        let mut property = Property::<Vertex,u32>::new();
        let cap = 16;
        property.reserve(cap);
        assert!(cap == property.capacity());
    }

    #[test]
    fn resize() {
        let mut property = Property::<Vertex,u32>::new();
        let size = 5;
        let val = 42;
        property.resize(size,val);
        assert!(property.len() == size);
        property.resize(size-3,val);
        assert!(property.len() == size-3);
    }

    #[test]
    fn index() {
        let mut property = Property::new();
        let size = 8;
        let val = 42;
        property.resize(size,val);
        let v1 = Vertex::new(3);
        assert!(property[v1] == 42);
        property[v1] = 23;
        assert!(property[v1] == 23);

        let ref property_const = & property;
        let v2 = Vertex::new(0);
        assert!(property_const[v2] == 42);
    }

    #[test]
    #[should_panic]
    fn index_out_of_bound() {
        let mut property = Property::new();
        let size = 8;
        let val = 42;
        property.resize(size,val);
        let v1 = Vertex::new(8);
        property[v1] = 0;
    }

    #[test]
    #[should_panic]
    fn index_none() {
        let mut property = Property::new();
        let size = 8;
        let val = 42;
        property.resize(size,val);
        let v1 = Vertex::invalid();
        property[v1] = 0;
    }
}
