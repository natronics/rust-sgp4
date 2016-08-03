/*!  # Simplified Perturbations Models (SGP4)

The _Simplified Perturbations Models_ are a set of models used for
satellites and objects relative to the Earth-centered inertial coordinate
system. These are often referred to collectively as **SGP4** because of how
popular that particular code is and how it's used with nearly all low Earth
orbit satellites.

The SGP4 and SDP4 models were published as FORTRAN IV in 1988. It has also
been ported to C. This is a port to Rust.

Original paper: [Hoots_Roehrich_1980_SPACETRACK_REPORT_NO_3.pdf](../Hoots_Roehrich_1980_SPACETRACK_REPORT_NO_3.pdf)
*/
#![deny(missing_docs,
        trivial_casts, trivial_numeric_casts,
        unsafe_code,
        unstable_features,
        unused_import_braces,
        unused_qualifications)]

// TODO: Think about names
#![allow(non_upper_case_globals)]


pub mod tle;
pub mod coordinates;

use std::io::Write;


macro_rules! println_stderr(
    ($($arg:tt)*) => { {
        let r = writeln!(&mut ::std::io::stderr(), $($arg)*);
        r.expect("failed printing to stderr");
    } }
);


/// $k_e = 7.43669161 \times 10\^{-2}$  Orbital constant for Earth defined as $\sqrt{GM_{\oplus}}$ where $G$ is Newton’s universal gravitational constant and $M_{\oplus}$ is the mass of the Earth. Units: $(\frac{\mathrm{Earth\ radii}}{\mathrm{minute}})\^{\frac{3}{2}}$
pub const ke: f64 = 7.43669161e-2;

/// $k_2 = 5.413080 \times 10\^{-4}$  Harmonic gravity constant for the SGP4 model. Defined as $\frac{1}{2}J_2aE\^2$.
pub const k2: f64 = 5.413080e-4;

/// $R_\oplus = 1.0$  Radius of the Earth (in Earth Radii).
pub const RE: f64 = 1.0;

/// $6378.135$ kilometers/Earth radii.
pub const XKMPER: f64 = 6378.135;


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

    //         ⌈     1           134    ⌉
    // aₒ = a₁ | 1 - -δ₁ - δ₁² - ---δ₁³ |
    //         ⌊     3            81    ⌋
    let a0 = a1 * ( 1.0 - (d1/3.0) - (d1 * d1) - (134.0 * d1 * d1 * d1 / 81.0));

    //      3 k₂   (3 cos² iₒ - 1)
    // δₒ = - --- ----------------
    //      2 aₒ²   (1 - eₒ²)³/₂
    let d0 = (3.0 * k2  * ( 3.0 * cos2_i0 - 1.0)) / (2.0 * a0 * a0 * ( 1.0 - e02).powf(3.0/2.0));

    //          nₒ
    // nₒ" = --------
    //       (1 + δₒ)
    let n0_dp = n0 / (1.0 + d0);

    //          aₒ
    // aₒ" = --------
    //       (1 - δₒ)
    let ao_dp = a0 / (1.0 - d0);


    // ************************************************************************
    // Section 2.
    // Determine apogee and perigee so we can deicide which SGP4 variant to
    // use later.

    // p = [aₒ"(1 - eₒ) - Rₑ] * XKMPER
    let perigee = (ao_dp * (1.0 - e0) - RE) * XKMPER;

    // p = [aₒ"(1 + eₒ) - Rₑ] * XKMPER
    let apogee = (ao_dp * (1.0 + e0) - RE) * XKMPER;


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

    use tle::load_from_str;
    use coordinates::TEME;
    use super::propagate;

    #[test]
    fn spacetrack_report_3_sgp4_test_case() {
        // This testcase is from "SPACETRACK REPORT NO. 3, Models for
        // Propagation of NORAD Element Sets, Hoots & Roehrich 1980
        // pg. 81:
        let tle = load_from_str(
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
