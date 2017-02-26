use property::PropertyContainer;
use property::PropertyAccess;
use handle::*;
use connectivity::*;

pub struct Mesh {
    vprop_ : PropertyContainer<Vertex>,
    hprop_ : PropertyContainer<Halfedge>,
    eprop_ : PropertyContainer<Edge>,
    fprop_ : PropertyContainer<Face>,
    vconn_ : Property,
    hconn_ : Property,
    fconn_ : Property,
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
        let mut m = Mesh {
            vprop_ : PropertyContainer::new(),
            hprop_ : PropertyContainer::new(),
            eprop_ : PropertyContainer::new(),
            fprop_ : PropertyContainer::new(),
            vconn_ : Property::invalid(),
            hconn_ : Property::invalid(),
            fconn_ : Property::invalid(),
        };
        m.vconn_ = m.vprop_.add::<VertexConnectivity>("v:connectivity",VertexConnectivity::invalid());
        m.hconn_ = m.hprop_.add::<HalfedgeConnectivity>("h:connectivity",HalfedgeConnectivity::invalid());
        m.fconn_ = m.fprop_.add::<FaceConnectivity>("f:connectivity",FaceConnectivity::invalid());
        return m;
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
        self.vprop_.reserve(size);
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
    pub fn vertex_capacity(&mut self) -> usize {
        self.vprop_.capacity()
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
        self.fprop_.reserve(size);
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
    pub fn face_capacity(&mut self) -> usize {
        self.fprop_.capacity()
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
        self.eprop_.reserve(size);
        self.hprop_.reserve(size*2);
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
    pub fn edge_capacity(&mut self) -> usize {
        self.eprop_.capacity()
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
    /// assert!(v.is_valid());
    /// ```
    pub fn add_vertex(&mut self) -> Vertex {
        self.vprop_.push()
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
        self.vprop_.len()
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
    /// assert!(m.n_faces() == 1);
    /// ```
    pub fn n_faces(& self) -> usize {
        self.fprop_.len()
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
    /// assert!(m.n_edges() == 3);
    /// ```
    pub fn n_edges(& self) -> usize {
        self.eprop_.len()
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
    /// let h = m.find_halfedge(vvec[0],vvec[1]);
    /// assert!(!m.is_boundary_halfedge(h));
    /// let oh = m.opposite_halfedge(h);
    /// assert!(m.is_boundary_halfedge(oh));
    /// ```
    pub fn is_boundary_halfedge(&self, h : Halfedge) -> bool {
        !(h.is_valid() && self.face(h).is_valid())
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
    /// let f = m.add_face(&vvec);
    /// let h = m.find_halfedge(vvec[0],vvec[1]);
    /// assert!(m.face(h) == f);
    /// ```
    pub fn face(&self, h : Halfedge) -> Face {
        self.hprop_.access::<HalfedgeConnectivity>(self.hconn_,h).face_
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
    /// let h = m.halfedge(vvec[0]);
    /// assert!(m.from_vertex(h) == vvec[0]);
    /// ```
    pub fn halfedge(&self, v : Vertex) -> Halfedge {
        self.vprop_.access::<VertexConnectivity>(self.vconn_,v).halfedge_
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
    /// let h = m.halfedge(vvec[0]);
    /// let ho = m.opposite_halfedge(h);
    /// assert!(m.edge(h) == m.edge(ho));
    /// ```
    pub fn edge(&self, h : Halfedge) -> Edge {
        Edge::new(h.idx().unwrap()/2)
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
    /// let h = m.find_halfedge(vvec[0],vvec[1]);
    /// assert!(m.to_vertex(h) == vvec[1]);
    /// ```
    pub fn to_vertex(&self, h : Halfedge) -> Vertex {
        self.hprop_.access::<HalfedgeConnectivity>(self.hconn_,h).vertex_
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
    /// let h = m.find_halfedge(vvec[0],vvec[1]);
    /// assert!(m.from_vertex(h) == vvec[0]);
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
    /// let h0 = m.find_halfedge(vvec[0],vvec[1]);
    /// let h1 = m.find_halfedge(vvec[1],vvec[2]);
    /// let h2 = m.find_halfedge(vvec[2],vvec[0]);
    /// assert!(m.next_halfedge(h0) == h1);
    /// assert!(m.next_halfedge(h1) == h2);
    /// assert!(m.next_halfedge(h2) == h0);
    /// ```
    pub fn next_halfedge(&self, h : Halfedge) -> Halfedge {
        self.hprop_.access::<HalfedgeConnectivity>(self.hconn_,h).next_halfedge_
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
    /// let h0 = m.find_halfedge(vvec[0],vvec[1]);
    /// let h1 = m.find_halfedge(vvec[1],vvec[2]);
    /// let h2 = m.find_halfedge(vvec[2],vvec[0]);
    /// assert!(m.prev_halfedge(h0) == h2);
    /// assert!(m.prev_halfedge(h1) == h0);
    /// assert!(m.prev_halfedge(h2) == h1);
    /// ```
    pub fn prev_halfedge(&self, h : Halfedge) -> Halfedge {
        self.hprop_.access::<HalfedgeConnectivity>(self.hconn_,h).prev_halfedge_
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
    /// let h0 = m.find_halfedge(vvec[0],vvec[1]);
    /// let h1 = m.find_halfedge(vvec[1],vvec[0]);
    /// assert!(m.opposite_halfedge(h0) == h1);
    /// ```
    pub fn opposite_halfedge(&self, h : Halfedge) -> Halfedge {
        let idx = h.idx().unwrap();
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
    /// let h0 = m.find_halfedge(vvec[0],vvec[1]);
    /// let h1 = m.find_halfedge(vvec[0],vvec[2]);
    /// assert!(m.cw_rotated_halfedge(h0) == h1);
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
    /// let h = m.find_halfedge(vvec[0],vvec[1]);
    /// assert!(h.is_valid());
    /// assert!(m.from_vertex(h) == vvec[0]);
    /// assert!(m.to_vertex(h) == vvec[1]);
    /// ```
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
    /// ```
    pub fn add_face(&mut self, vertices : & Vec<Vertex>) -> Face {
        let n = vertices.len();
        let mut hvec = Vec::<Halfedge>::new();
        let mut new_hvec = Vec::<bool>::new();
        hvec.reserve(n);
        new_hvec.reserve(n);

        // Does the face to add is valid
        for i in 0..n {
            if !self.is_boundary_vertex(vertices[i]) {
                return Face::invalid();
            }
            hvec.push(self.find_halfedge(vertices[i],vertices[(i+1)%n]));
            new_hvec.push(!hvec[i].is_valid());
            if !new_hvec[i] && !self.is_boundary_halfedge(hvec[i]) {
                return Face::invalid();
            }
        }

        // Create missing edges
        for i in 0..n {
            if new_hvec[i] {
                let ii = (i+1)%n;
                hvec[i] = self.new_edge(vertices[i], vertices[ii]);
            }
        }

        // Creates the new face
        let f = self.fprop_.push();
        *self.fprop_.access_mut::<FaceConnectivity>(self.fconn_,f) = FaceConnectivity::new(hvec[n-1]);

        // Setup halfedges
        let mut next_cache : Vec<(Halfedge,Halfedge)> = Vec::new();
        let mut needs_adjust : Vec<bool> = Vec::new();
        needs_adjust.resize(n,false);
        for i in 0..n {
            let ii = (i+1)%n;
            let v = vertices[ii];
            let inner_prev = hvec[i];
            let inner_next = hvec[ii];

            if new_hvec[i] || new_hvec[ii] {
                let outer_prev = self.opposite_halfedge(inner_next);
                let outer_next = self.opposite_halfedge(inner_prev);

                if !new_hvec[ii] { // prev is new, next is not
                    let boundary_prev = self.prev_halfedge(inner_next);
                    next_cache.push((boundary_prev,outer_next));
                    self.set_halfedge(v,outer_next);
                } else if !new_hvec[i] { // next is new, prev is not
                    let boundary_next = self.next_halfedge(inner_prev);
                    next_cache.push((outer_prev,boundary_next));
                    self.set_halfedge(v,boundary_next);
                } else { // both are new
                    if !self.halfedge(v).is_valid() {
                        self.set_halfedge(v,outer_next);
                        next_cache.push((outer_prev,outer_next));
                    } else {
                        let boundary_next = self.halfedge(v);
                        let boundary_prev = self.prev_halfedge(boundary_next);
                        next_cache.push((boundary_prev,outer_next));
                        next_cache.push((outer_prev,boundary_next));
                    }
                }
                next_cache.push((inner_prev,inner_next));
            } else {
                needs_adjust[ii] = self.halfedge(v) == inner_next;
            }
            self.set_face(hvec[i],f);
        }

        // process cache
        for (first,second) in next_cache {
            self.set_next_halfedge(first,second);
        }

        // adjust vertices halfedge handle
        for i in 0..n {
            if needs_adjust[i] {
                self.adjust_outgoing_halfedge(vertices[i]);
            }
        }

        return f;
    }

    /// Add a vertex property with default value. If a vertex property with this name already exists, return `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// let pv = m.add_vertex_property::<u32>("v:my_prop",17);
    /// assert!(pv.is_valid());
    /// ```
    pub fn add_vertex_property<D : 'static + Clone>(&mut self, name : & 'static str, default_value : D) -> Property {
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
    /// let pf = m.add_face_property::<u32>("f:my_prop",17);
    /// assert!(pf.is_valid());
    /// ```
    pub fn add_face_property<D : 'static + Clone>(&mut self, name : & 'static str, default_value : D) -> Property {
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
    /// let pe = m.add_edge_property::<u32>("e:my_prop",17);
    /// assert!(pe.is_valid());
    /// ```
    pub fn add_edge_property<D : 'static + Clone>(&mut self, name : & 'static str, default_value : D) -> Property {
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
    /// let ph = m.add_halfedge_property::<u32>("h:my_prop",17);
    /// assert!(ph.is_valid());
    /// ```
    pub fn add_halfedge_property<D : 'static + Clone>(&mut self, name : & 'static str, default_value : D) -> Property {
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
    /// m.add_vertex_property::<u32>("v:my_prop",17);
    /// let pv = m.get_vertex_property::<u32>("v:my_prop");
    /// assert!(pv.is_valid());
    /// ```
    pub fn get_vertex_property<D : 'static + Clone>(&self, name : & 'static str) -> Property {
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
    /// m.add_face_property::<u32>("f:my_prop",17);
    /// let pf = m.get_face_property::<u32>("f:my_prop");
    /// assert!(pf.is_valid());
    /// ```
    pub fn get_face_property<D : 'static + Clone>(&self, name : & 'static str) -> Property {
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
    /// m.add_edge_property::<u32>("e:my_prop",17);
    /// let pe = m.get_edge_property::<u32>("e:my_prop");
    /// assert!(pe.is_valid());
    /// ```
    pub fn get_edge_property<D : 'static + Clone>(&self, name : & 'static str) -> Property {
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
    /// m.add_halfedge_property::<u32>("h:my_prop",17);
    /// let ph = m.get_halfedge_property::<u32>("h:my_prop");
    /// assert!(ph.is_valid());
    /// ```
    pub fn get_halfedge_property<D : 'static + Clone>(&self, name : & 'static str) -> Property {
        self.hprop_.get::<D>(name)
    }

    /// Sets the outgoing `Halfedge` of `Vertex` v to h.
    fn set_halfedge(&mut self, v : Vertex, h : Halfedge) {
        self.vprop_.access_mut::<VertexConnectivity>(self.vconn_,v).halfedge_ = h;
    }

    /// Sets the incident `Face` to `Halfedge` h to f.
    fn set_face(&mut self, h : Halfedge, f : Face) {
        self.hprop_.access_mut::<HalfedgeConnectivity>(self.hconn_,h).face_ = f;
    }

    /// Sets the `Vertex` the `Halfedge` h points to to v.
    fn set_vertex(&mut self, h : Halfedge, v : Vertex) {
        self.hprop_.access_mut::<HalfedgeConnectivity>(self.hconn_,h).vertex_ = v;
    }

    /// Sets the next `Halfedge` of h within the face to nh
    fn set_next_halfedge(&mut self, h : Halfedge, nh : Halfedge) {
        self.hprop_.access_mut::<HalfedgeConnectivity>(self.hconn_,h).next_halfedge_ = nh;
        self.hprop_.access_mut::<HalfedgeConnectivity>(self.hconn_,nh).prev_halfedge_ = h;
    }

    /// Makes sure that the outgoing `Halfedge` of `Vertex` v is boundary halfedge if v is a boundary vertex.
    fn adjust_outgoing_halfedge(&mut self, v : Vertex) {
        let mut h = self.halfedge(v);
        let hh = h;

        if h.is_valid() {
            let mut stop = false;
            while !stop {
                if self.is_boundary_halfedge(h) {
                    self.set_halfedge(v,h);
                    return;
                }
                h = self.cw_rotated_halfedge(h);
                if h == hh {
                    stop = true;
                }
            }
        }
    }

    /// allocate a new edge and returns the `Halfedge` from start to end
    fn new_edge(&mut self, start : Vertex, end : Vertex) -> Halfedge {
        assert!(start != end);

        self.eprop_.push();

        let h0 = self.hprop_.push();
        let h1 = self.hprop_.push();

        self.set_vertex(h0, end);
        self.set_vertex(h1, start);

        return h0;
    }
}

impl PropertyAccess<Vertex> for Mesh {
    /// Access the element of the vertex 'Property' prop indexing by 'Vertex' v.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    /// use lwmesh::property::PropertyAccess;
    ///
    /// let mut m = Mesh::new();
    /// let prop = m.add_vertex_property::<u32>("v:my_prop",17);
    /// let v0 = m.add_vertex();
    /// assert_eq!(*m.access::<u32>(prop,v0),17);
    /// ```
    fn access<D : 'static + Clone>(&self, prop : Property, v : Vertex) -> &D{
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
    /// let prop = m.add_vertex_property::<u32>("v:my_prop",17);
    /// let v0 = m.add_vertex();
    /// assert_eq!(*m.access::<u32>(prop,v0),17);
    /// *m.access_mut::<u32>(prop,v0) = 42;
    /// assert_eq!(*m.access::<u32>(prop,v0),42);
    /// ```
    fn access_mut<D : 'static + Clone>(&mut self, prop : Property, v : Vertex) -> &mut D{
        self.vprop_.access_mut::<D>(prop,v)
    }
}

impl PropertyAccess<Face> for Mesh {
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
    /// let prop = m.add_face_property::<u32>("f:my_prop",17);
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// assert_eq!(*m.access::<u32>(prop,f),17);
    /// ```
    fn access<D : 'static + Clone>(&self, prop : Property, f : Face) -> &D{
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
    /// let prop = m.add_face_property::<u32>("f:my_prop",17);
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// let f = m.add_face(&vvec);
    /// assert_eq!(*m.access::<u32>(prop,f),17);
    /// *m.access_mut::<u32>(prop,f) = 42;
    /// assert_eq!(*m.access::<u32>(prop,f),42);
    /// ```
    fn access_mut<D : 'static + Clone>(&mut self, prop : Property, f : Face) -> &mut D{
        self.fprop_.access_mut::<D>(prop,f)
    }
}

impl PropertyAccess<Edge> for Mesh {
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
    /// let prop = m.add_edge_property::<u32>("e:my_prop",17);
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    /// let e = m.edge(m.find_halfedge(vvec[0],vvec[1]));
    /// assert_eq!(*m.access::<u32>(prop,e),17);
    /// ```
    fn access<D : 'static + Clone>(&self, prop : Property, e : Edge) -> &D{
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
    /// let prop = m.add_edge_property::<u32>("e:my_prop",17);
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    /// let e = m.edge(m.find_halfedge(vvec[0],vvec[1]));
    /// assert_eq!(*m.access::<u32>(prop,e),17);
    /// *m.access_mut::<u32>(prop,e) = 42;
    /// assert_eq!(*m.access::<u32>(prop,e),42);
    /// ```
    fn access_mut<D : 'static + Clone>(&mut self, prop : Property, e : Edge) -> &mut D{
        self.eprop_.access_mut::<D>(prop,e)
    }
}

impl PropertyAccess<Halfedge> for Mesh {
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
    /// let prop = m.add_halfedge_property::<u32>("h:my_prop",17);
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    /// let h = m.find_halfedge(vvec[0],vvec[1]);
    /// assert_eq!(*m.access::<u32>(prop,h),17);
    /// *m.access_mut::<u32>(prop,h) = 42;
    /// assert_eq!(*m.access::<u32>(prop,h),42);
    /// ```
    fn access<D : 'static + Clone>(&self, prop : Property, h : Halfedge) -> &D{
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
    /// let prop = m.add_halfedge_property::<u32>("h:my_prop",17);
    /// let mut vvec = Vec::<Vertex>::new();
    /// for _ in 0..3 {
    ///     vvec.push(m.add_vertex());
    /// }
    /// m.add_face(&vvec);
    /// let h = m.find_halfedge(vvec[0],vvec[1]);
    /// assert_eq!(*m.access::<u32>(prop,h),17);
    /// *m.access_mut::<u32>(prop,h) = 42;
    /// assert_eq!(*m.access::<u32>(prop,h),42);
    /// ```
    fn access_mut<D : 'static + Clone>(&mut self, prop : Property, h : Halfedge) -> &mut D{
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
        assert!(m.vprop_.len() == 0);

        let v0 = m.add_vertex();
        assert!(m.vprop_.len() == 1);
        assert!(v0.idx().unwrap() == 0);

        m.add_vertex();
        let v2 = m.add_vertex();
        assert!(m.vprop_.len() == 3);
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
        assert!(f.is_valid());
        assert!(m.n_faces() == 1);

        vvec.clear();
        vvec.push(v2);
        vvec.push(v1);
        vvec.push(v3);
        let f = m.add_face(&vvec);
        assert!(f.is_valid());
        assert!(m.n_faces() == 2);

        let f = m.add_face(&vvec);
        assert!(!f.is_valid());
        assert!(m.n_faces() == 2);

        let v4 = m.add_vertex();
        vvec.clear();
        vvec.push(v2);
        vvec.push(v1);
        vvec.push(v4);
        let f = m.add_face(&vvec);
        assert!(!f.is_valid());
        assert!(m.n_faces() == 2);
    }

    #[test]
    fn property() {
        let mut m = Mesh::new();

        // Vertex
        let prop = m.add_vertex_property::<u32>("v:my_prop",17);
        let v0 = m.add_vertex();
        assert_eq!(*m.access::<u32>(prop,v0),17);
        *m.access_mut::<u32>(prop,v0) = 42;
        assert_eq!(*m.access::<u32>(prop,v0),42);

        // Face
        let prop = m.add_face_property::<u32>("f:my_prop",17);
        let mut vvec = Vec::<Vertex>::new();
        let v1 = m.add_vertex();
        let v2 = m.add_vertex();
        vvec.push(v0);
        vvec.push(v1);
        vvec.push(v2);
        let f = m.add_face(&vvec);
        assert_eq!(*m.access::<u32>(prop,f),17);
        *m.access_mut::<u32>(prop,f) = 42;
        assert_eq!(*m.access::<u32>(prop,f),42);

        // Edge
        let prop = m.add_edge_property::<u32>("v:my_prop",17);
        let e = m.edge(m.find_halfedge(v0,v1));
        assert_eq!(*m.access::<u32>(prop,e),17);
        *m.access_mut::<u32>(prop,e) = 42;
        assert_eq!(*m.access::<u32>(prop,e),42);

        // Halfedge
        let prop = m.add_halfedge_property::<u32>("v:my_prop",17);
        let h = m.find_halfedge(v2,v0);
        assert_eq!(*m.access::<u32>(prop,h),17);
        *m.access_mut::<u32>(prop,h) = 42;
        assert_eq!(*m.access::<u32>(prop,h),42);
    }

    #[test]
    #[should_panic]
    fn invalid_property() {
        let mut m = Mesh::new();
        let prop = m.get_vertex_property::<u32>("v:my_prop");
        let v0 = m.add_vertex();
        m.access::<u32>(prop,v0);
    }

    #[test]
    #[should_panic]
    fn mixing_property() {
        let mut m = Mesh::new();
        let prop = m.add_face_property::<u32>("v:my_prop",17);
        let v0 = m.add_vertex();
        m.access::<u32>(prop,v0);
    }

}
