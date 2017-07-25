use types::coords::*;
use types::geometrytype::*;

#[derive(Debug, Default)]
pub struct LineString {
  pub num_coords: u32,
  pub coords: Vec<Coords>
}

#[derive(Debug, Default)]
pub struct LineStringZ {
  pub num_coords: u32,
  pub coords: Vec<CoordsZ>
}

#[derive(Debug, Default)]
pub struct LineStringM {
  pub num_coords: u32,
  pub coords: Vec<CoordsM>
}

#[derive(Debug, Default)]
pub struct LineStringZM {
  pub num_coords: u32,
  pub coords: Vec<CoordsZM>
}

impl Geometric for LineString {
  fn geometry_type() -> GeometryType { GeometryType::LineString }
}

impl Geometric for LineStringZ {
  fn geometry_type() -> GeometryType { GeometryType::LineString }
}

impl Geometric for LineStringM {
  fn geometry_type() -> GeometryType { GeometryType::LineString }
}

impl Geometric for LineStringZM {
  fn geometry_type() -> GeometryType { GeometryType::LineString }
}
