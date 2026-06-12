mod coordinates;
mod geometry;

pub mod ellipsoid;
pub mod nested;
pub mod ring;
pub mod zuniq;

pub use crate::coordinates::Coordinate;
pub use crate::ellipsoid::Ellipsoid;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

pub mod nested_ {
    use crate::nested::{healpix_to_latlon, lonlat_to_healpix, vertex};
}
