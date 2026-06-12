mod coordinates;
mod geometry;

pub mod ellipsoid;
pub mod nested;
pub mod ring;
pub mod zuniq;

pub use crate::coordinates::Coordinate;
pub use crate::ellipsoid::Ellipsoid;
pub use crate::nested::{
    healpix_to_lonlat as healpix_to_lonlat_nested, lonlat_to_healpix as lonlat_to_healpix_nested,
    vertex as vertex_nested,
};

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
