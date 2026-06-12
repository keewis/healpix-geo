use cdshealpix as healpix;
use healpix_geo_core::ellipsoid::ReferenceBody;
use healpix_geo_core::scalar::nested::coordinates as scalar;
use wasm_bindgen::prelude::*;

use crate::coordinates::Coordinate;
use crate::ellipsoid::Ellipsoid;

/// Center coordinates for the given cell
#[wasm_bindgen(js_namespace = "nested")]
pub fn healpix_to_lonlat(ipix: u64, depth: u8, ellipsoid: Option<Ellipsoid>) -> Coordinate {
    let layer = healpix::nested::get(depth);

    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    let (lon, lat) = scalar::healpix_to_lonlat(&ipix, layer, &ellipsoid_);

    Coordinate { lon, lat }
}

/// Project the given coordinate to the healpix grid
#[wasm_bindgen(js_namespace = "nested")]
pub fn lonlat_to_healpix(lon: f64, lat: f64, depth: u8, ellipsoid: Option<Ellipsoid>) -> u64 {
    let layer = healpix::nested::get(depth);
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    scalar::lonlat_to_healpix(&lon, &lat, layer, &ellipsoid_)
}

/// Single vertex of the given cell
///
/// The parameters `u` and `v` represent offsets from the southern vertex of the given cell.
#[wasm_bindgen(js_namespace = "nested")]
pub fn vertex(hash: u64, depth: u8, u: f64, v: f64, ellipsoid: Option<Ellipsoid>) -> Coordinate {
    let layer = healpix::nested::get(depth);
    let ellipsoid_ = ellipsoid.map(|e| e.into_ellipsoid()).unwrap_or_default();

    let (lon, lat) = layer.sph_coo(hash, u, v);

    Coordinate {
        lon: lon.to_degrees().rem_euclid(360.0),
        lat: ellipsoid_.latitude_authalic_to_geographic(lat).to_degrees(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex() {
        let hash: u64 = 0;
        let depth: u8 = 0;

        let uv: Vec<(f64, f64)> = vec![
            (0.0, 0.0),
            (0.5, 0.0),
            (1.0, 0.0),
            (0.0, 0.5),
            (0.0, 1.0),
            (1.0, 1.0),
        ];

        let values = uv
            .into_iter()
            .map(|(u, v)| vertex(hash, depth, u, v, None))
            .collect::<Vec<_>>();
        let expected = (0..6)
            .into_iter()
            .map(|v| Coordinate {
                lon: v as f64 + 1.0,
                lat: v as f64 + 2.0,
            })
            .collect::<Vec<_>>();

        assert_eq!(values, expected);
    }
}
