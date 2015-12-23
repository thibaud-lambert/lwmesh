use std::marker::PhantomData;
use std::ops::Index;
use std::ops::IndexMut;
use handle::*;

/// A growable list type, to store data for the different `Handle`.
///
/// `H` is the type of the Handle to acces the data. `D` is the type the data.
pub struct Property<H,D> {
    type_ : PhantomData<H>,
    data_ : Vec<D>,
}

impl<H, D : Clone> Property<H,D> {

    /// Constructs a new `Property<H,D>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::property::Property;
    /// use lwmesh::handle::Vertex;
    ///
    /// let prop = Property::<Vertex,u32>::new();
    /// ```
    pub fn new() -> Property<H,D>{
        Property {
            type_ : PhantomData,
            data_ : Vec::new()
        }
    }

    /// Reserve the minimun capacity to store at least `size` elements in the given `Property<H,D>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::property::Property;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut prop = Property::<Vertex,u32>::new();
    /// prop.reserve(42);
    /// assert!(prop.capacity() >= 42);
    /// ```
    pub fn reserve(&mut self, size : usize) {
        self.data_.reserve(size);
    }

    /// Returns the number of elements the given `Property<H,D>` can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::property::Property;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut prop = Property::<Vertex,u32>::new();
    /// assert!(prop.capacity() == 0);
    /// prop.reserve(42);
    /// assert!(prop.capacity() >= 42);
    /// ```
    pub fn capacity(& self) -> usize {
        self.data_.capacity()
    }

    /// Resize the given `Property<H,D>` to have exactly `size` elements.
    ///
    /// If the actual size of the property is less than `size`, elements with value `value` are
    /// added to the property so that the size matches. Otherwise elements are removed from the
    /// vector until the size matches.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::property::Property;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut prop = Property::<Vertex,u32>::new();
    /// prop.resize(13,17);
    /// ```
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

    /// Returns the number of elements in the `Property<H,D>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::property::Property;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut prop = Property::<Vertex,u32>::new();
    /// assert!(prop.len() == 0);
    /// prop.resize(13,17);
    /// assert!(prop.len() == 13)
    /// ```
    pub fn len(&self) -> usize() {
        self.data_.len()
    }
}

impl<T,D> Index<Handle<T> > for Property<Handle<T>,D> {
    type Output = D;

    fn index<'a>(&'a self, _index: Handle<T>) -> &'a D {
        return & self.data_[_index.idx().unwrap()];
    }
}

impl<T,D> IndexMut<Handle<T> > for Property<Handle<T> ,D> {
    fn index_mut<'a>(&'a mut self, _index: Handle<T>) -> &'a mut D {
        return &mut self.data_[_index.idx().unwrap()];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use handle::*;

    #[test]
    fn reserve() {
        let mut property = Property::<Vertex,u32>::new();
        property.reserve(16);
        assert!(16 <= property.capacity());
        property.reserve(33);
        assert!(33 <= property.capacity());
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
