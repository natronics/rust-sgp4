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
#![deny(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications,
)]

// TODO: Think about names
#![allow(
    non_upper_case_globals,
    non_snake_case,
)]


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

/// S (?)
pub const S: f64 = 1.01222928;

/// qs4 (?)
pub const QS4: f64 = 1.88027916e-9;

/// $J_3 = -2.53881 \times 10\^{-4}$: the third gravitational zonal harmonic of the Earth
pub const J3: f64 = -2.53881e-4;

/// $A_{3,0} = -J_3a_E\^3$
pub const A30: f64 = -J3 * RE * RE * RE;

/// ## Propagate
///
/// Propagate the orbit to the desired time.
pub fn propagate(tle: tle::TLE, time: f64) -> coordinates::TEME {

    // Copy from NORAD elements
    let n0 = tle.mean_motion;
    let i0 = tle.i;
    let e0 = tle.e;
    let wo = tle.omega;
    let Bstar = tle.bstar;

    // Pre-compute expensive things
    let cos_i0 = i0.cos();
    let sin_io = i0.sin();
    let cos2_i0 = cos_i0.powi(2);
    let e02 = e0.powi(2);


    // ************************************************************************
    // Section 1.
    // Convert from NORAD (TLE) mean elements to SGP4 elements.

    // We go through two iterations of refining aₒ (semi-major axis) and
    // nₒ (mean motion)

    //       kₑ  ⅔
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


    // ************************************************************************
    // Section 3.
    // Calculate more constants

    // Set parameter "s" depending on perigee of the satellite:
    let s: f64;
    let qs4: f64;

    // Above 156 km we use normal SGP
    if perigee > 156.0 {
        // Use original value of s
        s = S;
        qs4 = QS4;
    }

    // Between 156 and 98 km use this modification:
    else if perigee > 98.0 {
        // s = aₒ"(1 − eₒ) − s + aE
        s = ao_dp * (1.0 - e0) - S + RE;
        qs4 = (QS4.powf(1.0/4.0) + S - s).powi(4);
    }
    else {
        s = (20.0 / XKMPER) + RE;
        qs4 = (QS4.powf(1.0/4.0) + S - s).powi(4);
    }

    // θ = cos iₒ
    let O = cos_i0;
    let O2 = O * O;

    //        1
    // ξ = -------
    //     aₒ" - s
    let xi = 1.0 / (ao_dp - s);
    let xi2 = xi.powi(2);
    let xi3 = xi.powi(3);
    let xi4 = xi.powi(4);
    let xi5 = xi.powi(5);

    //               ½
    // βₒ = (1 − eₒ²)
    let Bo = (1.0 - e02).sqrt();
    let Bo2 = Bo.powi(2);

    // η = aₒ"eₒξ
    let n = ao_dp * e0 * xi;
    let n2 = n.powi(2);
    let n3 = n.powi(3);
    let n4 = n.powi(4);

    //                           -⁷/₂⌈   ⌈    3                ⌉   3   k₂ξ    ⌈ 1   3  ⌉                ⌉
    // C₂ = (qₒ − s)⁴ξ⁴nₒ"(1 - η²)   |aₒ"|1 + -η² + 4eₒη + eₒη³| + - -------- |-- + -θ²|(8 + 24η² + 3η⁴)|
    //                               ⌊   ⌊    2                ⌋   2 (1 - η²) ⌊ 2   2  ⌋                ⌋
    let C2 = qs4 * xi4 * n0_dp * (1.0 - n2).powf(-7.0/2.0) * (ao_dp * (1.0 + (1.5 * n2) + (4.0 * e0 * n) + (e0 * n3)) + 1.5 * (k2 * xi)/(1.0 - n2) * (-0.5 + (1.5 * O2)) * (8.0 + (24.0 * n2) + (3.0 * n4)));

    // C₁ = B*C₂
    let C1 = Bstar * C2;

    //      (qₒ − s)⁴ξ⁵A₃₀ nₒ" aE sin iₒ
    // C₃ = -----------------------------
    //                 k₂eₒ
    let C3 = (qs4 * xi5 * A30 * n0_dp * RE * sin_io) / (k2 * e0);

    //                                  -⁷/₂⌈⌈              1     1  ⌉      2k₂ξ       ⌈          ⌈    3            1    ⌉   3                                ⌉⌉
    // C₄ = 2nₒ"(qₒ − s)⁴ξ⁴aₒ"βₒ²(1 - η²)   ||2η(1 + eₒη) + -eₒ + -η³| - ----------- × |3(1 - 3θ²)|1 + -η² - 2eₒη - -eₒη³| + -(1 - θ²)(2η² - eₒη - eₒη³)cos2ωₒ||
    //                                      ⌊⌊              2     2  ⌋   aₒ"(1 - η²)   ⌊          ⌊    2            2    ⌋   4                                ⌋⌋
    // This one is really long, so let's break it into some pieces:
    //     |            C4_1               | |         C4_2          |  |    C4_3    | |             C4_4                |  |               C4_5              |
    let C4_1 = 2.0 * n0_dp * qs4 * xi4 * ao_dp * Bo2 * (1.0 - n2).powf(-7.0/2.0);
    let C4_2 = 2.0 * n * (1.0 + e0 * n) + (0.5 * e0) + (0.5 * n3);
    let C4_3 = (2.0 * k2 * xi) / (ao_dp * (1.0 - n2));
    let C4_4 = 3.0 * (1.0 - 3.0 * O2) * (1.0 + (1.5 * n2) - (2.0 * e0 * n) - (0.5 * e0 * n3));
    let C4_5 = 0.75 * (1.0 - O2) * ((2.0 * n2) - (e0 * n) - (e0 * n3)) * (2.0 * wo).cos();
    let C4 = C4_1 * (C4_2 - (C4_3 * (C4_4 + C4_5)));

    //                               -⁷/₂⌈    11                ⌉
    // C₅ = 2(qₒ − s)⁴ξ⁴aₒ"βₒ²(1 - η²)   |1 + --η(η + eₒ) + eₒη³|
    //                                   ⌊     4                ⌋
    let C5 = 2.0 * qs4 * xi4 * ao_dp * Bo2 * (1.0 - n2).powf(-7.0/2.0) * (1.0 + (2.75 * n * (n + e0)) + (e0 * n3));

    // D₂ = 4aₒ"ξC₁²
    let D2 = 4.0 * ao_dp * xi * C1.powi(2);

    //      4
    // D₃ = -aₒ"ξ²(17aₒ" + s)C₁³
    //      3
    let D3 = (4.0/3.0) * ao_dp * xi2 * (17.0 * ao_dp + s) * C1.powi(3);

    //      2
    // D₄ = -aₒ"ξ³(221aₒ" + 31s)C₁⁴
    //      3
    let D4 = (2.0/3.0) * ao_dp * xi3 * (221.0 * ao_dp + (31.0 * s)) * C1.powi(4);


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
