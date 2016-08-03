/*!  # Two Line Element File Utilities
*/
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

    /// Name of the object
    pub name: String,

    /// The Satellite Catalog Number, a sequential 5-digit number assigned by United States Space Command to all Earth orbiting satellites.
    pub sat_number: u32,

    /// Classification (U=Unclassified)
    pub classification: char,

    /// International Designator
    pub int_designator: String,

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


/// Read a TLE from Strings
///
/// ### Example
///
/// ```
/// extern crate sgp4;
///
/// let line1 = "ISS (ZARYA)";
/// let line2 = "1 25544U 98067A   16210.59822142  .00000812  00000-0  11901-4 0  9990";
/// let line3 = "2 25544  51.6406 211.4156 0001780  85.8307 274.3426 15.54888439 11433";
///
/// let tle = sgp4::tle::load_from_str(line1, line2, line3);
/// ```
pub fn load_from_str(line1: &str, line2: &str, line3: &str) -> TLE {

    // The first line of a TLE (optional) is the Human-readable name of the object
    let name = String::from(line1);

    // Parse Line 1:
    // Field 1, Columns: 00-01, Content: Line Number (always 1)
    // Ignore

    // Field 2, Columns: 02–06, Content: Satellite number
    let sat_number = line2[2..7].parse::<u32>().unwrap();

    // Field 3, Columns: 07–07, Content: Classification (U=Unclassified)
    let classification = line2[7..8].chars().next().unwrap();

    // Field 4, Columns: 09–10 ...
    // Field 5, Columns: 11-13 ...
    // Field 6, Columns: 14-16, Content: International Designator
    let int_designator = String::from(line2[9..17].chars().as_str());

    // Field 7, Columns: 18–19, Content: Epoch Year (last two digits of year)
    let mut epoch_year = line2[18..20].parse::<u16>().unwrap();
    if epoch_year > 56 {
        epoch_year += 1900;
    } else {
        epoch_year += 2000;
    }

    // Field 8, Columns: 20–31, Content: Epoch (day of the year and fractional portion of the day)
    let epoch_day = line2[20..32].parse::<f64>().unwrap();

    // Field 9, Columns: 33–42, Content: First Time Derivative of the Mean Motion divided by two
    let first_mean_motion = line2[33..43].replace(" .", "0.").parse::<f64>().unwrap() * 2.0;

    // Field 10, Columns: 45–52, Content: Second Time Derivative of Mean Motion divided by six (decimal point assumed)
    let mut second_mean_motion_sign = 1.0;
    if line2[44..45].chars().next().unwrap() == '-' {
        second_mean_motion_sign = -1.0;
    }
    let second_mean_motion_exp = line2[51..52].parse::<i32>().unwrap();
    let mut second_mean_motion = line2[45..50].parse::<f64>().unwrap();
    second_mean_motion /= 100000.0;
    second_mean_motion *= second_mean_motion_sign;
    second_mean_motion *= 10_f64.powi(-second_mean_motion_exp);
    second_mean_motion *= 6.0;

    // Field 11, Columns: 53–60, Content: BSTAR drag term
    let bstar_sign: f64;
    if line2[53..54].chars().next().unwrap() == '-' {
        bstar_sign = -1.0;
    } else {
        bstar_sign = 1.0;
    }
    let bstar_exp = line2[59..61].parse::<i32>().unwrap();
    let mut bstar = line2[54..59].parse::<f64>().unwrap();
    bstar /= 100000.0;
    bstar *= bstar_sign;
    bstar *= 10_f64.powi(bstar_exp);

    // Field 12, Columns: 62–63, Content: The number 0 (originally this should have been "Ephemeris type")
    // Ignored

    // Field 13, Columns: 64–67, Content: Element set number. Incremented when a new TLE is generated for this object
    let tle_version: u16;
    match line2[64..68].parse::<u16>() {
        Ok(n) => tle_version = n,
        Err(_) => tle_version = 0,
    }

    // Field 14, Columns: 68–68, Content: Checksum (modulo 10)
    // TODO: Checksum


    // Parse Line 2:
    // Field 1, Columns: 00-01, Content: Line Number (always 2)
    // Ignore

    // Field 2, Columns: 02–06, Content: Satellite number
    // Ignore (redundant)

    // Field 3, Columns: 08–15, Content: Inclination (degrees)
    let i = line3[8..16].trim().parse::<f64>().unwrap();

    // Field 4, Columns: 17–24, Content: Right ascension of the ascending node (degrees)
    let raan = line3[17..25].trim().parse::<f64>().unwrap();

    // Field 5, Columns: 26–32, Content: Eccentricity
    let e = ("0.".to_string() + &line3[26..33]).parse::<f64>().unwrap();

    // Field 6, Columns: 34–41, Content: Argument of perigee (degrees)
    let omega = line3[34..42].trim().parse::<f64>().unwrap();

    // Field 7, Columns: 43–50, Content: Mean Anomaly (degrees)
    let mean_anomaly = line3[43..51].trim().parse::<f64>().unwrap();

    // Field 8, Columns: 52–62, Content: Mean Motion (revolutions per day)
    let mean_motion = line3[52..63].trim().parse::<f64>().unwrap();

    // Field 9, Columns: 63–67, Content: Revolution number at epoch (revolutions)
    let revolution_number = line3[63..68].trim().parse::<u32>().unwrap();

    // Field 10, Columns: 69–69, Content: Checksum (modulo 10)
    // TODO: Checksum

    TLE {
        name: name,
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


#[cfg(test)]
mod tests {

    use super::load_from_str;

    #[test]
    fn spacetrack_report_3_sgp4_test_case() {
        // This testcase is from "SPACETRACK REPORT NO. 3, Models for
        // Propagation of NORAD Element Sets, Hoots & Roehrich 1980
        // pg. 81:
        let line1 = "";
        let line2 = "1 88888U          80275.98708465  .00073094  13844-3  66816-4 0     8";
        let line3 = "2 88888  72.8435 115.9689 0086731  52.6988 110.5714 16.05824518   105";

        // Load our TLE:
        let tle = load_from_str(line1, line2, line3);

        // We should get these values:
        assert_eq!(tle.name, "");
        assert_eq!(tle.sat_number, 88888);
        assert_eq!(tle.classification, 'U');
        assert_eq!(tle.int_designator, "        ");
        assert_eq!(tle.epoch_year, 1980);
        assert_eq!(tle.epoch_day, 275.98708465);
        assert_eq!(tle.first_mean_motion, 0.00073094*2.0);
        assert_eq!(tle.second_mean_motion, 0.13844e-3*6.0);
        assert_eq!(tle.bstar, 0.66816e-4);
        assert_eq!(tle.tle_version, 0);
        assert_eq!(tle.i, 72.8435);
        assert_eq!(tle.raan, 115.9689);
        assert_eq!(tle.e, 0.0086731);
        assert_eq!(tle.omega, 52.6988);
        assert_eq!(tle.mean_anomaly, 110.5714);
        assert_eq!(tle.mean_motion, 16.05824518);
        assert_eq!(tle.revolution_number, 10);
    }
}
