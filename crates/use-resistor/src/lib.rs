#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_rating::{PowerRating, Tolerance};

/// Commonly used resistor primitives.
pub mod prelude {
    pub use crate::{
        ResistanceValue, ResistanceValueError, ResistorKind, ResistorKindParseError, ResistorSpec,
    };
}

/// A resistance value in ohms.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct ResistanceValue {
    ohms: f64,
}

impl ResistanceValue {
    /// Creates a non-negative resistance value in ohms.
    ///
    /// # Errors
    ///
    /// Returns [`ResistanceValueError`] when the value is not finite or is negative.
    pub fn new_ohms(value: f64) -> Result<Self, ResistanceValueError> {
        if !value.is_finite() {
            return Err(ResistanceValueError::NonFinite);
        }

        if value < 0.0 {
            return Err(ResistanceValueError::Negative);
        }

        Ok(Self { ohms: value })
    }

    /// Returns the value in ohms.
    #[must_use]
    pub const fn ohms(self) -> f64 {
        self.ohms
    }
}

impl fmt::Display for ResistanceValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ohm", self.ohms)
    }
}

/// Errors returned while constructing resistance values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResistanceValueError {
    /// The resistance was not finite.
    NonFinite,
    /// The resistance was negative.
    Negative,
}

impl fmt::Display for ResistanceValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("resistance must be finite"),
            Self::Negative => formatter.write_str("resistance cannot be negative"),
        }
    }
}

impl Error for ResistanceValueError {}

/// Descriptive resistor kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ResistorKind {
    Fixed,
    Variable,
    Potentiometer,
    Thermistor,
    Photoresistor,
    Shunt,
    PullUp,
    PullDown,
    Unknown,
    Custom(String),
}

impl fmt::Display for ResistorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Fixed => "fixed",
            Self::Variable => "variable",
            Self::Potentiometer => "potentiometer",
            Self::Thermistor => "thermistor",
            Self::Photoresistor => "photoresistor",
            Self::Shunt => "shunt",
            Self::PullUp => "pull-up",
            Self::PullDown => "pull-down",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for ResistorKind {
    type Err = ResistorKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ResistorKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "fixed" => Ok(Self::Fixed),
            "variable" => Ok(Self::Variable),
            "potentiometer" => Ok(Self::Potentiometer),
            "thermistor" => Ok(Self::Thermistor),
            "photoresistor" => Ok(Self::Photoresistor),
            "shunt" => Ok(Self::Shunt),
            "pull-up" => Ok(Self::PullUp),
            "pull-down" => Ok(Self::PullDown),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing resistor kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResistorKindParseError {
    /// The resistor kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ResistorKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("resistor kind cannot be empty"),
        }
    }
}

impl Error for ResistorKindParseError {}

/// A descriptive resistor specification.
#[derive(Clone, Debug, PartialEq)]
pub struct ResistorSpec {
    resistance: ResistanceValue,
    kind: ResistorKind,
    tolerance: Option<Tolerance>,
    power_rating: Option<PowerRating>,
}

impl ResistorSpec {
    /// Creates a resistor spec from resistance and kind.
    #[must_use]
    pub const fn new(resistance: ResistanceValue, kind: ResistorKind) -> Self {
        Self {
            resistance,
            kind,
            tolerance: None,
            power_rating: None,
        }
    }

    /// Returns the resistance value.
    #[must_use]
    pub const fn resistance(&self) -> ResistanceValue {
        self.resistance
    }

    /// Returns the resistor kind.
    #[must_use]
    pub fn kind(&self) -> ResistorKind {
        self.kind.clone()
    }

    /// Returns the optional tolerance.
    #[must_use]
    pub const fn tolerance(&self) -> Option<Tolerance> {
        self.tolerance
    }

    /// Returns the optional power rating.
    #[must_use]
    pub const fn power_rating(&self) -> Option<PowerRating> {
        self.power_rating
    }

    /// Returns this spec with a tolerance attached.
    #[must_use]
    pub const fn with_tolerance(mut self, tolerance: Tolerance) -> Self {
        self.tolerance = Some(tolerance);
        self
    }

    /// Returns this spec with a power rating attached.
    #[must_use]
    pub const fn with_power_rating(mut self, power_rating: PowerRating) -> Self {
        self.power_rating = Some(power_rating);
        self
    }
}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[cfg(test)]
mod tests {
    use super::{ResistanceValue, ResistanceValueError, ResistorKind, ResistorSpec};
    use use_rating::{PowerRating, Tolerance};

    #[test]
    fn accepts_valid_resistance() -> Result<(), ResistanceValueError> {
        let value = ResistanceValue::new_ohms(10_000.0)?;

        assert!((value.ohms() - 10_000.0).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn rejects_negative_resistance() {
        assert_eq!(
            ResistanceValue::new_ohms(-1.0),
            Err(ResistanceValueError::Negative)
        );
    }

    #[test]
    fn displays_and_parses_resistor_kinds() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!("pull up".parse::<ResistorKind>()?, ResistorKind::PullUp);
        assert_eq!(ResistorKind::Photoresistor.to_string(), "photoresistor");
        Ok(())
    }

    #[test]
    fn builds_resistor_specs_with_tolerance() -> Result<(), Box<dyn std::error::Error>> {
        let spec = ResistorSpec::new(ResistanceValue::new_ohms(1_000.0)?, ResistorKind::Fixed)
            .with_tolerance(Tolerance::from_percent(5.0)?);

        assert_eq!(spec.tolerance().map(Tolerance::percent), Some(5.0));
        Ok(())
    }

    #[test]
    fn builds_resistor_specs_with_power_rating() -> Result<(), Box<dyn std::error::Error>> {
        let spec = ResistorSpec::new(ResistanceValue::new_ohms(10.0)?, ResistorKind::Shunt)
            .with_power_rating(PowerRating::new_watts(1.0)?);

        assert_eq!(spec.power_rating().map(PowerRating::watts), Some(1.0));
        Ok(())
    }
}
