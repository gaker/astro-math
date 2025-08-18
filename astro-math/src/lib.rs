//! # Astro Math
//!
//! Astronomy library for Rust implementing algorithms from Jean Meeus and ERFA.
//! Built for telescope control, observation planning, and celestial mechanics.
//!
//! ## Core Capabilities
//!
//! This library provides everything needed for astronomical calculations:
//!
//! ### Time Systems
//! - [`time`] — Julian Date conversions, J2000 epoch calculations
//! - [`sidereal`] — Greenwich Mean Sidereal Time (GMST), Local Mean/Apparent Sidereal Time
//!
//! ### Observer Location  
//! - [`location`] — Earth coordinates with flexible parsing (27+ formats)
//! - Support for decimal degrees, DMS, HMS, aviation formats, Unicode symbols
//!
//! ### Coordinate Transformations
//! - [`transforms`] — RA/Dec ↔ Alt/Az conversions with spherical trigonometry
//! - [`galactic`] — Equatorial ↔ Galactic coordinate system conversions
//! - [`projection`] — Gnomonic/TAN projection for astrometry and plate solving
//!
//! ### Precision Corrections 
//! - [`precession`] — Convert coordinates between epochs (J2000 ↔ current date)
//! - [`nutation`] — Earth's axis wobble corrections (±18.6" longitude, ±9.2" obliquity)
//! - [`aberration`] — Annual stellar aberration corrections (±20.5 arcseconds)
//! - [`proper_motion`] — Linear and rigorous 3D space motion calculations
//! - [`parallax`] — Diurnal and annual parallax corrections
//!
//! ### Solar System Objects
//! - [`moon`] — Lunar position, phase, illumination, distance calculations
//! - [`sun`] — Solar position and rise/set calculations
//! - [`rise_set`] — Rise, set, and meridian transit times for any object
//!
//! ### Atmospheric Effects
//! - [`refraction`] — Multiple atmospheric refraction models (Bennett, Saemundsson, radio)
//! - [`airmass`] — Various airmass formulas for extinction calculations
//!
//! ### High Performance
//! - Parallel batch processing with Rayon for coordinate transformations
//! - ERFA (Essential Routines for Fundamental Astronomy) integration
//! - Input validation and clear error messages
//!
//! ## Accuracy & Standards
//!
//! This library implements algorithms from:
//! - **Jean Meeus**: *Astronomical Algorithms* (2nd edition)
//! - **IAU SOFA**: Standards of Fundamental Astronomy
//! - **ERFA**: Essential Routines for Fundamental Astronomy  
//! - **USNO**: US Naval Observatory references
//!
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
//! let (alt, az) = ra_dec_to_alt_az(279.23473479, 38.78368896, dt, &loc).unwrap();
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

pub mod aberration;
pub mod airmass;
pub mod erfa;
pub mod error;
pub mod galactic;
pub mod location;
pub mod moon;
pub mod nutation;
pub mod parallax;
pub mod precession;
pub mod projection;
pub mod proper_motion;
pub mod refraction;
pub mod rise_set;
pub mod sidereal;
pub mod sun;
pub mod time;
pub mod transforms;

pub use aberration::*;
pub use airmass::*;
pub use error::{AstroError, Result};
pub use galactic::*;
pub use location::*;
pub use moon::*;
pub use parallax::*;
pub use precession::*;
pub use projection::*;
pub use proper_motion::*;
pub use refraction::*;
pub use rise_set::*;
pub use sidereal::*;
pub use time::*;
pub use transforms::*;

#[cfg(test)]
pub mod tests;
