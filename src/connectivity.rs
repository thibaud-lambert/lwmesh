use handle::*;

/// Store the connectivity of a vertex in a halfedge mesh.
#[derive(Copy, Clone)]
pub struct VertexConnectivity {
    pub halfedge_ : Option<Halfedge>,
}

/// Store the connectivity of a halfedge in a halfedge mesh.
#[derive(Copy, Clone)]
pub struct HalfedgeConnectivity {
    pub face_ : Option<Face>,
    pub vertex_ : Vertex,
    pub next_halfedge_ : Halfedge,
    pub prev_halfedge_ : Halfedge,
}

/// Store the connectivity of a face in a halfedge mesh.
#[derive(Copy, Clone)]
pub struct FaceConnectivity {
    pub halfedge_ : Halfedge,
}

impl VertexConnectivity {
    /// Constructs an invalid `VertexConnectivity`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::connectivity::VertexConnectivity;
    ///
    /// let vc = VertexConnectivity::invalid();
    /// ```
    pub fn invalid() -> VertexConnectivity {
        VertexConnectivity {
            halfedge_ : None,
        }
    }

    /// Constructs a new `VertexConnectivity`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::connectivity::VertexConnectivity;
    /// use lwmesh::handle::Halfedge;
    ///
    /// let h = Halfedge::new(17);
    /// let vc = VertexConnectivity::new(h);
    /// ```
    pub fn new(h : Halfedge) -> VertexConnectivity {
        VertexConnectivity {
            halfedge_ : Some(h),
        }
    }
}

impl HalfedgeConnectivity {
    /// Constructs an invalid `HalfedgeConnectivity`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::connectivity::HalfedgeConnectivity;
    ///
    /// let hc = HalfedgeConnectivity::invalid();
    /// ```
    pub fn invalid() -> HalfedgeConnectivity {
        HalfedgeConnectivity {
            face_ : None,
            vertex_ : Vertex::new(0),
            next_halfedge_ : Halfedge::new(0),
            prev_halfedge_ : Halfedge::new(0),
        }
    }

    /// Constructs a new `HalfedgeConnectivity`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::connectivity::HalfedgeConnectivity;
    /// use lwmesh::handle::*;
    ///
    /// let f = Face::new(1);
    /// let v = Vertex::new(9);
    /// let nh = Halfedge::new(8);
    /// let ph = Halfedge::new(4);
    /// let hc = HalfedgeConnectivity::new(f,v,nh,ph);
    /// ```
    pub fn new(f : Face, v : Vertex, nh : Halfedge, ph : Halfedge) -> HalfedgeConnectivity {
        HalfedgeConnectivity {
            face_ : Some(f),
            vertex_ : v,
            next_halfedge_ : nh,
            prev_halfedge_ : ph,
        }
    }
}

impl FaceConnectivity {
    /// Constructs a new `FaceConnectivity`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::connectivity::FaceConnectivity;
    /// use lwmesh::handle::Halfedge;
    ///
    /// let h = Halfedge::new(5);
    /// let fc = FaceConnectivity::new(h);
    /// ```
    pub fn new(h : Halfedge) -> FaceConnectivity {
        FaceConnectivity {
            halfedge_ : h,
        }
    }
}
