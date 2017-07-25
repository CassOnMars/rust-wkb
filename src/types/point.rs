use types::coords::*;
use types::geometrytype::*;

#[derive(Debug, Default)]
pub struct Point {
  pub coords: Coords
}

#[derive(Debug, Default)]
pub struct PointZ {
  pub coords: CoordsZ
}

#[derive(Debug, Default)]
pub struct PointM {
  pub coords: CoordsM
}

#[derive(Debug, Default)]
pub struct PointZM {
  pub coords: CoordsZM
}

impl Geometric for Point {
  fn geometry_type() -> GeometryType { GeometryType::Point }
}

impl Geometric for PointZ {
  fn geometry_type() -> GeometryType { GeometryType::Point }
}

impl Geometric for PointM {
  fn geometry_type() -> GeometryType { GeometryType::Point }
}

impl Geometric for PointZM {
  fn geometry_type() -> GeometryType { GeometryType::Point }
}
