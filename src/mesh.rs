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
