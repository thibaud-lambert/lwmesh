use std::marker::PhantomData;
use std::cmp::Ordering;

#[derive(Copy, Clone)]
pub struct PhantomVertex;
#[derive(Copy, Clone)]
pub struct PhantomFace;
#[derive(Copy, Clone)]
pub struct PhantomEdge;
#[derive(Copy, Clone)]
pub struct PhantomHalfedge;
#[derive(Copy, Clone)]
pub struct PhantomProperty;

pub type Vertex = Handle<PhantomVertex>;
pub type Face = Handle<PhantomFace>;
pub type Edge = Handle<PhantomEdge>;
pub type Halfedge = Handle<PhantomHalfedge>;
pub type PropertyVertex = Handle<Vertex>;
pub type PropertyFace = Handle<Face>;
pub type PropertyEdge = Handle<Edge>;
pub type PropertyHalfedge = Handle<Halfedge>;

/// A basic handle
///
/// `Handle<A>` is a nice encapsulation for an `usize`. It's an elegant way to manipulate
/// elements (`Vertex`, `Face`, `Edge`, `Halfedge`).
#[derive(Copy, Clone)]
pub struct Handle<A> {
    type_ : PhantomData<A>,
    index_ : usize,
}

impl<A> Handle<A> {
    /// Constructs a new `Handle<A>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::handle::Vertex;
    ///
    /// let v = Vertex::new(3);
    /// ```
    pub fn new(idx : usize) -> Handle<A> {
        Handle {
            type_ : PhantomData,
            index_ : idx,
        }
    }

    /// Returns the index of the given `Handle<A>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::handle::Vertex;
    ///
    /// let v = Vertex::new(42);
    /// assert!(v.idx()==42);
    /// ```
    pub fn idx(&self) -> usize {
        self.index_
    }
}

impl<A> Ord for Handle<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index_.cmp(&other.index_)
    }
}

impl<A> PartialOrd for Handle<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.index_.cmp(&other.index_))
    }
}

impl<A> PartialEq for Handle<A> {
    fn eq(&self, other: &Self) -> bool {
        self.index_ == other.index_
    }
}
impl<A> Eq for Handle<A> { }

#[cfg(test)]
mod tests {
    use super::*;
    use std::cmp::Ordering;

    #[test]
    fn idx() {
        let idx = 42;
        let handle = Vertex::new(idx);
        assert!(handle.idx() == idx);
    }

    #[test]
    fn cmp() {
        let idx1 = 42;
        let idx2 = 13;
        let h1 = Vertex::new(idx1);
        let h2 = Vertex::new(idx2);
        let h3 = Vertex::new(idx1);

        assert!(h1!=h2);
        assert!(h1>h2);
        assert!(h2<h1);
        assert!(h1==h3);
        assert!(h1<=h3);
        assert_eq!(h2.cmp(&h1), Ordering::Less);
        assert_eq!(h1.cmp(&h2), Ordering::Greater);
        assert_eq!(h1.cmp(&h3), Ordering::Equal);
    }
}
