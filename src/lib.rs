//! # astro-math
//!
//! `astro-math` self-contained astronomy math library designed for real-time
//! telescope control, mount synchronization, and celestial coordinate transformations.
//!
//! It includes:
//! - Accurate Julian Date and epoch handling
//! - Mean Sidereal Time (GMST, LMST) from Meeus
//! - Earth location model (`Location`) with sidereal and DMS support
//! - RA/DEC ↔ Alt/Az transformations
//! - Verified tests using Astropy and historical Julian Dates
//!
//! This library is focused on mount-grade time/space conversions and is designed for guiding,
//! pointing models, and deep sky coordination logic.
//! ## Quick Example: Compute LST and Alt/Az for Vega
//!
//! ```
//! use chrono::{Utc, TimeZone};
//! use astro_math::{julian_date, Location, ra_dec_to_alt_az};
//!
//! let dt = Utc.with_ymd_and_hms(2024, 8, 4, 6, 0, 0).unwrap();
//! let loc = Location {
//!     latitude_deg: 31.9583,
//!     longitude_deg: -111.6,
//!     altitude_m: 2120.0,
//! };
//!
//! let jd = julian_date(dt);
//! let lst = loc.local_sidereal_time(dt);
//! let (alt, az) = ra_dec_to_alt_az(279.23473479, 38.78368896, dt, &loc);
//!
//! println!("JD: {:.5}", jd);
//! println!("LST: {:.5} h", lst);
//! println!("Vega Alt: {:.3}°, Az: {:.3}°", alt, az);
//! ```
//!
//! This computes the Julian Date, sidereal time, and sky position of Vega
//! from Kitt Peak at 06:00 UTC on August 4, 2024.
//!
//! You can verify this output against Astropy using:
//!
//! ```python
//! from astropy.coordinates import SkyCoord, EarthLocation, AltAz
//! from astropy.time import Time
//! import astropy.units as u
//!
//! time = Time("2024-08-04T06:00:00", location=EarthLocation(lat=31.9583*u.deg, lon=-111.6*u.deg, height=2120*u.m))
//! coord = SkyCoord(ra=279.23473479*u.deg, dec=38.78368896*u.deg)
//! altaz = coord.transform_to(AltAz(obstime=time, location=time.location))
//! print(altaz.alt.deg, altaz.az.deg)
//! ```

pub mod airmass;
pub mod galactic;
pub mod location;
pub mod moon;
mod nutation;
pub mod parallax;
pub mod precession;
pub mod projection;
pub mod refraction;
pub mod rise_set;
pub mod sidereal;
pub mod time;
pub mod transforms;

pub use airmass::*;
pub use galactic::*;
pub use location::*;
pub use moon::*;
pub use parallax::*;
pub use precession::*;
pub use projection::*;
pub use refraction::*;
pub use rise_set::*;
pub use sidereal::*;
pub use time::*;
pub use transforms::*;

#[cfg(test)]
pub mod tests;
