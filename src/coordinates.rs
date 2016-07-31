/*!  # Coordinates
*/
#![deny(missing_docs,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]

#![allow(non_snake_case)]

/// ## J2000
///
/// A commonly used ECI (**E**arth **C**entered **I**nertial) frame is defined with the Earth's Mean Equator
/// and Equinox at 12:00 Terrestrial Time on 1 January 2000. The x-axis is
/// aligned with the mean equinox. The z-axis is aligned with the Earth's
/// spin axis or celestial North Pole. The y-axis is rotated by 90° East
/// about the celestial equator.
///
/// See [Earth Centered Inertial Coordinates](https://en.wikipedia.org/wiki/Earth-centered_inertial)
#[derive(Debug, PartialEq, PartialOrd)]
pub struct J2000 {

    /// $X$
    pub X: f64,

    /// $Y$
    pub Y: f64,

    /// $Z$
    pub Z: f64,
}
