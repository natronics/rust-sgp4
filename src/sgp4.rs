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
use coordinates;

/// $k_e = 7.43669161 \times 10\^{-2}$  Orbital constant for Earth defined as $\sqrt{GM_{\oplus}}$ where $G$ is Newton’s universal gravitational constant and $M_{\oplus}$ is the mass of the Earth. Units: $(\frac{\mathrm{Earth\ radii}}{\mathrm{minute}})\^{\frac{3}{2}}$
pub const ke: f64 = 7.43669161e-2;

/// $k_2 = 5.413080 \times 10\^{-4}$  Harmonic gravity constant for the SGP4 model. Defined as $\frac{1}{2}J_2aE\^2$.
pub const k2: f64 = 5.413080e-4;

/// ## Propagate
///
/// Propagate the orbit to the desired time.
pub fn propagate(tle: tle::TLE, time: f64) -> coordinates::TEME {

    // Copy from NORAD elements
    let n0 = tle.mean_motion;
    let i0 = tle.i;
    let e0 = tle.e;

    // Pre-compute expensive things
    let cos_i0 = i0.cos();
    let cos2_i0 = cos_i0 * cos_i0;
    let e02 = e0 * e0;

    // ************************************************************************
    // Section 1.
    // Convert from NORAD (TLE) mean elements to SGP4 elements.

    // We go through two iterations of refining aₒ (semi-major axis) and
    // nₒ (mean motion)

    //       kₑ ²/₃
    // a₁ = ----
    //       nₒ
    let a1 = (ke/n0).powf(2.0/3.0);

    //      3 k₂   (3 cos² iₒ - 1)
    // δ₁ = - --- ----------------
    //      2 a₁²   (1 - eₒ²)³/₂
    let d1 = (3.0 * k2  * ( 3.0 * cos2_i0 - 1.0)) / (2.0 * a1 * a1 * ( 1.0 - e02).powf(3.0/2.0));


    let a0 = a1 * ( 1.0 - (d1/3.0) - (d1 * d1) - (134.0 * d1 * d1 * d1 / 81.0));
    let d0 = (3.0 * k2  * ( 3.0 * cos2_i0 - 1.0)) / (2.0 * a0 * a0 * ( 1.0 - e02).powf(3.0/2.0));
    let n0_dp = n0 / (1.0 + d0);
    let a0_dp = a0 / (1.0 - d0);


    // TODO: dummy
    // Return coordinates
    coordinates::TEME {
        X: 0.0,
        Y: 0.0,
        Z: 0.0,
    }
}

#[cfg(test)]
mod tests {

    use tle::TLE;
    use coordinates::TEME;
    use super::propagate;

    #[test]
    fn spacetrack_report_3_sgp4_test_case() {
        // This testcase is from "SPACETRACK REPORT NO. 3, Models for
        // Propagation of NORAD Element Sets, Hoots & Roehrich 1980
        // pg. 81:
        let tle = TLE::load_from_str(
            "Test",
            "1 88888U          80275.98708465  .00073094  13844-3  66816-4 0     8",
            "2 88888  72.8435 115.9689 0086731  52.6988 110.5714 16.05824518   105",
        );

        // Compute
        let result0 = propagate(tle, 0.0);
        assert_eq!(result0, TEME {
            X: 0.0,
            Y: 0.0,
            Z: 0.0,
        });

    }
}
