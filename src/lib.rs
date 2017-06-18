#[cfg(feature = "lwobj")]
extern crate lwobj;
#[cfg(feature = "nalgebra")]
extern crate nalgebra;

mod handle;
pub use handle::Vertex;
pub use handle::Face;
pub use handle::Edge;
pub use handle::Halfedge;
pub use handle::PropertyVertex;
pub use handle::PropertyFace;
pub use handle::PropertyEdge;
pub use handle::PropertyHalfedge;

mod connectivity;
mod property;
mod mesh;
#[cfg(feature = "mesh_io")]
mod mesh_io;
pub use mesh::Mesh;
pub use mesh::Topology;
pub use mesh::Properties;
pub mod mesh_iterator;
pub use mesh_iterator::VerticesAround;
pub use mesh_iterator::HalfedgesAround;
pub use mesh_iterator::FacesAround;
