use property::*;
use handle::*;
use connectivity::*;

pub struct Topology {
    vconn_ : PropertyVec<Vertex,VertexConnectivity>,
    hconn_ : PropertyVec<Halfedge,HalfedgeConnectivity>,
    fconn_ : PropertyVec<Face,FaceConnectivity>,
}

impl Topology {
    pub fn new() -> Topology {
        Topology {
            vconn_ : PropertyVec::<Vertex,VertexConnectivity>::new(VertexConnectivity::invalid()),
            hconn_ : PropertyVec::<Halfedge,HalfedgeConnectivity>::new(HalfedgeConnectivity::invalid()),
            fconn_ : PropertyVec::<Face,FaceConnectivity>::new(FaceConnectivity::new(Halfedge::new(0))),
        }
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
    /// assert!(m.topology.n_vertices() == 2);
    /// ```
    pub fn n_vertices(& self) -> usize {
        self.vconn_.len()
    }

    /// Returns the number of faces in the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///    vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// assert!(m.topology.n_faces() == 1);
    /// ```
    pub fn n_faces(& self) -> usize {
        self.fconn_.len()
    }

    /// Returns the number of edges in the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///    vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// assert!(m.topology.n_edges() == 3);
    /// ```
    pub fn n_edges(& self) -> usize {
        self.hconn_.len()/2
    }

    /// Returns the number of halfedges in the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///    vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// assert!(m.topology.n_halfedges() == 6);
    /// ```
    pub fn n_halfedges(& self) -> usize {
        self.hconn_.len()
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
    /// assert!(m.topology.is_boundary_vertex(v));
    /// ```
    pub fn is_boundary_vertex(&self, v : Vertex) -> bool {
        let o = self.halfedge(v);
        match o {
            Some(h) => self.face(h).is_none(),
            None => true,
        }
    }

    /// Returns if the `Halfedge` is on a boundary
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// let h = m.topology.find_halfedge(vvec[0],vvec[1]).unwrap();
    /// assert!(!m.topology.is_boundary_halfedge(h));
    /// let oh = m.topology.opposite_halfedge(h);
    /// assert!(m.topology.is_boundary_halfedge(oh));
    /// ```
    pub fn is_boundary_halfedge(&self, h : Halfedge) -> bool {
        self.face(h).is_none()
    }

    /// Returns the `Face` incident to the `Halfedge`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec).unwrap();
    /// let h = m.topology.find_halfedge(vvec[0],vvec[1]).unwrap();
    /// assert!(m.topology.face(h).unwrap() == f);
    /// ```
    pub fn face(&self, h : Halfedge) -> Option<Face> {
        self.hconn_[h].face_
    }

    /// Returns an outgoing `Haldedge` of `Vertex` `v`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// let h = m.topology.halfedge(vvec[0]).unwrap();
    /// assert!(m.topology.from_vertex(h) == vvec[0]);
    /// ```
    pub fn halfedge(&self, v : Vertex) -> Option<Halfedge> {
        self.vconn_[v].halfedge_
    }

    /// Returns an outgoing `Haldedge` of `Edge` `e`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// let h = m.topology.halfedge(vvec[0]).unwrap();
    /// assert!(m.topology.from_vertex(h) == vvec[0]);
    /// ```
    pub fn edge_halfedge(&self, e : Edge, i : usize) -> Halfedge {
        Halfedge::new(e.idx()*2+i)
    }

    /// Returns an outgoing `Haldedge` of `Vertex` `v`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// let h = m.topology.halfedge(vvec[0]).unwrap();
    /// assert!(m.topology.from_vertex(h) == vvec[0]);
    /// ```
    pub fn face_halfedge(&self, f : Face) -> Halfedge {
        self.fconn_[f].halfedge_
    }

    /// Returns the `Edge`  that contains `Halfedge` h as one of its two halfedges.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// let h = m.topology.halfedge(vvec[0]).unwrap();
    /// let ho = m.topology.opposite_halfedge(h);
    /// assert!(m.topology.edge(h) == m.topology.edge(ho));
    /// ```
    pub fn edge(&self, h : Halfedge) -> Edge {
        Edge::new(h.idx()/2)
    }

    /// Returns the `Vertex` the `Halfedge` h points to.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// let h = m.topology.find_halfedge(vvec[0],vvec[1]).unwrap();
    /// assert!(m.topology.to_vertex(h) == vvec[1]);
    /// ```
    pub fn to_vertex(&self, h : Halfedge) -> Vertex {
        self.hconn_[h].vertex_
    }

    /// Returns the `Vertex` the `Halfedge` h emanates from.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// let h = m.topology.find_halfedge(vvec[0],vvec[1]).unwrap();
    /// assert!(m.topology.from_vertex(h) == vvec[0]);
    /// ```
    pub fn from_vertex(&self, h : Halfedge) -> Vertex {
        self.to_vertex(self.prev_halfedge(h))
    }

    /// Returns the next `Halfedge` within the incident face.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// let h0 = m.topology.find_halfedge(vvec[0],vvec[1]).unwrap();
    /// let h1 = m.topology.find_halfedge(vvec[1],vvec[2]).unwrap();
    /// let h2 = m.topology.find_halfedge(vvec[2],vvec[0]).unwrap();
    /// assert!(m.topology.next_halfedge(h0) == h1);
    /// assert!(m.topology.next_halfedge(h1) == h2);
    /// assert!(m.topology.next_halfedge(h2) == h0);
    /// ```
    pub fn next_halfedge(&self, h : Halfedge) -> Halfedge {
        self.hconn_[h].next_halfedge_
    }

    /// Returns the previous `Halfedge` within the incident face.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// let h0 = m.topology.find_halfedge(vvec[0],vvec[1]).unwrap();
    /// let h1 = m.topology.find_halfedge(vvec[1],vvec[2]).unwrap();
    /// let h2 = m.topology.find_halfedge(vvec[2],vvec[0]).unwrap();
    /// assert!(m.topology.prev_halfedge(h0) == h2);
    /// assert!(m.topology.prev_halfedge(h1) == h0);
    /// assert!(m.topology.prev_halfedge(h2) == h1);
    /// ```
    pub fn prev_halfedge(&self, h : Halfedge) -> Halfedge {
        self.hconn_[h].prev_halfedge_
    }

    /// Returns the opposite `Halfedge` of h.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// let h0 = m.topology.find_halfedge(vvec[0],vvec[1]).unwrap();
    /// let h1 = m.topology.find_halfedge(vvec[1],vvec[0]).unwrap();
    /// assert!(m.topology.opposite_halfedge(h0) == h1);
    /// ```
    pub fn opposite_halfedge(&self, h : Halfedge) -> Halfedge {
        let idx = h.idx();
        if (idx & 1) == 1 {
            Halfedge::new(idx-1)
        } else {
            Halfedge::new(idx+1)
        }
    }

    /// Retunrs the `Halfedge` that is rotated clockwise around the start `Vertex` of h.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// let h0 = m.topology.find_halfedge(vvec[0],vvec[1]).unwrap();
    /// let h1 = m.topology.find_halfedge(vvec[0],vvec[2]).unwrap();
    /// assert!(m.topology.cw_rotated_halfedge(h0) == h1);
    /// ```
    pub fn cw_rotated_halfedge(&self, h : Halfedge) -> Halfedge {
        self.next_halfedge(self.opposite_halfedge(h))
    }

    /// find the `Halfedge` from start to end.
    /// Retunrs the `Halfedge` that is rotated clockwise around the start `Vertex` of h.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// let h = m.topology.find_halfedge(vvec[0],vvec[1]);
    /// assert!(h.is_some());
    /// assert!(m.topology.from_vertex(h.unwrap()) == vvec[0]);
    /// assert!(m.topology.to_vertex(h.unwrap()) == vvec[1]);
    /// ```
    pub fn find_halfedge(&self, start : Vertex, end : Vertex) -> Option<Halfedge> {
        let mut h;
        match self.halfedge(start) {
            Some(x) => h=x,
            None => return None,
        };
        let h_end = h;
        loop {
            if self.to_vertex(h) == end {return Some(h);}
            h = self.cw_rotated_halfedge(h);
            if h == h_end {break;}
        }
        None
    }

    /// Sets the outgoing `Halfedge` of `Vertex` v to h.
    fn set_halfedge(&mut self, v : Vertex, h : Halfedge) {
        self.vconn_[v].halfedge_ = Some(h);
    }

    /// Sets the incident `Face` to `Halfedge` h to f.
    fn set_face(&mut self, h : Halfedge, f : Face) {
        self.hconn_[h].face_ = Some(f);
    }

    /// Sets the `Vertex` the `Halfedge` h points to to v.
    fn set_vertex(&mut self, h : Halfedge, v : Vertex) {
        self.hconn_[h].vertex_ = v;
    }

    /// Sets the next `Halfedge` of h within the face to nh
    fn set_next_halfedge(&mut self, h : Halfedge, nh : Halfedge) {
        self.hconn_[h].next_halfedge_ = nh;
        self.hconn_[nh].prev_halfedge_ = h;
    }

    /// Makes sure that the outgoing `Halfedge` of `Vertex` v is boundary halfedge if v is a boundary vertex.
    fn adjust_outgoing_halfedge(&mut self, v : Vertex) {
        let mut h;
        match self.halfedge(v) {
            Some(x) => h=x,
            None => return,
        }
        let hh = h;
        loop {
            if self.is_boundary_halfedge(h) {
                self.set_halfedge(v,h);
                return;
            }
            h = self.cw_rotated_halfedge(h);
            if h == hh {break;}
        }
    }
}

pub struct Properties {
    vprop_ : PropertyContainer<Vertex>,
    hprop_ : PropertyContainer<Halfedge>,
    eprop_ : PropertyContainer<Edge>,
    fprop_ : PropertyContainer<Face>,
}

impl Properties {
    pub fn new() -> Properties {
        Properties {
            vprop_ : PropertyContainer::new(),
            hprop_ : PropertyContainer::new(),
            eprop_ : PropertyContainer::new(),
            fprop_ : PropertyContainer::new(),
        }
    }

    /// Add a vertex property with default value. If a vertex property with this name already exists, return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// let pv = m.properties.add_vertex_property::<u32>("v:my_prop",17);
    /// assert!(pv.is_some());
    /// ```
    pub fn add_vertex_property<D : 'static + Clone>(&mut self, name : & 'static str, default_value : D) -> Option<PropertyVertex> {
        self.vprop_.add::<D>(name,default_value)
    }

    /// Add a face property with default value. If a face property with this name already exists, return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// let pf = m.properties.add_face_property::<u32>("f:my_prop",17);
    /// assert!(pf.is_some());
    /// ```
    pub fn add_face_property<D : 'static + Clone>(&mut self, name : & 'static str, default_value : D) -> Option<PropertyFace> {
        self.fprop_.add::<D>(name,default_value)
    }

