mod coords;
mod point;
mod geometrytype;
mod linestring;
mod linearring;
mod polygon;
mod multipoint;
mod multilinestring;
mod multipolygon;
mod geometrycollection;

pub enum ByteOrder {
  XDR,
  NDR
}
