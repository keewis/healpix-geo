use geodesy::ellps::Ellipsoid as GeodesyEllipsoid;
use healpix_geo_core::ellipsoid::{
    Ellipsoid as RustEllipsoid, ReferenceEllipsoid, ReferenceSphere,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Ellipsoid {
    EllipsoidInverseFlattening(EllipsoidInverse),
    EllipsoidSemiMinorAxis(EllipsoidSemiMinor),
    Sphere(Sphere),
}

#[wasm_bindgen]
pub struct EllipsoidInverse {
    semi_major_axis: f64,
    inverse_flattening: f64,
}

#[wasm_bindgen]
pub struct EllipsoidSemiMinor {
    semi_major_axis: f64,
    semi_minor_axis: f64,
}

#[wasm_bindgen]
pub struct Sphere {
    radius: f64,
}

impl Ellipsoid {
    pub fn into_ellipsoid(self) -> RustEllipsoid {
        match self {
            Self::EllipsoidInverseFlattening(ell) => {
                let ellipsoid =
                    GeodesyEllipsoid::new(ell.semi_major_axis, 1.0f64 / ell.inverse_flattening);

                RustEllipsoid::Ellipsoid(ReferenceEllipsoid::new(ellipsoid))
            }
            Self::EllipsoidSemiMinorAxis(ell) => {
                let a = ell.semi_major_axis;
                let b = ell.semi_minor_axis;
                let ellipsoid = GeodesyEllipsoid::new(a, (a - b) / a);

                RustEllipsoid::Ellipsoid(ReferenceEllipsoid::new(ellipsoid))
            }
            Self::Sphere(sphere) => {
                let ellipsoid = GeodesyEllipsoid::new(sphere.radius, 0.0f64);

                RustEllipsoid::Sphere(ReferenceSphere::new(ellipsoid))
            }
        }
    }
}