    /// Add a edge property with default value. If a edge property with this name already exists, return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// let pe = m.properties.add_edge_property::<u32>("e:my_prop",17);
    /// assert!(pe.is_some());
    /// ```
    pub fn add_edge_property<D : 'static + Clone>(&mut self, name : & 'static str, default_value : D) -> Option<PropertyEdge> {
        self.eprop_.add::<D>(name,default_value)
    }

    /// Add a halfedge property with default value. If a halfedge property with this name already exists, return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// let ph = m.properties.add_halfedge_property::<u32>("h:my_prop",17);
    /// assert!(ph.is_some());
    /// ```
    pub fn add_halfedge_property<D : 'static + Clone>(&mut self, name : & 'static str, default_value : D) -> Option<PropertyHalfedge> {
        self.hprop_.add::<D>(name,default_value)
    }

    /// Get a vertex property by its name. If it does not exist, return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// m.properties.add_vertex_property::<u32>("v:my_prop",17);
    /// let pv = m.properties.get_vertex_property::<u32>("v:my_prop");
    /// assert!(pv.is_some());
    /// ```
    pub fn get_vertex_property<D : 'static + Clone>(&self, name : & 'static str) -> Option<PropertyVertex> {
        self.vprop_.get::<D>(name)
    }

    /// Get a face property by its name. If it does not exist, return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// m.properties.add_face_property::<u32>("f:my_prop",17);
    /// let pf = m.properties.get_face_property::<u32>("f:my_prop");
    /// assert!(pf.is_some());
    /// ```
    pub fn get_face_property<D : 'static + Clone>(&self, name : & 'static str) -> Option<PropertyFace> {
        self.fprop_.get::<D>(name)
    }

    /// Get a edge property by its name. If it does not exist, return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// m.properties.add_edge_property::<u32>("e:my_prop",17);
    /// let pe = m.properties.get_edge_property::<u32>("e:my_prop");
    /// assert!(pe.is_some());
    /// ```
    pub fn get_edge_property<D : 'static + Clone>(&self, name : & 'static str) -> Option<PropertyEdge> {
        self.eprop_.get::<D>(name)
    }

    /// Get a halfedge property by its name. If it does not exist, return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// m.properties.add_halfedge_property::<u32>("h:my_prop",17);
    /// let ph = m.properties.get_halfedge_property::<u32>("h:my_prop");
    /// assert!(ph.is_some());
    /// ```
    pub fn get_halfedge_property<D : 'static + Clone>(&self, name : & 'static str) -> Option<PropertyHalfedge> {
        self.hprop_.get::<D>(name)
    }


}

