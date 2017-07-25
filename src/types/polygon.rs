use types::geometrytype::*;
use types::linearring::*;

#[derive(Debug, Default)]
pub struct Polygon {
  pub num_rings: u32,
  pub rings: Vec<LinearRing>
}

#[derive(Debug, Default)]
pub struct PolygonZ {
  pub num_rings: u32,
  pub rings: Vec<LinearRingZ>
}

#[derive(Debug, Default)]
pub struct PolygonM {
  pub num_rings: u32,
  pub rings: Vec<LinearRingM>
}

#[derive(Debug, Default)]
pub struct PolygonZM {
  pub num_rings: u32,
  pub rings: Vec<LinearRingZM>
}

impl Geometric for Polygon {
  fn geometry_type() -> GeometryType { GeometryType::Polygon }
}

impl Geometric for PolygonZ {
  fn geometry_type() -> GeometryType { GeometryType::Polygon }
}

impl Geometric for PolygonM {
  fn geometry_type() -> GeometryType { GeometryType::Polygon }
}

impl Geometric for PolygonZM {
  fn geometry_type() -> GeometryType { GeometryType::Polygon }
}
