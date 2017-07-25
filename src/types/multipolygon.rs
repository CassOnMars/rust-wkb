use types::polygon::*;
use types::geometrytype::*;

#[derive(Debug, Default)]
pub struct MultiPolygon {
  pub num_polygons: u32,
  pub polygons: Vec<Polygon>
}

#[derive(Debug, Default)]
pub struct MultiPolygonZ {
  pub num_polygons: u32,
  pub polygons: Vec<PolygonZ>
}

#[derive(Debug, Default)]
pub struct MultiPolygonM {
  pub num_polygons: u32,
  pub polygons: Vec<PolygonM>
}

#[derive(Debug, Default)]
pub struct MultiPolygonZM {
  pub num_polygons: u32,
  pub polygons: Vec<PolygonZM>
}

impl Geometric for MultiPolygon {
  fn geometry_type() -> GeometryType { GeometryType::MultiPolygon }
}

impl Geometric for MultiPolygonZ {
  fn geometry_type() -> GeometryType { GeometryType::MultiPolygon }
}

impl Geometric for MultiPolygonM {
  fn geometry_type() -> GeometryType { GeometryType::MultiPolygon }
}

impl Geometric for MultiPolygonZM {
  fn geometry_type() -> GeometryType { GeometryType::MultiPolygon }
}