pub struct Mesh {
    pub topology : Topology,
    pub properties : Properties
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
            topology : Topology::new(),
            properties : Properties::new()
        }
    }

    /// Reserve the minimun capacity to store at least `size` vertex.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// m.vertex_reserve(15);
    /// assert_eq!(m.vertex_capacity(),15);
    /// ```
    pub fn vertex_reserve(&mut self, size : usize) {
        self.topology.vconn_.reserve(size);
        self.properties.vprop_.reserve(size);
    }

    /// Returns the number of vertex the given `Mesh` can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// m.vertex_reserve(15);
    /// assert_eq!(m.vertex_capacity(),15);
    /// ```
    pub fn vertex_capacity(&self) -> usize {
        self.topology.vconn_.capacity()
    }

    /// Reserve the minimun capacity to store at least `size` face.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// m.face_reserve(17);
    /// assert_eq!(m.face_capacity(),17);
    /// ```
    pub fn face_reserve(&mut self, size : usize) {
        self.topology.fconn_.reserve(size);
        self.properties.fprop_.reserve(size);
    }

    /// Returns the number of face the given `Mesh` can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// m.face_reserve(17);
    /// assert_eq!(m.face_capacity(),17);
    /// ```
    pub fn face_capacity(&self) -> usize {
        self.topology.fconn_.capacity()
    }

    /// Reserve the minimun capacity to store at least `size` edge.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// m.edge_reserve(17);
    /// assert_eq!(m.edge_capacity(),17);
    /// ```
    pub fn edge_reserve(&mut self, size : usize) {
        self.topology.hconn_.reserve(size*2);
        self.properties.eprop_.reserve(size);
        self.properties.hprop_.reserve(size*2);
    }

    /// Returns the number of edge the given `Mesh` can hold without reallocating.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// m.edge_reserve(17);
    /// assert_eq!(m.edge_capacity(),17);
    /// ```
    pub fn edge_capacity(&self) -> usize {
        self.topology.hconn_.capacity()/2
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
        self.properties.vprop_.push();
        self.topology.vconn_.push();
        Vertex::new(self.topology.vconn_.len()-1)
    }

    /// Adds new vertices to the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// let vec = m.add_vertices(17);
    /// assert_eq!(m.topology.n_vertices(),17);
    /// assert_eq!(vec.len(),17);
    /// ```
    pub fn add_vertices(&mut self, nb : usize) -> Vec<Vertex> {
        let mut vec : Vec<Vertex> = Vec::new();
        if self.vertex_capacity() < self.topology.n_vertices()+nb {
            let new_cap = self.topology.n_vertices()+nb;
            self.vertex_reserve(new_cap);
        }
        for _ in 0..nb {
            self.properties.vprop_.push();
            self.topology.vconn_.push();
            vec.push(Vertex::new(self.topology.vconn_.len()-1));
        }
        return vec;
    }

    /// Adds a new `Face` to the `Mesh`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// assert!(f.is_some());
    /// ```
    pub fn add_face(&mut self, vertices : & Vec<Vertex>) -> Option<Face> {
        let n = vertices.len();
        let mut hvec = Vec::<Option<Halfedge>>::new();
        let mut new_hvec = Vec::<bool>::new();
        hvec.reserve(n);
        new_hvec.reserve(n);

        // Does the face to add is valid
        for i in 0..n {
            if !self.topology.is_boundary_vertex(vertices[i]) {
                return None;
            }
            hvec.push(self.topology.find_halfedge(vertices[i],vertices[(i+1)%n]));
            new_hvec.push(hvec[i].is_none());
            if !new_hvec[i] && !self.topology.is_boundary_halfedge(hvec[i].unwrap()) {
                return None;
            }
        }

        // Create missing edges
        for i in 0..n {
            if new_hvec[i] {
                let ii = (i+1)%n;
                hvec[i] = Some(self.new_edge(vertices[i], vertices[ii]));
            }
        }

        // Creates the new face
        self.properties.fprop_.push();
        self.topology.fconn_.push();
        let f = Face::new(self.topology.fconn_.len()-1);
        self.topology.fconn_[f] = FaceConnectivity::new(hvec[n-1].unwrap());

        // Setup halfedges
        let mut next_cache : Vec<(Halfedge,Halfedge)> = Vec::new();
        let mut needs_adjust : Vec<bool> = Vec::new();
        needs_adjust.resize(n,false);
        for i in 0..n {
            let ii = (i+1)%n;
            let v = vertices[ii];
            let inner_prev = hvec[i].unwrap();
            let inner_next = hvec[ii].unwrap();

            if new_hvec[i] || new_hvec[ii] {
                let outer_prev = self.topology.opposite_halfedge(inner_next);
                let outer_next = self.topology.opposite_halfedge(inner_prev);

                if !new_hvec[ii] { // prev is new, next is not
                    let boundary_prev = self.topology.prev_halfedge(inner_next);
                    next_cache.push((boundary_prev,outer_next));
                    self.topology.set_halfedge(v,outer_next);
                } else if !new_hvec[i] { // next is new, prev is not
                    let boundary_next = self.topology.next_halfedge(inner_prev);
                    next_cache.push((outer_prev,boundary_next));
                    self.topology.set_halfedge(v,boundary_next);
                } else { // both are new
                    match self.topology.halfedge(v) {
                        None => {
                            self.topology.set_halfedge(v,outer_next);
                            next_cache.push((outer_prev,outer_next));
                        }
                        Some(h) => {
                            let boundary_prev = self.topology.prev_halfedge(h);
                            next_cache.push((boundary_prev,outer_next));
                            next_cache.push((outer_prev,h));
                        }
                    }
                }
                next_cache.push((inner_prev,inner_next));
            } else {
                needs_adjust[ii] = self.topology.halfedge(v).unwrap() == inner_next;
            }
            self.topology.set_face(inner_prev,f);
        }

        // process cache
        for (first,second) in next_cache {
            self.topology.set_next_halfedge(first,second);
        }

        // adjust vertices halfedge handle
        for i in 0..n {
            if needs_adjust[i] {
                self.topology.adjust_outgoing_halfedge(vertices[i]);
            }
        }

        return Some(f);
    }

    /// allocate a new edge and returns the `Halfedge` from start to end
    fn new_edge(&mut self, start : Vertex, end : Vertex) -> Halfedge {
        assert!(start != end);

        self.properties.eprop_.push();
        self.properties.hprop_.push();
        self.properties.hprop_.push();
        self.topology.hconn_.push();
        let h0 = Halfedge::new(self.topology.hconn_.len()-1);
        self.topology.hconn_.push();
        let h1 = Halfedge::new(self.topology.hconn_.len()-1);

        self.topology.set_vertex(h0, end);
        self.topology.set_vertex(h1, start);

        return h0;
    }
}

