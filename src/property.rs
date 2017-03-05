use std::marker::PhantomData;
use std::any::Any;
use handle::*;
use std::ops::Index;
use std::ops::IndexMut;

pub trait ResizableVec {
    fn len(&self) -> usize;
    fn reserve(&mut self, size : usize);
    fn capacity(&self) -> usize;
    fn push(&mut self);
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
}

pub struct PropertyVec<H : 'static, D : 'static> {
    default_ : D,
    data_ : Vec<D>,
    handle_ : PhantomData<H>,
}

impl<T : 'static, D : 'static> PropertyVec<Handle<T>, D> {
    pub fn new(default_value : D) -> PropertyVec<Handle<T>, D> {
        PropertyVec {
            default_ : default_value,
            data_ : Vec::new(),
            handle_ : PhantomData,
        }
    }
}

impl<T : 'static, D : Clone> ResizableVec for PropertyVec<Handle<T>, D> {
    fn reserve(&mut self, size : usize) {
        self.data_.reserve(size);
    }

    fn len(& self) -> usize {
        self.data_.len()
    }

    fn capacity(& self) -> usize {
        self.data_.capacity()
    }

    fn push(&mut self) {
        self.data_.push(self.default_.clone());
    }

    fn as_any(&self) -> &Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
}

impl<T,D> Index<Handle<T>> for PropertyVec<Handle<T>, D> {
    type Output = D;

    fn index(&self, h: Handle<T>) -> &D {
        &self.data_[h.idx()]
    }
}

impl<T,D> IndexMut<Handle<T>> for PropertyVec<Handle<T>, D> {
    fn index_mut(&mut self, h: Handle<T>) -> &mut D {
        &mut self.data_[h.idx()]
    }
}

/// A growable list type, to store data for the different `Handle`.
///
/// `H` is the type of the Handle to acces the data.
pub struct PropertyContainer<H> {
    handle_ : PhantomData<H>,
    parrays_ : Vec<(& 'static str,Box<ResizableVec>)>,
    size_ : usize,
    capacity_ : usize
}

impl<T : 'static> PropertyContainer<Handle<T>> {
    /// Constructs a new `PropertyContainer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::property::PropertyContainer;
    /// use lwmesh::handle::Vertex;
    ///
    /// let pcontainer = PropertyContainer::<Vertex>::new();
    /// ```
    pub fn new() -> PropertyContainer<Handle<T>>{
        PropertyContainer {
            handle_ : PhantomData,
            parrays_ : Vec::new(),
            size_ : 0,
            capacity_ : 0
        }
    }

    /// Returns the number of elements in the `PropertyContainer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::property::PropertyContainer;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut pcontainer = PropertyContainer::<Vertex>::new();
    /// assert!(pcontainer.len() == 0);
    /// pcontainer.push();
    /// assert!(pcontainer.len() == 1);
    /// ```
    pub fn len(&self) -> usize() {
        self.size_
    }

    /// Reserve the minimun capacity to store at least `size` elements in the given `PropertyContainer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::property::PropertyContainer;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut pcontainer = PropertyContainer::<Vertex>::new();
    /// pcontainer.reserve(42);
    /// assert!(pcontainer.capacity() >= 42);
    /// ```
    pub fn reserve(&mut self, size : usize) {
        self.capacity_ = size;
        for &mut(_, ref mut b) in self.parrays_.iter_mut() {
            b.reserve(size);
        }
    }

    /// Returns the number of elements the given `PropertyContainer` can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::property::PropertyContainer;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut pcontainer = PropertyContainer::<Vertex>::new();
    /// assert!(pcontainer.capacity() == 0);
    /// pcontainer.reserve(42);
    /// assert!(pcontainer.capacity() >= 42);
    /// ```
    pub fn capacity(& self) -> usize {
        self.capacity_
    }

