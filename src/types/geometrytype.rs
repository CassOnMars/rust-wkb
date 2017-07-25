pub enum GeometryType {
  Geometry,
  Point,
  LineString,
  Polygon,
  MultiPoint,
  MultiLineString,
  MultiPolygon,
  GeometryCollection
}

pub trait Geometric {
  fn geometry_type() -> GeometryType;
}
