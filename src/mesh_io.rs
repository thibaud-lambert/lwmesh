use mesh::*;
use property::*;
use std::path::Path;
use std::fs::File;
use std::io;
use std::ffi::OsString;
use std::io::BufReader;
#[cfg(feature = "obj-rs")]
use obj::ObjError;
#[cfg(feature = "obj-rs")]
use obj::raw::*;
use handle::*;

#[derive(Debug)]
pub enum MeshLoadingError {
    NoExtension,
    UnknwonExtension(OsString),
    Io(io::Error),
    #[cfg(feature = "obj-rs")]
    Obj(ObjError)
}

impl From<io::Error> for MeshLoadingError {
    fn from(err : io::Error) -> MeshLoadingError {
        MeshLoadingError::Io(err)
    }
}

#[cfg(feature = "obj-rs")]
impl From<ObjError> for MeshLoadingError {
    fn from(err : ObjError) -> MeshLoadingError {
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
    /// #[cfg(feature = "obj-rs")]
    /// let mut m = Mesh::load("cube.obj").ok().unwrap();
    /// ```
    pub fn load(filename : & 'static str) -> Result<Mesh,MeshLoadingError> {
        let path = Path::new(filename);
        match path.extension() {
            None => Err(MeshLoadingError::NoExtension),
            Some(ext) => {
                if cfg!(feature = "obj-rs") && ext == "obj" {
                    let f = try!(File::open(filename));
                    let input = BufReader::new(f);
                    #[cfg(feature = "obj-rs")]
                    return Mesh::load_obj(input);
                    #[cfg(not(feature = "obj-rs"))]
                    return Err(MeshLoadingError::UnknwonExtension(ext.to_os_string()));
                } else {
                    return Err(MeshLoadingError::UnknwonExtension(ext.to_os_string()));
                }
            },
        }
    }

    #[cfg(feature = "obj-rs")]
    /// Load Obj `Mesh` from  a `BufReader`.
    ///
    /// # Examples
    ///
    /// ```
    /// use lwmesh::mesh::Mesh;
    ///
    /// let mut m = Mesh::load("cube.obj").ok().unwrap();
    /// ```
    pub fn load_obj<R : io::Read>(input : BufReader<R>) -> Result<Mesh,MeshLoadingError> {
        let obj: RawObj = try!(parse_obj(input));

        let mut m = Mesh::new();
        let pos = m.properties.add_vertex_property::<[f32; 3]>("v:position",[0.;3]).unwrap();
        for (x,y,z,_) in obj.positions {
            let v = m.add_vertex();
            *m.properties.access_mut::<[f32; 3]>(pos,v) = [x,y,z];
        }
        for poly in obj.polygons {
            let mut vvec : Vec<Vertex> = Vec::new();
            match poly {
                object::Polygon::P(idx) => {
                    for i in idx {
                        vvec.push(Vertex::new(i));
                    }
                },
                object::Polygon::PN(idx) | object::Polygon::PT(idx) => {
                    for i in idx {
                        vvec.push(Vertex::new(i.0));
                    }
                },
                object::Polygon::PTN(idx) => {
                    for i in idx {
                        vvec.push(Vertex::new(i.0));
                    }
                }
            }
            m.add_face(&vvec);
        }

        return Ok(m);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mesh::*;
    use std::io::ErrorKind;

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
    #[cfg(feature = "obj-rs")]
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
    #[cfg(feature = "obj-rs")]
    fn load_obj() {
        let res = Mesh::load("cube.obj");
        assert!(res.is_ok());
        let m = res.ok().unwrap();
        assert_eq!(m.topology.n_faces(),12);
        assert_eq!(m.topology.n_vertices(),8);
    }

}
