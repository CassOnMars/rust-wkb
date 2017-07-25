use types::coords::*;

#[derive(Debug, Default)]
pub struct LinearRing {
  pub num_coords: u32,
  pub coords: Vec<Coords>
}

#[derive(Debug, Default)]
pub struct LinearRingZ {
  pub num_coords: u32,
  pub coords: Vec<CoordsZ>
}

#[derive(Debug, Default)]
pub struct LinearRingM {
  pub num_coords: u32,
  pub coords: Vec<CoordsM>
}

#[derive(Debug, Default)]
pub struct LinearRingZM {
  pub num_coords: u32,
  pub coords: Vec<CoordsZM>
}

