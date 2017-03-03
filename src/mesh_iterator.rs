use mesh::*;
use handle::*;

pub struct VertexIterator<'a> {
    topology_ : &'a Topology,
    curr_ : Vertex,
}

impl<'a> Iterator for VertexIterator<'a> {
    type Item = Vertex;

    fn next(&mut self) -> Option<Vertex> {
        let v = self.curr_;
        self.curr_ = Vertex::new(v.idx().unwrap()+1);
        if self.topology_.n_vertices() <= v.idx().unwrap() {
            return None;
        } else {
            return Some(v);
        }
    }
}

pub struct FaceIterator<'a> {
    topology_ : &'a Topology,
    curr_ : Face,
}

impl<'a> Iterator for FaceIterator<'a> {
    type Item = Face;

    fn next(&mut self) -> Option<Face> {
        let f = self.curr_;
        self.curr_ = Face::new(f.idx().unwrap()+1);
        if self.topology_.n_faces() <= f.idx().unwrap() {
            return None;
        } else {
            return Some(f);
        }
    }
}

pub struct EdgeIterator<'a> {
    topology_ : &'a Topology,
    curr_ : Edge,
}

impl<'a> Iterator for EdgeIterator<'a> {
    type Item = Edge;

    fn next(&mut self) -> Option<Edge> {
        let e = self.curr_;
        self.curr_ = Edge::new(e.idx().unwrap()+1);
        if self.topology_.n_edges() <= e.idx().unwrap() {
            return None;
        } else {
            return Some(e);
        }
    }
}

pub struct HalfedgeIterator<'a> {
    topology_ : &'a Topology,
    curr_ : Halfedge,
}

impl<'a> Iterator for HalfedgeIterator<'a> {
    type Item = Halfedge;

    fn next(&mut self) -> Option<Halfedge> {
        let h = self.curr_;
        self.curr_ = Halfedge::new(h.idx().unwrap()+1);
        if self.topology_.n_halfedges() <= h.idx().unwrap() {
            return None;
        } else {
            return Some(h);
        }
    }
}

pub struct VerticesAroundVertexCirculator<'a> {
    topology_ : &'a Topology,
    end_ : Halfedge,
    curr_ : Halfedge,
    active_ : bool
}

impl<'a> Iterator for VerticesAroundVertexCirculator<'a> {
    type Item = Vertex;

    fn next(&mut self) -> Option<Vertex> {
        if !self.curr_.is_valid() {
            return None;
        }
        if self.active_ && self.curr_ == self.end_ {
            return None;
        }
        self.active_ = true;
        let v = self.topology_.to_vertex(self.curr_);
        self.curr_ = self.topology_.cw_rotated_halfedge(self.curr_);
        return Some(v);
    }
}

pub struct HalfedgesAroundVertexCirculator<'a> {
    topology_ : &'a Topology,
    end_ : Halfedge,
    curr_ : Halfedge,
    active_ : bool
}

impl<'a> Iterator for HalfedgesAroundVertexCirculator<'a> {
    type Item = Halfedge;

    fn next(&mut self) -> Option<Halfedge> {
        if !self.curr_.is_valid() {
            return None;
        }
        if self.active_ && self.curr_ == self.end_ {
            return None;
        }
        self.active_ = true;
        let h = self.curr_;
        self.curr_ = self.topology_.cw_rotated_halfedge(self.curr_);
        return Some(h);
    }
}

pub struct FacesAroundVertexCirculator<'a> {
    topology_ : &'a Topology,
    end_ : Halfedge,
    curr_ : Halfedge,
    active_ : bool
}

impl<'a> Iterator for FacesAroundVertexCirculator<'a> {
    type Item = Face;

    fn next(&mut self) -> Option<Face> {
        if !self.curr_.is_valid() {
            return None;
        }
        if self.active_ && self.curr_ == self.end_ {
            return None;
        }
        self.active_ = true;
        let f = self.topology_.face(self.curr_);
        loop {
            self.curr_ = self.topology_.cw_rotated_halfedge(self.curr_);
            if !self.topology_.is_boundary_halfedge(self.curr_) {break;}
        }
        return Some(f);
    }
}

impl Topology {
    /// Iterator over the vertices in the `Mesh`
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::new();
    /// m.add_vertex();
    /// m.add_vertex();
    /// m.add_vertex();
    ///
    /// for v in m.topology.vertices() {
    ///     println!("v{}",v.idx().unwrap())
    /// }
    /// ```
    pub fn vertices(&self) -> VertexIterator {
        VertexIterator {
            topology_ : &self,
            curr_ : Vertex::new(0),
        }
    }

