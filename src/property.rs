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
    pub parrays_ : Vec<(& 'static str,Box<ResizableVec>)>,
    size_ : usize,
    capacity_ : usize,
}

impl<T : 'static> PropertyContainer<Handle<T>> {
    /// Constructs a new `PropertyContainer`.
    pub fn new() -> PropertyContainer<Handle<T>>{
        PropertyContainer {
            handle_ : PhantomData,
            parrays_ : Vec::new(),
            size_ : 0,
            capacity_ : 0
        }
    }

    /// Reserve the minimun capacity to store at least `size` elements in the given `PropertyContainer`.
    pub fn reserve(&mut self, size : usize) {
        self.capacity_ = size;
        for &mut(_, ref mut b) in self.parrays_.iter_mut() {
            b.reserve(size);
        }
    }

    /// Add a property with default value. If a property with this name already exists, return `None`.
    pub fn add<D : 'static + Clone>(&mut self, name : & 'static str, default_value : D) -> Option<Handle<(T,D)> > {
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
        return Some(Handle::<(T,D)>::new(self.parrays_.len()-1));
    }

    /// Get a property by its name. If it does not exist, return `None`.
    pub fn get<D : 'static + Clone>(&self, name : & 'static str) -> Option<Handle<(T,D)> > {
        for (i, &(n, ref b)) in self.parrays_.iter().enumerate() {
            if n == name && b.as_any().downcast_ref::<PropertyVec<Handle<T>,D>>().is_some() {
                return Some(Handle::<(T,D)>::new(i));
            }
        }
        return None;
    }

    /// Adds a new element to all existing Property.
    pub fn push(&mut self){
        self.size_ += 1;
        for &mut(_, ref mut b) in self.parrays_.iter_mut() {
            b.push();
        }
    }
}

impl<T : 'static, D : 'static> Index<(Handle<(T,D)>,Handle<T>)> for PropertyContainer<Handle<T>> {
    type Output = D;

    /// Access the element of the vertex 'Property' prop indexing by 'Vertex' v.
    fn index(&self, (p,h): (Handle<(T,D)>,Handle<T>)) -> &D {
        let (_,ref b) = self.parrays_[p.idx()];
        let pa : &PropertyVec<Handle<T>,D> = b.as_any().downcast_ref::<PropertyVec<Handle<T>,D>>().unwrap();
        return &pa[h];
    }
}

impl<T : 'static, D : 'static> IndexMut<(Handle<(T,D)>,Handle<T>)> for PropertyContainer<Handle<T>> {

    /// Mutable access to the element of the vertex 'Property' prop indexing by 'Vertex' v.
    fn index_mut(&mut self, (p,h): (Handle<(T,D)>,Handle<T>)) -> &mut D {
        let (_,ref mut b) = self.parrays_[p.idx()];
        let pa : &mut PropertyVec<Handle<T>,D> = b.as_any_mut().downcast_mut::<PropertyVec<Handle<T>,D>>().unwrap();
        return &mut pa[h];
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use handle::Vertex;

    #[test]
    fn len() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        assert!(pcontainer.size_ == 0);
        pcontainer.push();
        assert!(pcontainer.size_ == 1);
        pcontainer.push();
        pcontainer.push();
        assert!(pcontainer.size_ == 3);
    }

    #[test]
    fn reserve_and_capacity() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        pcontainer.reserve(16);
        assert!(16 <= pcontainer.capacity_);
        pcontainer.reserve(33);
        assert!(33 <= pcontainer.capacity_);
    }

    #[test]
    fn push() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        assert!(pcontainer.size_ == 0);
        let size = 5;
        for _ in 0..size {
            pcontainer.push();
        }
        assert!(pcontainer.size_ == size);
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
        assert!(17 <= pcontainer.capacity_);
        let prop = pcontainer.add::<u32>("v:my_prop",0).unwrap();
        let (_,ref b) = pcontainer.parrays_[prop.idx()];
        let pa = b.as_any().downcast_ref::<PropertyVec<Vertex,u32>>().unwrap();
        assert_eq!(pa.capacity(),17);
    }

    #[test]
    fn push_and_add() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        pcontainer.push();
        let v0 = Vertex::new(pcontainer.size_-1);
        assert!(1 == pcontainer.size_);
        let prop = pcontainer.add::<u32>("v:my_prop",17).unwrap();
        pcontainer.push();
        let v1 = Vertex::new(pcontainer.size_-1);
        assert!(2 == pcontainer.size_);
        assert_eq!(pcontainer[(prop,v0)],17);
        assert_eq!(pcontainer[(prop,v1)],17);
    }

    #[test]
    fn access() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        let prop = pcontainer.add::<u32>("v:my_prop",17).unwrap();
        pcontainer.push();
        let v = Vertex::new(pcontainer.size_-1);
        assert_eq!(pcontainer[(prop,v)],17);
        pcontainer[(prop,v)] = 42;
        assert_eq!(pcontainer[(prop,v)],42);
    }

    #[test]
    #[should_panic]
    fn access_out_of_bound() {
        let mut pcontainer = PropertyContainer::<Vertex>::new();
        let v = Vertex::new(8);
        let prop = pcontainer.add::<u32>("v:my_prop",17).unwrap();
        pcontainer[(prop,v)];
    }
}