impl PropertyAccess<Vertex> for Properties {
    /// Access the element of the vertex 'Property' prop indexing by 'Vertex' v.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::property::PropertyAccess;
    ///
    /// let mut m = Mesh::new();
    /// let prop = m.properties.add_vertex_property::<u32>("v:my_prop",17).unwrap();
    /// let v0 = m.add_vertex();
    /// assert_eq!(*m.properties.access::<u32>(prop,v0),17);
    /// ```
    fn access<D : 'static + Clone>(&self, prop : PropertyVertex, v : Vertex) -> &D{
        self.vprop_.access::<D>(prop,v)
    }

    /// Mutable access to the element of the vertex 'Property' prop indexing by 'Vertex' v.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::property::PropertyAccess;
    ///
    /// let mut m = Mesh::new();
    /// let prop = m.properties.add_vertex_property::<u32>("v:my_prop",17).unwrap();
    /// let v0 = m.add_vertex();
    /// assert_eq!(*m.properties.access::<u32>(prop,v0),17);
    /// *m.properties.access_mut::<u32>(prop,v0) = 42;
    /// assert_eq!(*m.properties.access::<u32>(prop,v0),42);
    /// ```
    fn access_mut<D : 'static + Clone>(&mut self, prop : PropertyVertex, v : Vertex) -> &mut D{
        self.vprop_.access_mut::<D>(prop,v)
    }
}

