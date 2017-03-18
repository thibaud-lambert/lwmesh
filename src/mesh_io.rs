use std::path::Path;
use std::fs::File;
use std::io;
use std::ffi::OsString;
use std::io::BufReader;
use mesh::*;
use property::*;
use handle::*;
use lwobj::*;
use nalgebra::Vector3;

#[derive(Debug)]
pub enum MeshLoadingError {
    NoExtension,
    MissingPosition,
    UnknwonExtension(OsString),
    Io(io::Error),
    Obj(LoadingError)
}

impl From<io::Error> for MeshLoadingError {
    fn from(err : io::Error) -> MeshLoadingError {
        MeshLoadingError::Io(err)
    }
}

impl From<LoadingError> for MeshLoadingError {
    fn from(err : LoadingError) -> MeshLoadingError {
        MeshLoadingError::Obj(err)
    }
}

impl Mesh {
    /// Load `Mesh` from file base on the extension.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::load("cube.obj").ok().unwrap();
    /// ```
    pub fn load(filename : & 'static str) -> Result<Mesh,MeshLoadingError> {
        let path = Path::new(filename);
        match path.extension() {
            None => Err(MeshLoadingError::NoExtension),
            Some(ext) => {
                if ext == "obj" {
                    let f = try!(File::open(filename));
                    let mut input = BufReader::new(f);
                    return Mesh::load_obj(&mut input);
                } else {
                    return Err(MeshLoadingError::UnknwonExtension(ext.to_os_string()));
                }
            },
        }
    }

    /// Load Obj `Mesh` from  a `BufReader`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::load("cube.obj").ok().unwrap();
    /// ```
    pub fn load_obj<R : io::Read>(input : &mut BufReader<R>) -> Result<Mesh,MeshLoadingError> {
        let obj: ObjData = try!(ObjData::load(input));

        let mut m = Mesh::new();
        let pos = m.properties.add_vertex_property::<Vector3<f32>>("v:position",Vector3::new(0f32,0f32,0f32)).unwrap();
        for (x,y,z,_) in obj.vertices {
            let v = m.add_vertex();
            *m.properties.access_mut::<Vector3<f32>>(pos,v) = Vector3::new(x,y,z);
        }
        for f in obj.faces {
            let mut vvec : Vec<Vertex> = Vec::new();
            for (i,_,_) in f {
                vvec.push(Vertex::new(i));
            }
            m.add_face(&vvec);
        }
        return Ok(m);
    }

    /// Write `Mesh` base on the extension.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::load("cube.obj").ok().unwrap();
    /// ```
    pub fn write(&self, filename : & 'static str) -> Result<(),MeshLoadingError> {
        let path = Path::new(filename);
        match path.extension() {
            None => Err(MeshLoadingError::NoExtension),
            Some(ext) => {
                if ext == "obj" {
                    let f = try!(File::create(filename));
                    let mut output = io::BufWriter::new(f);
                    return self.write_obj(&mut output);
                } else {
                    return Err(MeshLoadingError::UnknwonExtension(ext.to_os_string()));
                }
            },
        }
    }

