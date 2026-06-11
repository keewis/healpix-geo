use cdshealpix as healpix;
use healpix_geo_core::scalar::nested::coordinates as scalar;
use wasm_bindgen::prelude::*;

use crate::coordinates::Coordinate;
use crate::ellipsoid::Ellipsoid;

#[wasm_bindgen]
pub fn healpix_to_lonlat(ipix: u64, depth: u8, ellipsoid: Option<Ellipsoid>) -> Coordinate {
    let layer = healpix::nested::get(depth);

    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    let (lon, lat) = scalar::healpix_to_lonlat(&ipix, layer, &ellipsoid_);

    Coordinate { lon, lat }
}

#[wasm_bindgen]
pub fn lonlat_to_healpix(lon: f64, lat: f64, depth: u8, ellipsoid: Option<Ellipsoid>) -> u64 {
    let layer = healpix::nested::get(depth);
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    scalar::lonlat_to_healpix(&lon, &lat, layer, &ellipsoid_)
}
