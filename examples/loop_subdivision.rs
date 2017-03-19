extern crate lwmesh;
extern crate nalgebra;
use lwmesh::mesh::Mesh;
use lwmesh::handle::*;
use nalgebra::Vector3;

fn loop_subdivision(m : &mut Mesh) -> Mesh {
    let vmap = m.properties.add_vertex_property::<Option<Vertex>>("v:mapping",None).unwrap();
    let emap = m.properties.add_edge_property::<Option<Vertex>>("e:mapping",None).unwrap();
    let pos = m.properties.get_vertex_property::<Vector3<f32>>("v:position").unwrap();

    let mut subdiv = Mesh::new();
    let spos = subdiv.properties.add_vertex_property::<Vector3<f32>>("v:position",Vector3::new(0f32,0f32,0f32)).unwrap();

    // Add existing vertices to their new positions
    subdiv.vertex_reserve(m.topology.n_vertices());
    for v in m.topology.vertices() {
        let sv = subdiv.add_vertex();
        m.properties[(vmap,v)] = Some(sv);
        let mut new_pos = Vector3::<f32>::new(0.,0.,0.);
        let mut deg = 0u32;
        for u in m.topology.vertices_around_vertex(v) {
            new_pos += m.properties[(pos,u)];
            deg+=1;
        }
        let beta : f32;
        if deg == 3 {
            beta = 3./16.;
        } else if deg > 3 {
            let tmp = 3./8.+(2.*std::f32::consts::PI/deg as f32).cos()/4.;
            beta = (5f32/8f32-tmp*tmp)/deg as f32;
        } else {
            panic!();
        }
        new_pos *= beta;
        new_pos += (1.-beta*deg as f32)*(m.properties[(pos,v)]);
        subdiv.properties[(spos,sv)] = new_pos;
    }

    for e in m.topology.edges() {
        let sv = subdiv.add_vertex();
        m.properties[(emap,e)] = Some(sv);
        let h = m.topology.edge_halfedge(e,0);
        let v0 = m.topology.to_vertex(h);
        let v2 = m.topology.from_vertex(h);
        let v1 = m.topology.to_vertex(m.topology.next_halfedge(h));
        let v3 = m.topology.to_vertex(m.topology.next_halfedge(m.topology.opposite_halfedge(h)));
        let mut new_pos = Vector3::<f32>::new(0.,0.,0.);
        new_pos += 3./8. * m.properties[(pos,v0)];
        new_pos += 3./8. * m.properties[(pos,v2)];
        new_pos += 1./8. * m.properties[(pos,v1)];
        new_pos += 1./8. * m.properties[(pos,v3)];
        subdiv.properties[(spos,sv)] = new_pos;
    }

    for f in m.topology.faces() {
        let mut v : [Vertex;3] = [Vertex::new(0);3];
        let mut ve : [Vertex;3] = [Vertex::new(0);3];
        for (i,h) in m.topology.halfedges_around_face(f).enumerate() {
            let e = m.topology.edge(h);
            ve[i] = m.properties[(emap,e)].unwrap();
            v[i] = m.topology.from_vertex(h);
        }
        subdiv.add_face(&vec![v[0],ve[0],ve[2]]);
        subdiv.add_face(&vec![ve[0],ve[1],ve[2]]);
        subdiv.add_face(&vec![ve[0],v[1],ve[1]]);
        subdiv.add_face(&vec![ve[1],v[2],ve[2]]);
    }
    return subdiv;
}

fn main() {
    let m = Mesh::load("./examples/ico.obj").ok().unwrap();
    let mut subdiv = m;
    for _ in 1..5 {
        subdiv = loop_subdivision(&mut subdiv);
    }
    assert!(subdiv.write("loop.obj").is_ok());
}