    /// Write Obj `Mesh` into a `BufWriter`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::load("cube.obj").ok().unwrap();
    /// m.write("cube2.obj");
    /// ```
    pub fn write_obj<R : io::Write>(&self, output : &mut io::BufWriter<R>) -> Result<(),MeshLoadingError> {
        let mut obj_data: ObjData = ObjData::new();
        let posprop = match self.properties.get_vertex_property::<Vector3<f32>>("v:position") {
            Some(prop) => prop,
            None => return Err(MeshLoadingError::MissingPosition),
        };
        // let posprop = self.properties.get_vertex_property::<Vector3<f32>>("v:position").unwrap();
        for v in self.topology.vertices() {
            let pos = self.properties.access::<Vector3<f32>>(posprop, v);
            obj_data.vertices.push((pos.x,pos.y,pos.z,1.));
        }
        let mut obj = Object {
            name : String::from(""),
            primitives : Vec::new(),
        };
        for f in self.topology.faces() {
            let mut findex : Vec<(usize,Option<usize>,Option<usize>)> = Vec::new();
            for v in self.topology.vertices_around_face(f) {
                findex.push((v.idx(),None,None));
            }
            obj.primitives.push(f.idx());
            obj_data.faces.push(findex);
        }
        obj_data.objects = vec![obj];
        try!(obj_data.write(output));
        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mesh::Mesh;
    use std::io::ErrorKind;
    use std::io::BufWriter;
    use handle::Vertex;
    use nalgebra::Vector3;
    use property::PropertyAccess;
    use std::str;

    #[test]
    fn load_no_extension() {
        match Mesh::load("cube") {
            Err(MeshLoadingError::NoExtension) => {}
            _ => assert!(false),
        }
    }

    #[test]
    fn load_unknown_extension() {
        match Mesh::load("cube.abc") {
            Err(MeshLoadingError::UnknwonExtension(s)) => assert_eq!(s,*"abc"),
            _ => assert!(false),
        }
    }

    #[test]
    fn load_non_existing_obj_file() {
        match Mesh::load("dummy.obj") {
            Err(MeshLoadingError::Io(err)) => {
                match err.kind() {
                    ErrorKind::NotFound => {}
                    _ => assert!(false),
                }
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn load_obj() {
        let res = Mesh::load("cube.obj");
        assert!(res.is_ok());
        let m = res.ok().unwrap();
        assert_eq!(m.topology.n_faces(),12);
        assert_eq!(m.topology.n_vertices(),8);
    }

    #[test]
    fn write_missing_position() {
        let m = Mesh::new();
        match m.write("cube.abc") {
            Err(MeshLoadingError::UnknwonExtension(s)) => assert_eq!(s,*"abc"),
            _ => assert!(false),
        }
    }

    #[test]
    fn write_unknown_extension() {
        let m = Mesh::new();
        match m.write("cube2.obj") {
            Err(MeshLoadingError::MissingPosition) => {},
            _ => assert!(false),
        }
    }

    #[test]
    fn write_obj() {
        let mut m = Mesh::new();
        let pos = m.properties.add_vertex_property::<Vector3<f32>>("v:position",Vector3::new(0f32,0f32,0f32)).unwrap();
        let v0 = m.add_vertex();
        *m.properties.access_mut::<Vector3<f32>>(pos,v0) = Vector3::new(0.,0.,0.);
        let v1 = m.add_vertex();
        *m.properties.access_mut::<Vector3<f32>>(pos,v1) = Vector3::new(0.,0.5,1.);
        let v2 = m.add_vertex();
        *m.properties.access_mut::<Vector3<f32>>(pos,v2) = Vector3::new(0.5,1.,0.);
        let v3 = m.add_vertex();
        *m.properties.access_mut::<Vector3<f32>>(pos,v3) = Vector3::new(1.,1.,1.);
        let mut vvec = Vec::<Vertex>::new();
        vvec.push(v0);
        vvec.push(v1);
        vvec.push(v2);
        m.add_face(&vvec);
        vvec.clear();
        vvec.push(v2);
        vvec.push(v1);
        vvec.push(v3);
        m.add_face(&vvec);
        let expected =
        r#"v 0 0 0 1
v 0 0.5 1 1
v 0.5 1 0 1
v 1 1 1 1
f 1// 2// 3//
f 3// 2// 4//
"#;

        let mut output = BufWriter::new(Vec::<u8>::new());
        assert!(m.write_obj(&mut output).is_ok());
        let buf = output.into_inner().unwrap();
        println!("{}",str::from_utf8(&buf).unwrap());
        assert_eq!(expected,str::from_utf8(&buf).unwrap());
    }

    #[test]
    fn load_write_obj() {
        let m = Mesh::load("cube.obj").ok().unwrap();
        assert!(m.write("cube2.obj").is_ok());
    }
}
