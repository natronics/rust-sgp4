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

pub mod tle;
pub mod sgp4;
pub mod coordinates;