    /// Iterator over the faces in the `Mesh`
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
    /// m.add_face(&vvec);
    ///
    /// for f in m.topology.faces() {
    ///     println!("f{}",f.idx().unwrap())
    /// }
    /// ```
    pub fn faces(&self) -> FaceIterator {
        FaceIterator {
            topology_ : &self,
            curr_ : Face::new(0),
        }
    }

    /// Iterator over the edges in the `Mesh`
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
    /// m.add_face(&vvec);
    ///
    /// for e in m.topology.edges() {
    ///     println!("e{}",e.idx().unwrap())
    /// }
    /// ```
    pub fn edges(&self) -> EdgeIterator {
        EdgeIterator {
            topology_ : &self,
            curr_ : Edge::new(0),
        }
    }

    /// Iterator over the halfedges in the `Mesh`
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
    /// m.add_face(&vvec);
    ///
    /// for h in m.topology.halfedges() {
    ///     println!("h{}",h.idx().unwrap())
    /// }
    /// ```
    pub fn halfedges(&self) -> HalfedgeIterator {
        HalfedgeIterator {
            topology_ : &self,
            curr_ : Halfedge::new(0),
        }
    }

    /// Iterator over the vertices around a vertex in the `Mesh`
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
    /// m.add_face(&vvec);
    ///
    /// for v in m.topology.vertices_around_vertex(vvec[0]) {
    ///     println!("v{}",v.idx().unwrap());
    /// }
    /// ```
    pub fn vertices_around_vertex(&self, v : Vertex) -> VerticesAroundVertexCirculator {
        VerticesAroundVertexCirculator {
            topology_ : &self,
            end_ : self.halfedge(v),
            curr_ : self.halfedge(v),
            active_ : false
        }
    }

    /// Iterator over the halfedges around a vertex in the `Mesh`
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
    /// m.add_face(&vvec);
    ///
    /// for h in m.topology.halfedges_around_vertex(vvec[0]) {
    ///     println!("h{}",h.idx().unwrap());
    /// }
    /// ```
    pub fn halfedges_around_vertex(&self, v : Vertex) -> HalfedgesAroundVertexCirculator {
        HalfedgesAroundVertexCirculator {
            topology_ : &self,
            end_ : self.halfedge(v),
            curr_ : self.halfedge(v),
            active_ : false
        }
    }

    /// Iterator over the halfedges around a vertex in the `Mesh`
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
    /// m.add_face(&vvec);
    ///
    /// for f in m.topology.faces_around_vertex(vvec[0]) {
    ///     println!("f{}",f.idx().unwrap());
    /// }
    /// ```
    pub fn faces_around_vertex(&self, v : Vertex) -> FacesAroundVertexCirculator {
        let mut cir = FacesAroundVertexCirculator {
            topology_ : &self,
            end_ : self.halfedge(v),
            curr_ : self.halfedge(v),
            active_ : false
        };
        cir.next();
        cir.end_ = cir.curr_;
        cir.active_ = false;
        return cir;
    }
}

#[cfg(test)]
mod tests {
    use mesh::*;
    use property::PropertyAccess;
    use handle::Vertex;

    #[test]
    fn iterator_and_properties() {
        let mut m = Mesh::new();
        let vprop = m.properties.add_vertex_property::<u32>("v:my_prop",17);
        let fprop = m.properties.add_face_property::<u32>("f:my_prop",17);
        let eprop = m.properties.add_edge_property::<u32>("e:my_prop",17);
        let hprop = m.properties.add_halfedge_property::<u32>("h:my_prop",17);
        let mut vvec = Vec::<Vertex>::new();
        for _ in 0..3 {
            vvec.push(m.add_vertex());
        }
        m.add_face(&vvec);

        for v in m.topology.vertices() {
            *m.properties.access_mut::<u32>(vprop,v) += 1;
            assert_eq!(18,*m.properties.access::<u32>(vprop,v));
        }

        for f in m.topology.faces() {
            *m.properties.access_mut::<u32>(fprop,f) += 1;
            assert_eq!(18,*m.properties.access::<u32>(fprop,f));
        }

        for e in m.topology.edges() {
            *m.properties.access_mut::<u32>(eprop,e) += 1;
            assert_eq!(18,*m.properties.access::<u32>(eprop,e));
        }

        for h in m.topology.halfedges() {
            *m.properties.access_mut::<u32>(hprop,h) += 1;
            assert_eq!(18,*m.properties.access::<u32>(hprop,h));
        }
    }

    #[test]
    fn around_iterator() {
        let mut m = Mesh::new();
        let mut vvec = Vec::<Vertex>::new();
        for _ in 0..3 {
            vvec.push(m.add_vertex());
        }
        m.add_face(&vvec);

        let mut i = 0;
        for _ in m.topology.vertices_around_vertex(vvec[0]) {
            i += 1;
        }
        assert_eq!(i,2);
    }
}
