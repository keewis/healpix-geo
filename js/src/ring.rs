use cdshealpix as healpix;
use healpix_geo_core::ellipsoid::ReferenceBody;
use healpix_geo_core::scalar::nested::coordinates as scalar;
use wasm_bindgen::prelude::*;

use crate::coordinates::Coordinate;
use crate::ellipsoid::Ellipsoid;

/// Center coordinates for the given cell
#[wasm_bindgen]
pub fn healpix_to_lonlat_ring(hash: u64, depth: u8, ellipsoid: Option<Ellipsoid>) -> Coordinate {
    let layer = healpix::nested::get(depth);
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();
    let hash_ = layer.from_ring(hash);

    let (lon, lat) = scalar::healpix_to_lonlat(&hash_, layer, &ellipsoid_);

    Coordinate { lon, lat }
}

/// Project the given coordinate to the healpix grid
#[wasm_bindgen]
pub fn lonlat_to_healpix_ring(lon: f64, lat: f64, depth: u8, ellipsoid: Option<Ellipsoid>) -> u64 {
    let layer = healpix::nested::get(depth);
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    layer.to_ring(scalar::lonlat_to_healpix(&lon, &lat, layer, &ellipsoid_))
}

/// Single vertex of the given cell
///
/// The parameters `u` and `v` represent offsets from the southern vertex of the given cell.
#[wasm_bindgen]
pub fn vertex_ring(
    hash: u64,
    depth: u8,
    u: f64,
    v: f64,
    ellipsoid: Option<Ellipsoid>,
) -> Coordinate {
    let layer = healpix::nested::get(depth);
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    let (lon, lat) = layer.sph_coo(layer.from_ring(hash), u, v);

    Coordinate {
        lon: lon.to_degrees().rem_euclid(360.0),
        lat: ellipsoid_.latitude_authalic_to_geographic(lat).to_degrees(),
    }
}