impl PropertyAccess<Face> for Properties {
    /// Access the element of the face 'Property' prop indexing by 'Face' f.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::property::PropertyAccess;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let prop = m.properties.add_face_property::<u32>("f:my_prop",17).unwrap();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec).unwrap();
    /// assert_eq!(*m.properties.access::<u32>(prop,f),17);
    /// ```
    fn access<D : 'static + Clone>(&self, prop : PropertyFace, f : Face) -> &D{
        self.fprop_.access::<D>(prop,f)
    }

    /// Mutable access to the element of the face 'Property' prop indexing by 'Face' f.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::property::PropertyAccess;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let prop = m.properties.add_face_property::<u32>("f:my_prop",17).unwrap();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec).unwrap();
    /// assert_eq!(*m.properties.access::<u32>(prop,f),17);
    /// *m.properties.access_mut::<u32>(prop,f) = 42;
    /// assert_eq!(*m.properties.access::<u32>(prop,f),42);
    /// ```
    fn access_mut<D : 'static + Clone>(&mut self, prop : PropertyFace, f : Face) -> &mut D{
        self.fprop_.access_mut::<D>(prop,f)
    }
}

impl PropertyAccess<Edge> for Properties {
    /// Access the element of the edge 'Property' prop indexing by 'Edge' e.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::property::PropertyAccess;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let prop = m.properties.add_edge_property::<u32>("e:my_prop",17).unwrap();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    /// let e = m.topology.edge(m.topology.find_halfedge(vvec[0],vvec[1]).unwrap());
    /// assert_eq!(*m.properties.access::<u32>(prop,e),17);
    /// ```
    fn access<D : 'static + Clone>(&self, prop : PropertyEdge, e : Edge) -> &D{
        self.eprop_.access::<D>(prop,e)
    }

    /// Mutable access to the element of the edge 'Property' prop indexing by 'Edge' e.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::property::PropertyAccess;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let prop = m.properties.add_edge_property::<u32>("e:my_prop",17).unwrap();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    /// let e = m.topology.edge(m.topology.find_halfedge(vvec[0],vvec[1]).unwrap());
    /// assert_eq!(*m.properties.access::<u32>(prop,e),17);
    /// *m.properties.access_mut::<u32>(prop,e) = 42;
    /// assert_eq!(*m.properties.access::<u32>(prop,e),42);
    /// ```
    fn access_mut<D : 'static + Clone>(&mut self, prop : PropertyEdge, e : Edge) -> &mut D{
        self.eprop_.access_mut::<D>(prop,e)
    }
}

