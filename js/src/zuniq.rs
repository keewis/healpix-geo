use cdshealpix as healpix;
use healpix_geo_core::ellipsoid::ReferenceBody;
use healpix_geo_core::scalar::zuniq::coordinates as scalar;
use wasm_bindgen::prelude::*;

use crate::coordinates::Coordinate;
use crate::ellipsoid::Ellipsoid;

/// Center coordinates for the given cell
#[wasm_bindgen(js_name = healpixToLonLatZuniq)]
pub fn healpix_to_lonlat(hash: u64, ellipsoid: Option<Ellipsoid>) -> Coordinate {
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    let (lon, lat) = scalar::healpix_to_lonlat(&hash, &ellipsoid_);

    Coordinate { lon, lat }
}

/// Project the given coordinate to the healpix grid
#[wasm_bindgen(js_name = lonLatToHealpixZuniq)]
pub fn lonlat_to_healpix(lon: f64, lat: f64, depth: u8, ellipsoid: Option<Ellipsoid>) -> u64 {
    let layer = healpix::nested::get(depth);
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    scalar::lonlat_to_healpix(&lon, &lat, layer, &ellipsoid_)
}

/// Single vertex of the given cell
///
/// The parameters `u` and `v` represent offsets from the southern vertex of the given cell.
#[wasm_bindgen(js_name = vertexZuniq)]
pub fn vertex(hash: u64, u: f64, v: f64, ellipsoid: Option<Ellipsoid>) -> Coordinate {
    let (depth, nested) = healpix::nested::from_zuniq(hash);
    let layer = healpix::nested::get(depth);
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    let (lon, lat) = layer.sph_coo(nested, u, v);

    Coordinate {
        lon: lon.to_degrees().rem_euclid(360.0),
        lat: ellipsoid_.latitude_authalic_to_geographic(lat).to_degrees(),
    }
}
