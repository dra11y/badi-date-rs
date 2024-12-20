use std::fmt;

use serde::{Deserialize, Serialize};

/// WGS-84 GPS coordinates used to calculate sunset times for a [`LocalBadiDate`][`crate::LocalBadiDate`]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Coordinates {
    /// The latitude [-90...90.] in the WGS-84 coordinate system
    pub latitude: f64,
    /// The longitude [-180...180.] in the WGS-84 coordinate system
    pub longitude: f64,
}

/// Error returned for invalid [`Coordinates`]
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum CoordinatesError {
    /// The latitude is not within -90...90. inclusive
    LatitudeInvalid,
    /// The longitude is not within -180...180. inclusive
    LongitudeInvalid,
}

impl fmt::Display for Coordinates {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:0>11.6},{:0>11.6}", self.longitude, self.latitude)
    }
}

impl CoordinatesError {
    /// Message to display for the [`CoordinatesError`]
    pub fn message(&self) -> String {
        match self {
            CoordinatesError::LatitudeInvalid => {
                "Latitude must be between -90.0 and 90.0 (inclusive)".to_string()
            }
            CoordinatesError::LongitudeInvalid => {
                "Longitude must be between -180.0 and 180.0 (inclusive)".to_string()
            }
        }
    }
}

impl Coordinates {
    /// Create a new set of WGS-84 GPS coordinates used in calculating sunset times
    /// (ensure these match the time zone being used!)
    pub fn new(latitude: f64, longitude: f64) -> Result<Self, CoordinatesError> {
        Ok(Self {
            latitude,
            longitude,
        })
    }
}