impl PropertyAccess<Halfedge> for Properties {
    /// Access the element of the halfedge 'Property' prop indexing by 'Halfedge' h.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::property::PropertyAccess;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let prop = m.properties.add_halfedge_property::<u32>("h:my_prop",17).unwrap();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    /// let h = m.topology.find_halfedge(vvec[0],vvec[1]).unwrap();
    /// assert_eq!(*m.properties.access::<u32>(prop,h),17);
    /// *m.properties.access_mut::<u32>(prop,h) = 42;
    /// assert_eq!(*m.properties.access::<u32>(prop,h),42);
    /// ```
    fn access<D : 'static + Clone>(&self, prop : PropertyHalfedge, h : Halfedge) -> &D{
        self.hprop_.access::<D>(prop,h)
    }

    /// Mutable access to the element of the halfedge 'Property' prop indexing by 'Halfedge' h.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::property::PropertyAccess;
    /// use lwmesh::handle::Vertex;
    ///
    /// let mut m = Mesh::new();
    /// let prop = m.properties.add_halfedge_property::<u32>("h:my_prop",17).unwrap();
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    /// let h = m.topology.find_halfedge(vvec[0],vvec[1]).unwrap();
    /// assert_eq!(*m.properties.access::<u32>(prop,h),17);
    /// *m.properties.access_mut::<u32>(prop,h) = 42;
    /// assert_eq!(*m.properties.access::<u32>(prop,h),42);
    /// ```
    fn access_mut<D : 'static + Clone>(&mut self, prop : PropertyHalfedge, h : Halfedge) -> &mut D{
        self.hprop_.access_mut::<D>(prop,h)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::handle::Vertex;
    use property::PropertyAccess;

    #[test]
    fn add_vertex() {
        let mut m = Mesh::new();
        assert!(m.properties.vprop_.len() == 0);

        let v0 = m.add_vertex();
        assert!(m.properties.vprop_.len() == 1);
        assert!(v0.idx() == 0);

        m.add_vertex();
        let v2 = m.add_vertex();
        assert!(m.properties.vprop_.len() == 3);
        assert!(v2.idx() == 2);
    }

    #[test]
    fn n_vertices() {
        let mut m = Mesh::new();
        assert!(m.topology.n_vertices() == 0);

        m.add_vertex();
        assert!(m.topology.n_vertices() == 1);

        m.add_vertex();
        m.add_vertex();
        assert!(m.topology.n_vertices() == 3);
    }

    #[test]
    fn add_face() {
        let mut m = Mesh::new();
        let mut vvec = Vec::<Vertex>::new();
        let v0 = m.add_vertex();
        let v1 = m.add_vertex();
        let v2 = m.add_vertex();
        let v3 = m.add_vertex();
        vvec.push(v0);
        vvec.push(v1);
        vvec.push(v2);
        let f = m.add_face(&vvec);
        assert!(f.is_some());
        assert!(m.topology.n_faces() == 1);

        vvec.clear();
        vvec.push(v2);
        vvec.push(v1);
        vvec.push(v3);
        let f = m.add_face(&vvec);
        assert!(f.is_some());
        assert!(m.topology.n_faces() == 2);

        let f = m.add_face(&vvec);
        assert!(f.is_none());
        assert!(m.topology.n_faces() == 2);

        let v4 = m.add_vertex();
        vvec.clear();
        vvec.push(v2);
        vvec.push(v1);
        vvec.push(v4);
        let f = m.add_face(&vvec);
        assert!(f.is_none());
        assert!(m.topology.n_faces() == 2);
    }

    #[test]
    fn property() {
        let mut m = Mesh::new();

        // Vertex
        let prop = m.properties.add_vertex_property::<u32>("v:my_prop",17).unwrap();
        let v0 = m.add_vertex();
        assert_eq!(*m.properties.access::<u32>(prop,v0),17);
        *m.properties.access_mut::<u32>(prop,v0) = 42;
        assert_eq!(*m.properties.access::<u32>(prop,v0),42);

        // Face
        let prop = m.properties.add_face_property::<u32>("f:my_prop",17).unwrap();
        let mut vvec = Vec::<Vertex>::new();
        let v1 = m.add_vertex();
        let v2 = m.add_vertex();
        vvec.push(v0);
        vvec.push(v1);
        vvec.push(v2);
        let f = m.add_face(&vvec).unwrap();
        assert_eq!(*m.properties.access::<u32>(prop,f),17);
        *m.properties.access_mut::<u32>(prop,f) = 42;
        assert_eq!(*m.properties.access::<u32>(prop,f),42);

        // Edge
        let prop = m.properties.add_edge_property::<u32>("v:my_prop",17).unwrap();
        let e = m.topology.edge(m.topology.find_halfedge(v0,v1).unwrap());
        assert_eq!(*m.properties.access::<u32>(prop,e),17);
        *m.properties.access_mut::<u32>(prop,e) = 42;
        assert_eq!(*m.properties.access::<u32>(prop,e),42);

        // Halfedge
        let prop = m.properties.add_halfedge_property::<u32>("v:my_prop",17).unwrap();
        let h = m.topology.find_halfedge(v2,v0).unwrap();
        assert_eq!(*m.properties.access::<u32>(prop,h),17);
        *m.properties.access_mut::<u32>(prop,h) = 42;
        assert_eq!(*m.properties.access::<u32>(prop,h),42);
    }

    #[test]
    #[should_panic]
    fn invalid_property() {
        let mut m = Mesh::new();
        let prop = m.properties.get_vertex_property::<u32>("v:my_prop").unwrap();
        let v0 = m.add_vertex();
        m.properties.access::<u32>(prop,v0);
    }
}
