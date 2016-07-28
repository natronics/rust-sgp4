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
    pub int_designator: &'static str,

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
    pub fn load_from_str(self, line1: String, line2: String, line3: String) -> TLE {

        let sat_number = 0;
        let classification = 'U';
        let int_designator = "";
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


mod tests {
    #[test]
    fn it_works() {
    }
}
