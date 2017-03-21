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
    pub fn invalid() -> VertexConnectivity {
        VertexConnectivity {
            halfedge_ : None,
        }
    }
}

impl HalfedgeConnectivity {
    /// Constructs an invalid `HalfedgeConnectivity`.
    pub fn invalid() -> HalfedgeConnectivity {
        HalfedgeConnectivity {
            face_ : None,
            vertex_ : Vertex::new(0),
            next_halfedge_ : Halfedge::new(0),
            prev_halfedge_ : Halfedge::new(0),
        }
    }
}

impl FaceConnectivity {
    /// Constructs an invalid `FaceConnectivity`.
    pub fn invalid() -> FaceConnectivity {
        FaceConnectivity {
            halfedge_ : Halfedge::new(0),
        }
    }
}
