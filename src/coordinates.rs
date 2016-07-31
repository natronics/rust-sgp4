/*!  # Coordinates
*/
#![deny(missing_docs,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]

#![allow(non_snake_case)]

/// ## J2000 ECI
#[derive(Debug, PartialEq, PartialOrd)]
pub struct J2000 {

    /// X
    pub X: f64,

    /// Y
    pub Y: f64,

    /// Z
    pub Z: f64,
}
