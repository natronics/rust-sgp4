/*!  # Coordinates
*/
#![deny(missing_docs,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]

#![allow(non_snake_case)]

/// ## TEME
///
/// **T**rue **E**quator, **M**ean **E**quinox coordinate.
#[derive(Debug, PartialEq, PartialOrd)]
pub struct TEME {

    /// $X$
    pub X: f64,

    /// $Y$
    pub Y: f64,

    /// $Z$
    pub Z: f64,
}
