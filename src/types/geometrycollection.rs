use types::geometrytype::*;
use types::point::Point;
use types::linestring::LineString;
use types::polygon::Polygon;
use types::multipoint::MultiPoint;
use types::multilinestring::MultiLineString;
use types::multipolygon::MultiPolygon;

#[derive(Debug, Default)]
pub struct GeometryCollection<T: Geometric> {
  pub num_geometries: u32,
  pub geometries: Vec<T>
}

impl Geometric for GeometryCollection<Point> {
  fn geometry_type() -> GeometryType { GeometryType::GeometryCollection }
}

impl Geometric for GeometryCollection<LineString> {
  fn geometry_type() -> GeometryType { GeometryType::GeometryCollection }
}

impl Geometric for GeometryCollection<Polygon> {
  fn geometry_type() -> GeometryType { GeometryType::GeometryCollection }
}

impl Geometric for GeometryCollection<MultiPoint> {
  fn geometry_type() -> GeometryType { GeometryType::GeometryCollection }
}

impl Geometric for GeometryCollection<MultiLineString> {
  fn geometry_type() -> GeometryType { GeometryType::GeometryCollection }
}

impl Geometric for GeometryCollection<MultiPolygon> {
  fn geometry_type() -> GeometryType { GeometryType::GeometryCollection }
}
