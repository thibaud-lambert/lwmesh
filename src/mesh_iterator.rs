use mesh::*;
use handle::*;
use std::marker::PhantomData;

pub struct VertexIterator<'a> {
    mesh_ : *mut Mesh,
    curr_ : Vertex,
    dummy_: PhantomData<&'a Mesh>
}

impl<'a> Iterator for VertexIterator<'a> {
    type Item = (&'a mut Mesh,Vertex);

    fn next(&mut self) -> Option<(&'a mut Mesh,Vertex)> {
        let v = self.curr_;
        self.curr_ = Vertex::new(v.idx().unwrap()+1);
        let mesh : &mut Mesh = unsafe { &mut*self.mesh_ };
        if mesh.n_vertices() <= v.idx().unwrap() {
            return None;
        } else {
            return Some((mesh,v));
        }
    }
}

impl Mesh {
    pub fn vertex_iterator(&mut self) -> VertexIterator {
        VertexIterator {
            mesh_ : self,
            curr_ : Vertex::new(0),
            dummy_ : PhantomData
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mesh::*;
    use property::PropertyAccess;

    #[test]
    fn vertex_iterator() {
        let mut m = Mesh::new();
        let prop = m.add_vertex_property::<u32>("v:my_prop",17);
        m.add_vertex();
        m.add_vertex();
        m.add_vertex();

        for (me,v) in m.vertex_iterator() {
            println!("v{} : my_prop {}",v.idx().unwrap(),me.access::<u32>(prop,v));
        }

    }
}
