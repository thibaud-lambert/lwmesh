use handle::*;

pub struct VertexConnectivity {
    pub halfedge_ : Halfedge,
}

pub struct HalfedgeConnectivity {
    pub face_ : Face,
    pub vertex_ : Vertex,
    pub next_halfedge_ : Halfedge,
    pub prev_halfedge_ : Halfedge,
}

pub struct FaceConnectivity {
    pub halfedge_ : Halfedge,
}