    /// Add a property with default value. If a property with this name already exists, return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::property::PropertyContainer;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut pcontainer = PropertyContainer::<Vertex>::new();
    ///
    /// let prop = pcontainer.add::<u32>("my_prop",0);
    /// assert!(prop.is_some());
    ///
    /// let prop = pcontainer.add::<u32>("my_prop",0);
    /// assert!(prop.is_none());
    /// ```
    pub fn add<D : 'static + Clone>(&mut self, name : & 'static str, default_value : D) -> Option<Handle<Handle<T> > > {
        for &(n, _) in self.parrays_.iter() {
            if n == name {
                return None;
            }
        }
        let mut gv = PropertyVec::<Handle<T>,D>::new(default_value);
        gv.reserve(self.capacity_);
        for _ in 0..self.size_ {
            gv.push();
        }
        let p = Box::new(gv);
        self.parrays_.push((name,p));
        return Some(Handle::<Handle<T> >::new(self.parrays_.len()-1));
    }

    /// Get a property by its name. If it does not exist, return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::property::PropertyContainer;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut pcontainer = PropertyContainer::<Vertex>::new();
    ///
    /// let prop = pcontainer.get::<u32>("my_prop");
    /// assert!(prop.is_none());
    ///
    /// let prop = pcontainer.add::<u32>("my_prop",0);
    /// assert!(prop.is_some());
    ///
    /// let prop = pcontainer.get::<u32>("my_prop");
    /// assert!(prop.is_some());
    /// ```
    pub fn get<D : 'static + Clone>(&self, name : & 'static str) -> Option<Handle<Handle<T> > > {
        for (i, &(n, ref b)) in self.parrays_.iter().enumerate() {
            if n == name {
                if b.as_any().downcast_ref::<PropertyVec<Handle<T>,D>>().is_some() {
                    return Some(Handle::<Handle<T> >::new(i));
                }
            }
        }
        return None;
    }

    /// Adds a new element to all existing Property.
    ///
    /// # Examples
    ///
    /// ```
    /// ```
    /// use lwmesh::property::PropertyContainer;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut pcontainer = PropertyContainer::<Vertex>::new();
    ///
    /// let prop = pcontainer.add::<u32>("my_prop",17).unwrap();
    /// assert_eq!(prop.len(),0);
    /// let v = prop.push();
    /// assert_eq!(prop.len(),1);
    /// assert!(v.is_valid());
    /// ```
    pub fn push(&mut self){
        self.size_ += 1;
        for &mut(_, ref mut b) in self.parrays_.iter_mut() {
            b.push();
        }
    }

    /// Acces a property by its name. If it does not exist, return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::property::PropertyContainer;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut pcontainer = PropertyContainer::<Vertex>::new();
    ///
    /// let prop = pcontainer.add::<u32>("my_prop",17).unwrap();
    ///
    /// pcontainer.push();
    /// let v = Vertex::new(pcontainer.len()-1);
    ///
    /// assert_eq!(*pcontainer.access::<u32>(prop,v),17);
    ///
    /// ```
    pub fn access<D : 'static>(&self, prop : Handle<Handle<T> >, h : Handle<T>) -> &D {
        let (_,ref b) = self.parrays_[prop.idx()];
        let pa : &PropertyVec<Handle<T>,D> = b.as_any().downcast_ref::<PropertyVec<Handle<T>,D>>().unwrap();
        return &pa[h];
    }

    /// Acces a property by its name. If it does not exist, return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::property::PropertyContainer;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut pcontainer = PropertyContainer::<Vertex>::new();
    ///
    /// let prop = pcontainer.add::<u32>("my_prop",17).unwrap();
    ///
    /// pcontainer.push();
    /// let v = Vertex::new(pcontainer.len()-1);
    ///
    /// assert_eq!(*pcontainer.access::<u32>(prop,v),17u32);
    ///
    /// *pcontainer.access_mut::<u32>(prop,v) = 42;
    ///
    /// assert_eq!(*pcontainer.access::<u32>(prop,v),42u32);
    ///
    /// ```
    pub fn access_mut<D : 'static>(&mut self, prop : Handle<Handle<T> >, h : Handle<T>) -> &mut D {
        let (_,ref mut b) = self.parrays_[prop.idx()];
        let pa : &mut PropertyVec<Handle<T>,D> = b.as_any_mut().downcast_mut::<PropertyVec<Handle<T>,D>>().unwrap();
        return &mut pa[h];
    }
}

pub trait PropertyAccess<H> {
    fn access<D : 'static + Clone>(&self, prop : Handle<H>, h : H) -> &D;
    fn access_mut<D : 'static + Clone>(&mut self, prop : Handle<H>, h : H) -> &mut D;
}

#[cfg(test)]
mod tests {
    use super::*;
    use handle::*;

    #[test]
    fn reserve() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        pcontainer.reserve(16);
        assert!(16 <= pcontainer.capacity());
        pcontainer.reserve(33);
        assert!(33 <= pcontainer.capacity());
    }

    #[test]
    fn push() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        assert!(pcontainer.len() == 0);
        let size = 5;
        for _ in 0..size {
            pcontainer.push();
        }
        assert!(pcontainer.len() == size);
    }

    #[test]
    fn add() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        let prop = pcontainer.add::<u32>("v:my_prop",17);
        assert!(prop.is_some());
        let prop = pcontainer.add::<u32>("v:my_prop",17);
        assert!(prop.is_none());
    }

    #[test]
    fn get() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        let prop = pcontainer.get::<u32>("v:my_prop");
        assert!(prop.is_none());
        let prop = pcontainer.add::<u32>("v:my_prop",17);
        assert!(prop.is_some());
        let prop = pcontainer.get::<u32>("v:my_prop");
        assert!(prop.is_some());
    }

    #[test]
    fn reserve_and_add() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        pcontainer.reserve(17);
        assert!(17 <= pcontainer.capacity());
        let prop = pcontainer.add::<u32>("v:my_prop",0).unwrap();
        let (_,ref b) = pcontainer.parrays_[prop.idx()];
        let pa = b.as_any().downcast_ref::<PropertyVec<Vertex,u32>>().unwrap();
        assert_eq!(pa.capacity(),17);
    }

    #[test]
    fn push_and_add() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        pcontainer.push();
        let v0 = Vertex::new(pcontainer.len()-1);
        assert!(1 == pcontainer.len());
        let prop = pcontainer.add::<u32>("v:my_prop",17).unwrap();
        pcontainer.push();
        let v1 = Vertex::new(pcontainer.len()-1);
        assert!(2 == pcontainer.len());
        assert_eq!(*pcontainer.access::<u32>(prop,v0),17);
        assert_eq!(*pcontainer.access::<u32>(prop,v1),17);
    }

    #[test]
    fn access() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        let prop = pcontainer.add::<u32>("v:my_prop",17).unwrap();
        pcontainer.push();
        let v = Vertex::new(pcontainer.len()-1);
        assert_eq!(*pcontainer.access::<u32>(prop,v),17);
        *pcontainer.access_mut::<u32>(prop,v) = 42;
        assert_eq!(*pcontainer.access::<u32>(prop,v),42);
    }

    #[test]
    #[should_panic]
    fn access_out_of_bound() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        let v = Vertex::new(8);
        let prop = pcontainer.add::<u32>("v:my_prop",17).unwrap();
        pcontainer.access::<u32>(prop,v);
    }
}
