use crate::Instant;
use crate::Quaternion;

use crate::TimeScale;
use std::f64::consts::PI;

///
/// Rotation from True Equator Mean Equinox (TEME) frame
/// to International Terrestrial Reference Frame (ITRF)
///
/// # Arguments
///
/// * `tm` -  Time at which to compute rotation
///
/// # Returns
///
/// * Quaternion representing rotation from TEME to ITRF
///
/// # Notes
///
/// * The TEME frame is the default frame output by the
///   SGP4 propagator
/// * This is Equation 3-90 in Vallado
///
pub fn qteme2itrf(tm: &Instant) -> Quaternion {
    qitrf2tirs(tm).conjugate() * Quaternion::rotz(gmst(tm))
}

///
/// Rotation from International Terrestrial Reference Frame (ITRF)
/// to the Terrestrial Intermediate Reference System (TIRS)
///
/// # Arguments:
///  * `tm` - Time instant at which to compute rotation
///
/// # Return:
///
///  * Quaternion representing rotation from ITRF to TIRS
///
pub fn qitrf2tirs(_tm: &Instant) -> Quaternion {
    /*
    const ASEC2RAD: f64 = PI / 180.0 / 3600.0;
    let eop = earth_orientation_params::get(tm).unwrap();
    let xp = eop[1] * ASEC2RAD;
    let yp = eop[2] * ASEC2RAD;
    let t_tt = (tm.as_mjd_with_scale(TimeScale::TT) - 51544.5) / 36525.0;
    let sp = -47.0e-6 * ASEC2RAD * t_tt;
    qrot_zcoord(-sp) * qrot_ycoord(xp) * qrot_xcoord(yp)
    */
    Quaternion::identity()
}

///
/// Greenwich Mean Sidereal Time
///
/// Vallado algorithm 15:
///
/// GMST = 67310.5481 + (876600h + 8640184.812866) * tᵤₜ₁ * (0.983104 + tᵤₜ₁ * −6.2e−6)
///
///
/// # Arguments
///
/// * `tm` - Instant object representing input time
///
/// # Returns
///
/// * `gmst` - in radians
///
pub fn gmst(tm: &Instant) -> f64 {
    let tut1: f64 = (tm.as_mjd_with_scale(TimeScale::UT1) - 51544.5) / 36525.0;
    let mut gmst: f64 = 67310.54841
        + tut1 * ((876600.0 * 3600.0 + 8640184.812866) + tut1 * (0.093104 + tut1 * -6.2e-6));

    gmst = (gmst % 86400.0) / 240.0 * PI / 180.0;
    gmst
}

/// Equation of Equinoxes
/// Equation of the equinoxes
pub fn eqeq(tm: &Instant) -> f64 {
    let d: f64 = tm.as_mjd_with_scale(TimeScale::TT) - 51544.5;
    let omega = PI / 180.0 * (125.04 - 0.052954 * d);
    let l = (280.47 + 0.98565 * d) * PI / 180.0;
    let epsilon = (23.4393 - 0.0000004 * d) * PI / 180.0;
    let d_psi = (-0.000319 * f64::sin(omega) - 0.000024 * f64::sin(2.0 * l)) * 15.0 * PI / 180.0;
    d_psi * f64::cos(epsilon)
}

/// Greenwich Apparent Sidereal Time
///
/// # Arguments
///
/// * `tm` - Instant object representing input time
///
/// # Returns
///
/// * `gast` - in radians
pub fn gast(tm: &Instant) -> f64 {
    gmst(tm) + eqeq(tm)
}
