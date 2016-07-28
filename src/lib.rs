//! # Simplified Perturbations Models (SGP4)
//!
//! The _Simplified Perturbations Models_ are a set of models used for
//! satellites and objects relative to the Earth-centered inertial coordinate
//! system. These are often referred to collectively as **SGP4** because of how
//! popular that particular code is and how it's used with nearly all low Earth
//! orbit satellites.
//!
//! The SGP4 and SDP4 models were published as FORTRAN IV in 1988. It has also
//! been ported to C. This is a port to Rust.

#![deny(missing_docs,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]

/// ## Satellite elements from a TLE file
///
/// Read "Two Line Element" files that are a standard way of distributing
/// defined orbits.
pub struct TLE {
    /// The Satellite Catalog Number, a sequential 5-digit number assigned by United States Space Command to all Earth orbiting satellites.
    pub sat_number: u32,

    /// Classification (U=Unclassified)
    pub classification: char,

    /// International Designator
    pub int_designator: [char;8],

    /// Epoch Year
    pub epoch_year: u16,

    /// Epoch Day
    pub epoch_day: f64,

    /// First Time Derivative of the Mean Motion
    pub first_mean_motion: f64,

    /// Second Time Derivative of Mean Motion
    pub second_mean_motion: f64,

    /// BSTAR drag term
    pub bstar: f64,

    /// Element set number (Incremented when a new TLE is generated for this object)
    pub tle_version: u16,

    /// Inclination
    pub i: f64,

    /// Right ascension of the ascending node
    pub raan: f64,

    /// Eccentricity
    pub e: f64,

    /// Argument of perigee
    pub omega: f64,

    /// Mean Anomaly
    pub mean_anomaly: f64,

    /// Mean Motion (revolutions per day)
    pub mean_motion: f64,

    /// Revolution number at epoch (revolutions)
    pub revolution_number: u32,
}


impl TLE {
    /// Read a TLE from Strings
    ///
    /// ### Example
    ///
    ///
    /// ```
    /// extern crate sgp4;
    ///
    /// let line1 = "ISS (ZARYA)";
    /// let line2 = "1 25544U 98067A   16210.59822142  .00000812  00000-0  11901-4 0  9990";
    /// let line3 = "2 25544  51.6406 211.4156 0001780  85.8307 274.3426 15.54888439 11433";
    ///
    /// let tle = sgp4::TLE::load_from_str(line1, line2, line3);
    /// ```
    ///
    pub fn load_from_str(line1: &str, line2: &str, line3: &str) -> TLE {

        let sat_number = 0;
        let classification = 'U';
        let int_designator = ['9', '8', '0', '6', '7', 'A', ' ', ' '];
        let epoch_year = 0;
        let epoch_day = 0.0;
        let first_mean_motion = 0.0;
        let second_mean_motion = 0.0;
        let bstar = 0.0;
        let tle_version = 0;
        let i = 0.0;
        let raan = 0.0;
        let e = 0.0;
        let omega = 0.0;
        let mean_anomaly = 0.0;
        let mean_motion = 0.0;
        let revolution_number = 0;

        TLE {
            sat_number: sat_number,
            classification: classification,
            int_designator: int_designator,
            epoch_year: epoch_year,
            epoch_day: epoch_day,
            first_mean_motion: first_mean_motion,
            second_mean_motion: second_mean_motion,
            bstar: bstar,
            tle_version: tle_version,
            i: i,
            raan: raan,
            e: e,
            omega: omega,
            mean_anomaly: mean_anomaly,
            mean_motion: mean_motion,
            revolution_number: revolution_number,
        }
    }
}


#[cfg(test)]
mod tests {

    use super::TLE;

    #[test]
    fn spacetrack_report_3_sgp4_test_case() {
        let line1 = "";
        let line2 = "1 88888U          80275.98708465  .00073094  13844-3  66816-4 0     8";
        let line3 = "2 88888  72.8435 115.9689 0086731  52.6988 110.5714 16.05824518   105";

        let tle = TLE::load_from_str(line1, line2, line3);
    }
}
