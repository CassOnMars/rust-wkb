use types::point::*;
use types::geometrytype::*;

#[derive(Debug, Default)]
pub struct MultiPoint {
  pub num_points: u32,
  pub points: Vec<Point>
}

#[derive(Debug, Default)]
pub struct MultiPointZ {
  pub num_points: u32,
  pub points: Vec<PointZ>
}

#[derive(Debug, Default)]
pub struct MultiPointM {
  pub num_points: u32,
  pub points: Vec<PointM>
}

#[derive(Debug, Default)]
pub struct MultiPointZM {
  pub num_points: u32,
  pub points: Vec<PointZM>
}

impl Geometric for MultiPoint {
  fn geometry_type() -> GeometryType { GeometryType::MultiPoint }
}

impl Geometric for MultiPointZ {
  fn geometry_type() -> GeometryType { GeometryType::MultiPoint }
}

impl Geometric for MultiPointM {
  fn geometry_type() -> GeometryType { GeometryType::MultiPoint }
}

impl Geometric for MultiPointZM {
  fn geometry_type() -> GeometryType { GeometryType::MultiPoint }
}

