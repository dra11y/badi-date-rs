#[derive(Clone, Copy, Debug, PartialEq)]
// WGS-84 GPS coordinates used to calculate sunset times
pub struct Coordinates {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug)]
pub enum CoordinatesError {
    LatitudeInvalid,
    LongitudeInvalid,
}

impl CoordinatesError {
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
