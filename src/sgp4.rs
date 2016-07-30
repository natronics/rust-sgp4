/*!  # The SGP4 Model
*/
#![deny(missing_docs,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]

// TODO: Think about names
#![allow(non_upper_case_globals)]

use tle;

/// $k_e = 7.43669161 \times 10\^{-2}$  Orbital constant for Earth defined as $\sqrt{GM_{\oplus}}$ where $G$ is Newton’s universal gravitational constant and $M_{\oplus}$ is the mass of the Earth. Units: $(\frac{\mathrm{Earth\ radii}}{\mathrm{minute}})\^{\frac{3}{2}}$
pub const ke: f64 = 7.43669161e-2;

/// $k_2 = 5.413080 \times 10\^{-4}$  Harmonic gravity constant for the SGP4 model. Defined as $\frac{1}{2}J_2aE\^2$.
pub const k2: f64 = 5.413080e-4;

/// ## Compute
///
/// Reading a TLE will give NORAD mean elements. The original mean motion
/// $n_0$ and semimajor axis $a_0$ are first recovered from the input elements
/// by the equations
pub fn compute(tle: tle::TLE) {

    // Copy from NORAD elements
    let n0 = tle.mean_motion;
    let i0 = tle.i;
    let e0 = tle.e;

    // Pre-compute expensive things
    let cos_i0 = i0.cos();
    let cos2_i0 = cos_i0 * cos_i0;
    let e02 = e0 * e0;

    // Convert NORAD mean elements
    let a1 = (ke/n0).powf(2.0/3.0);
    let d1 = (3.0 * k2  * ( 3.0 * cos2_i0 - 1.0)) / (2.0 * a1 * a1 * ( 1.0 - e02).powf(3.0/2.0));
    let a0 = a1 * ( 1.0 - (d1/3.0) - (d1 * d1) - (134.0 * d1 * d1 * d1 / 81.0));
    let d0 = (3.0 * k2  * ( 3.0 * cos2_i0 - 1.0)) / (2.0 * a0 * a0 * ( 1.0 - e02).powf(3.0/2.0));
    let n0_dp = n0 / (1.0 + d0);
    let a0_dp = a0 / (1.0 - d0);
}

#[cfg(test)]
mod tests {

    use tle::TLE;
    use super::compute;

    #[test]
    fn spacetrack_report_3_sgp4_test_case() {
        // This testcase is from "SPACETRACK REPORT NO. 3, Models for
        // Propagation of NORAD Element Sets, Hoots & Roehrich 1980
        // pg. 81:
        let line1 = "";
        let line2 = "1 88888U          80275.98708465  .00073094  13844-3  66816-4 0     8";
        let line3 = "2 88888  72.8435 115.9689 0086731  52.6988 110.5714 16.05824518   105";

        // Load our TLE:
        let tle = TLE::load_from_str(line1, line2, line3);

        // Compute
        let result0 = compute(tle);
    }
}
