#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;
use std::error::Error;

/// Commonly used rating primitives.
pub mod prelude {
    pub use crate::{
        CurrentRating, FrequencyRating, PowerRating, RatingError, TemperatureRating, Tolerance,
        VoltageRating,
    };
}

/// Errors returned while constructing rating values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RatingError {
    /// The rating value was not finite.
    NonFinite,
    /// The rating value was negative where only non-negative values make sense.
    Negative,
}

impl fmt::Display for RatingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("rating value must be finite"),
            Self::Negative => formatter.write_str("rating value cannot be negative"),
        }
    }
}

impl Error for RatingError {}

/// A voltage rating in volts.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct VoltageRating {
    volts: f64,
}

impl VoltageRating {
    /// Creates a non-negative voltage rating in volts.
    ///
    /// # Errors
    ///
    /// Returns [`RatingError`] when the value is not finite or is negative.
    pub fn new_volts(value: f64) -> Result<Self, RatingError> {
        non_negative_finite(value).map(|volts| Self { volts })
    }

    /// Returns the rating in volts.
    #[must_use]
    pub const fn volts(self) -> f64 {
        self.volts
    }
}

impl fmt::Display for VoltageRating {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} V", self.volts)
    }
}

/// A current rating in amperes.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CurrentRating {
    amperes: f64,
}

impl CurrentRating {
    /// Creates a non-negative current rating in amperes.
    ///
    /// # Errors
    ///
    /// Returns [`RatingError`] when the value is not finite or is negative.
    pub fn new_amperes(value: f64) -> Result<Self, RatingError> {
        non_negative_finite(value).map(|amperes| Self { amperes })
    }

    /// Returns the rating in amperes.
    #[must_use]
    pub const fn amperes(self) -> f64 {
        self.amperes
    }
}

impl fmt::Display for CurrentRating {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} A", self.amperes)
    }
}

/// A power rating in watts.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PowerRating {
    watts: f64,
}

impl PowerRating {
    /// Creates a non-negative power rating in watts.
    ///
    /// # Errors
    ///
    /// Returns [`RatingError`] when the value is not finite or is negative.
    pub fn new_watts(value: f64) -> Result<Self, RatingError> {
        non_negative_finite(value).map(|watts| Self { watts })
    }

    /// Returns the rating in watts.
    #[must_use]
    pub const fn watts(self) -> f64 {
        self.watts
    }
}

impl fmt::Display for PowerRating {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} W", self.watts)
    }
}

/// A tolerance value in percent.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Tolerance {
    percent: f64,
}

impl Tolerance {
    /// Creates a non-negative tolerance percentage.
    ///
    /// # Errors
    ///
    /// Returns [`RatingError`] when the value is not finite or is negative.
    pub fn from_percent(value: f64) -> Result<Self, RatingError> {
        non_negative_finite(value).map(|percent| Self { percent })
    }

    /// Returns the tolerance in percent.
    #[must_use]
    pub const fn percent(self) -> f64 {
        self.percent
    }
}

impl fmt::Display for Tolerance {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "+/-{}%", self.percent)
    }
}

/// A temperature rating in degrees Celsius.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct TemperatureRating {
    celsius: f64,
}

impl TemperatureRating {
    /// Creates a temperature rating in degrees Celsius.
    ///
    /// # Errors
    ///
    /// Returns [`RatingError::NonFinite`] when the value is not finite.
    pub fn new_celsius(value: f64) -> Result<Self, RatingError> {
        finite(value).map(|celsius| Self { celsius })
    }

    /// Returns the rating in degrees Celsius.
    #[must_use]
    pub const fn celsius(self) -> f64 {
        self.celsius
    }
}

impl fmt::Display for TemperatureRating {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} C", self.celsius)
    }
}

/// A frequency rating in hertz.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct FrequencyRating {
    hertz: f64,
}

impl FrequencyRating {
    /// Creates a non-negative frequency rating in hertz.
    ///
    /// # Errors
    ///
    /// Returns [`RatingError`] when the value is not finite or is negative.
    pub fn new_hertz(value: f64) -> Result<Self, RatingError> {
        non_negative_finite(value).map(|hertz| Self { hertz })
    }

    /// Returns the rating in hertz.
    #[must_use]
    pub const fn hertz(self) -> f64 {
        self.hertz
    }
}

impl fmt::Display for FrequencyRating {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} Hz", self.hertz)
    }
}

fn finite(value: f64) -> Result<f64, RatingError> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(RatingError::NonFinite)
    }
}

fn non_negative_finite(value: f64) -> Result<f64, RatingError> {
    let value = finite(value)?;
    if value < 0.0 {
        Err(RatingError::Negative)
    } else {
        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{CurrentRating, PowerRating, RatingError, Tolerance, VoltageRating};

    #[test]
    fn constructs_voltage_ratings() -> Result<(), RatingError> {
        let rating = VoltageRating::new_volts(16.0)?;

        assert!((rating.volts() - 16.0).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn constructs_current_ratings() -> Result<(), RatingError> {
        let rating = CurrentRating::new_amperes(0.5)?;

        assert!((rating.amperes() - 0.5).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn constructs_power_ratings() -> Result<(), RatingError> {
        let rating = PowerRating::new_watts(0.25)?;

        assert!((rating.watts() - 0.25).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn constructs_tolerance_percentages() -> Result<(), RatingError> {
        let tolerance = Tolerance::from_percent(1.0)?;

        assert!((tolerance.percent() - 1.0).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn rejects_negative_tolerance() {
        assert_eq!(Tolerance::from_percent(-1.0), Err(RatingError::Negative));
    }

    #[test]
    fn displays_rating_values() -> Result<(), RatingError> {
        assert_eq!(VoltageRating::new_volts(5.0)?.to_string(), "5 V");
        assert_eq!(CurrentRating::new_amperes(2.0)?.to_string(), "2 A");
        assert_eq!(PowerRating::new_watts(0.25)?.to_string(), "0.25 W");
        assert_eq!(Tolerance::from_percent(10.0)?.to_string(), "+/-10%");
        Ok(())
    }
}
