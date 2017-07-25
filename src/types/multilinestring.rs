use types::linestring::*;
use types::geometrytype::*;

#[derive(Debug, Default)]
pub struct MultiLineString {
  pub num_linestrings: u32,
  pub linestrings: Vec<LineString>
}

#[derive(Debug, Default)]
pub struct MultiLineStringZ {
  pub num_linestrings: u32,
  pub linestrings: Vec<LineStringZ>
}

#[derive(Debug, Default)]
pub struct MultiLineStringM {
  pub num_linestrings: u32,
  pub linestrings: Vec<LineStringM>
}

#[derive(Debug, Default)]
pub struct MultiLineStringZM {
  pub num_linestrings: u32,
  pub linestrings: Vec<LineStringZM>
}

impl Geometric for MultiLineString {
  fn geometry_type() -> GeometryType { GeometryType::MultiLineString }
}

impl Geometric for MultiLineStringZ {
  fn geometry_type() -> GeometryType { GeometryType::MultiLineString }
}

impl Geometric for MultiLineStringM {
  fn geometry_type() -> GeometryType { GeometryType::MultiLineString }
}

impl Geometric for MultiLineStringZM {
  fn geometry_type() -> GeometryType { GeometryType::MultiLineString }
}
