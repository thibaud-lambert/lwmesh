use property::Property;
use handle::*;
use connectivity::*;

pub struct Mesh {
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
        self.vconn_.push(vc)
    }

}
