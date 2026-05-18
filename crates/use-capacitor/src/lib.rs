#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_rating::VoltageRating;

/// Commonly used capacitor primitives.
pub mod prelude {
    pub use crate::{
        CapacitanceValue, CapacitanceValueError, CapacitorKind, CapacitorKindParseError,
        CapacitorPolarity, CapacitorPolarityParseError, CapacitorSpec,
    };
}

/// A capacitance value in farads.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct CapacitanceValue {
    farads: f64,
}

impl CapacitanceValue {
    /// Creates a non-negative capacitance value in farads.
    ///
    /// # Errors
    ///
    /// Returns [`CapacitanceValueError`] when the value is not finite or is negative.
    pub fn new_farads(value: f64) -> Result<Self, CapacitanceValueError> {
        if !value.is_finite() {
            return Err(CapacitanceValueError::NonFinite);
        }

        if value < 0.0 {
            return Err(CapacitanceValueError::Negative);
        }

        Ok(Self { farads: value })
    }

    /// Returns the value in farads.
    #[must_use]
    pub const fn farads(self) -> f64 {
        self.farads
    }
}

impl fmt::Display for CapacitanceValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} F", self.farads)
    }
}

/// Errors returned while constructing capacitance values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapacitanceValueError {
    /// The capacitance was not finite.
    NonFinite,
    /// The capacitance was negative.
    Negative,
}

impl fmt::Display for CapacitanceValueError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("capacitance must be finite"),
            Self::Negative => formatter.write_str("capacitance cannot be negative"),
        }
    }
}

impl Error for CapacitanceValueError {}

/// Descriptive capacitor kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CapacitorKind {
    Ceramic,
    Electrolytic,
    Tantalum,
    Film,
    Supercapacitor,
    Variable,
    Unknown,
    Custom(String),
}

impl fmt::Display for CapacitorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Ceramic => "ceramic",
            Self::Electrolytic => "electrolytic",
            Self::Tantalum => "tantalum",
            Self::Film => "film",
            Self::Supercapacitor => "supercapacitor",
            Self::Variable => "variable",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for CapacitorKind {
    type Err = CapacitorKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(CapacitorKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "ceramic" => Ok(Self::Ceramic),
            "electrolytic" => Ok(Self::Electrolytic),
            "tantalum" => Ok(Self::Tantalum),
            "film" => Ok(Self::Film),
            "supercapacitor" => Ok(Self::Supercapacitor),
            "variable" => Ok(Self::Variable),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing capacitor kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapacitorKindParseError {
    /// The capacitor kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for CapacitorKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("capacitor kind cannot be empty"),
        }
    }
}

impl Error for CapacitorKindParseError {}

/// Descriptive capacitor polarity vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CapacitorPolarity {
    Polarized,
    NonPolarized,
    Unknown,
}

impl fmt::Display for CapacitorPolarity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Polarized => "polarized",
            Self::NonPolarized => "non-polarized",
            Self::Unknown => "unknown",
        })
    }
}

impl FromStr for CapacitorPolarity {
    type Err = CapacitorPolarityParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(CapacitorPolarityParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "polarized" => Ok(Self::Polarized),
            "non-polarized" | "nonpolarized" => Ok(Self::NonPolarized),
            "unknown" => Ok(Self::Unknown),
            _ => Err(CapacitorPolarityParseError::Unknown),
        }
    }
}

/// Errors returned while parsing capacitor polarity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapacitorPolarityParseError {
    /// The polarity was empty after trimming whitespace.
    Empty,
    /// The polarity was not part of the fixed vocabulary.
    Unknown,
}

impl fmt::Display for CapacitorPolarityParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("capacitor polarity cannot be empty"),
            Self::Unknown => formatter.write_str("unknown capacitor polarity"),
        }
    }
}

impl Error for CapacitorPolarityParseError {}

/// A descriptive capacitor specification.
#[derive(Clone, Debug, PartialEq)]
pub struct CapacitorSpec {
    capacitance: CapacitanceValue,
    kind: CapacitorKind,
    polarity: CapacitorPolarity,
    voltage_rating: Option<VoltageRating>,
}

impl CapacitorSpec {
    /// Creates a capacitor spec from capacitance and kind.
    #[must_use]
    pub const fn new(capacitance: CapacitanceValue, kind: CapacitorKind) -> Self {
        Self {
            capacitance,
            kind,
            polarity: CapacitorPolarity::Unknown,
            voltage_rating: None,
        }
    }

    /// Returns the capacitance value.
    #[must_use]
    pub const fn capacitance(&self) -> CapacitanceValue {
        self.capacitance
    }

    /// Returns the capacitor kind.
    #[must_use]
    pub fn kind(&self) -> CapacitorKind {
        self.kind.clone()
    }

    /// Returns the capacitor polarity.
    #[must_use]
    pub const fn polarity(&self) -> CapacitorPolarity {
        self.polarity
    }

    /// Returns the optional voltage rating.
    #[must_use]
    pub const fn voltage_rating(&self) -> Option<VoltageRating> {
        self.voltage_rating
    }

    /// Returns this spec with polarity attached.
    #[must_use]
    pub const fn with_polarity(mut self, polarity: CapacitorPolarity) -> Self {
        self.polarity = polarity;
        self
    }

    /// Returns this spec with a voltage rating attached.
    #[must_use]
    pub const fn with_voltage_rating(mut self, voltage_rating: VoltageRating) -> Self {
        self.voltage_rating = Some(voltage_rating);
        self
    }
}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[cfg(test)]
mod tests {
    use super::{
        CapacitanceValue, CapacitanceValueError, CapacitorKind, CapacitorPolarity, CapacitorSpec,
    };
    use use_rating::VoltageRating;

    #[test]
    fn accepts_valid_capacitance() -> Result<(), CapacitanceValueError> {
        let value = CapacitanceValue::new_farads(0.000_001)?;

        assert!((value.farads() - 0.000_001).abs() < f64::EPSILON);
        Ok(())
    }

    #[test]
    fn rejects_negative_capacitance() {
        assert_eq!(
            CapacitanceValue::new_farads(-1.0),
            Err(CapacitanceValueError::Negative)
        );
    }

    #[test]
    fn displays_and_parses_capacitor_kinds() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!("ceramic".parse::<CapacitorKind>()?, CapacitorKind::Ceramic);
        assert_eq!(CapacitorKind::Supercapacitor.to_string(), "supercapacitor");
        Ok(())
    }

    #[test]
    fn displays_and_parses_polarity() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            "non polarized".parse::<CapacitorPolarity>()?,
            CapacitorPolarity::NonPolarized
        );
        assert_eq!(CapacitorPolarity::Polarized.to_string(), "polarized");
        Ok(())
    }

    #[test]
    fn builds_capacitor_specs_with_voltage_rating() -> Result<(), Box<dyn std::error::Error>> {
        let spec = CapacitorSpec::new(
            CapacitanceValue::new_farads(0.000_001)?,
            CapacitorKind::Ceramic,
        )
        .with_voltage_rating(VoltageRating::new_volts(16.0)?);

        assert_eq!(spec.voltage_rating().map(VoltageRating::volts), Some(16.0));
        Ok(())
    }
}
