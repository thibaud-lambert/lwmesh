use property::Property;
use handle::*;
use connectivity::*;

pub struct Mesh {
    n_v_ : usize,

    vconn_ : Property<Vertex,VertexConnectivity>,
    hconn_ : Property<Halfedge,HalfedgeConnectivity>,
    fconn_ : Property<Face,FaceConnectivity>,
}

impl Mesh {
    /// Constructs an empty `Mesh`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let m = Mesh::new();
    /// ```
    pub fn new() -> Mesh {
        Mesh {
            n_v_ : 0,
            vconn_ : Property::new(),
            hconn_ : Property::new(),
            fconn_ : Property::new(),
        }
    }

    /// Adds a new vertex to the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// let v = m.add_vertex();
    /// ```
    pub fn add_vertex(&mut self) -> Vertex {
        let vc = VertexConnectivity {
                halfedge_ : Halfedge::invalid()
        };
        self.n_v_+=1;
        self.vconn_.push(vc)
    }

    /// Returns the number of vertices in the `Mesh`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// let v1 = m.add_vertex();
    /// let v2 = m.add_vertex();
    /// assert!(m.n_vertices() == 2);
    /// ```
    pub fn n_vertices(& self) -> usize {
        self.n_v_
    }


    /// Returns if the `Vertex` is on a boundary
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// let v = m.add_vertex();
    /// assert!(m.is_boundary_vertex(v));
    /// ```
    pub fn is_boundary_vertex(&self, v : Vertex) -> bool {
        let h = self.halfedge(v);
        !(h.is_valid() && self.face(h).is_valid())
    }

    /// Returns if the `Halfedge` is on a boundary
    pub fn is_boundary_halfedge(&self, h : Halfedge) -> bool {
        !(h.is_valid() && self.face(h).is_valid())
    }

    /// Returns the `Face` incident to the `Halfedge`.
    pub fn face(&self, h : Halfedge) -> Face {
        self.hconn_[h].face_
    }

    /// Returns an outgoing `Haldedge` of `Vertex` `v`.
    pub fn halfedge(&self, v : Vertex) -> Halfedge {
        self.vconn_[v].halfedge_
    }

    /// Returns the `Vertex` the `Halfedge` h points to.
    pub fn to_vertex(&self, h : Halfedge) -> Vertex {
        self.hconn_[h].vertex_
    }

    /// Returns the `Vertex` the `Halfedge` h emanates from.
    pub fn from_vertex(&self, h : Halfedge) -> Vertex {
        self.hconn_[self.hconn_[h].prev_halfedge_].vertex_
    }

    /// Returns the next `Halfedge` within the incident face.
    pub fn next_halfedge(&self, h : Halfedge) -> Halfedge {
        self.hconn_[h].next_halfedge_
    }

    /// Returns the previous `Halfedge` within the incident face.
    pub fn prev_halfedge(&self, h : Halfedge) -> Halfedge {
        self.hconn_[h].prev_halfedge_
    }

    /// Returns the opposite `Halfedge` of h.
    pub fn opposite_halfedge(&self, h : Halfedge) -> Halfedge {
        let idx = h.idx().unwrap();
        if (idx & 1) == 1 {
            Halfedge::new(idx-1)
        } else {
            Halfedge::new(idx+1)
        }
    }

    /// Retunrs the `Halfedge` that is rotated clockwise around the start `Vertex` of h.
    pub fn cw_rotated_halfedge(&self, h : Halfedge) -> Halfedge {
        self.next_halfedge(self.opposite_halfedge(h))
    }

    /// find the `Halfedge` from start to end.
    pub fn find_halfedge(&self, start : Vertex, end : Vertex) -> Halfedge {
        if start.is_valid() && end.is_valid() {
            let mut h = self.halfedge(start);
            let h_end = h;
            if h.is_valid() {
                loop {
                    if self.to_vertex(h) == end {return h;}
                    h = self.cw_rotated_halfedge(h);
                    if h == h_end {break;}
                }
            }
        }
        Halfedge::invalid()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vertex() {
        let mut m = Mesh::new();
        assert!(m.n_v_ == 0);

        let v0 = m.add_vertex();
        assert!(m.n_v_ == 1);
        assert!(v0.idx().unwrap() == 0);

        m.add_vertex();
        let v2 = m.add_vertex();
        assert!(m.n_v_ == 3);
        assert!(v2.idx().unwrap() == 2);
    }

    #[test]
    fn n_vertices() {
        let mut m = Mesh::new();
        assert!(m.n_vertices() == 0);

        m.add_vertex();
        assert!(m.n_vertices() == 1);

        m.add_vertex();
        m.add_vertex();
        assert!(m.n_vertices() == 3);
    }
}
