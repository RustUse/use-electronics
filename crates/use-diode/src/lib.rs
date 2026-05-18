#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_rating::{CurrentRating, VoltageRating};

/// Commonly used diode primitives.
pub mod prelude {
    pub use crate::{
        DiodeKind, DiodeKindParseError, DiodePolarity, DiodePolarityParseError, DiodeSpec,
    };
}

/// Descriptive diode kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiodeKind {
    Signal,
    Rectifier,
    Zener,
    Schottky,
    Led,
    Tvs,
    Photodiode,
    Unknown,
    Custom(String),
}

impl fmt::Display for DiodeKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Signal => "signal",
            Self::Rectifier => "rectifier",
            Self::Zener => "zener",
            Self::Schottky => "schottky",
            Self::Led => "led",
            Self::Tvs => "tvs",
            Self::Photodiode => "photodiode",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for DiodeKind {
    type Err = DiodeKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(DiodeKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "signal" => Ok(Self::Signal),
            "rectifier" => Ok(Self::Rectifier),
            "zener" => Ok(Self::Zener),
            "schottky" => Ok(Self::Schottky),
            "led" => Ok(Self::Led),
            "tvs" => Ok(Self::Tvs),
            "photodiode" => Ok(Self::Photodiode),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing diode kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiodeKindParseError {
    /// The diode kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for DiodeKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("diode kind cannot be empty"),
        }
    }
}

impl Error for DiodeKindParseError {}

/// Diode terminal polarity vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum DiodePolarity {
    Anode,
    Cathode,
}

impl fmt::Display for DiodePolarity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Anode => "anode",
            Self::Cathode => "cathode",
        })
    }
}

impl FromStr for DiodePolarity {
    type Err = DiodePolarityParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(DiodePolarityParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "anode" => Ok(Self::Anode),
            "cathode" => Ok(Self::Cathode),
            _ => Err(DiodePolarityParseError::Unknown),
        }
    }
}

/// Errors returned while parsing diode polarity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiodePolarityParseError {
    /// The polarity was empty after trimming whitespace.
    Empty,
    /// The polarity was not part of the fixed vocabulary.
    Unknown,
}

impl fmt::Display for DiodePolarityParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("diode polarity cannot be empty"),
            Self::Unknown => formatter.write_str("unknown diode polarity"),
        }
    }
}

impl Error for DiodePolarityParseError {}

/// A descriptive diode specification.
#[derive(Clone, Debug, PartialEq)]
pub struct DiodeSpec {
    kind: DiodeKind,
    forward_voltage: Option<VoltageRating>,
    reverse_voltage_rating: Option<VoltageRating>,
    current_rating: Option<CurrentRating>,
}

impl DiodeSpec {
    /// Creates a diode spec from a diode kind.
    #[must_use]
    pub const fn new(kind: DiodeKind) -> Self {
        Self {
            kind,
            forward_voltage: None,
            reverse_voltage_rating: None,
            current_rating: None,
        }
    }

    /// Returns the diode kind.
    #[must_use]
    pub fn kind(&self) -> DiodeKind {
        self.kind.clone()
    }

    /// Returns the optional forward voltage metadata.
    #[must_use]
    pub const fn forward_voltage(&self) -> Option<VoltageRating> {
        self.forward_voltage
    }

    /// Returns the optional reverse voltage rating.
    #[must_use]
    pub const fn reverse_voltage_rating(&self) -> Option<VoltageRating> {
        self.reverse_voltage_rating
    }

    /// Returns the optional current rating.
    #[must_use]
    pub const fn current_rating(&self) -> Option<CurrentRating> {
        self.current_rating
    }

    /// Returns this spec with forward voltage metadata attached.
    #[must_use]
    pub const fn with_forward_voltage(mut self, forward_voltage: VoltageRating) -> Self {
        self.forward_voltage = Some(forward_voltage);
        self
    }

    /// Returns this spec with reverse voltage rating metadata attached.
    #[must_use]
    pub const fn with_reverse_voltage_rating(mut self, reverse_voltage: VoltageRating) -> Self {
        self.reverse_voltage_rating = Some(reverse_voltage);
        self
    }

    /// Returns this spec with current rating metadata attached.
    #[must_use]
    pub const fn with_current_rating(mut self, current_rating: CurrentRating) -> Self {
        self.current_rating = Some(current_rating);
        self
    }
}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[cfg(test)]
mod tests {
    use super::{DiodeKind, DiodePolarity, DiodeSpec};
    use use_rating::{CurrentRating, VoltageRating};

    #[test]
    fn displays_and_parses_diode_kinds() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!("schottky".parse::<DiodeKind>()?, DiodeKind::Schottky);
        assert_eq!(DiodeKind::Photodiode.to_string(), "photodiode");
        Ok(())
    }

    #[test]
    fn supports_custom_diode_kinds() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            "pin-diode".parse::<DiodeKind>()?,
            DiodeKind::Custom("pin-diode".to_string())
        );
        Ok(())
    }

    #[test]
    fn displays_diode_polarity() {
        assert_eq!(DiodePolarity::Anode.to_string(), "anode");
        assert_eq!(DiodePolarity::Cathode.to_string(), "cathode");
    }

    #[test]
    fn builds_diode_specs_with_ratings() -> Result<(), Box<dyn std::error::Error>> {
        let spec = DiodeSpec::new(DiodeKind::Zener)
            .with_forward_voltage(VoltageRating::new_volts(0.7)?)
            .with_reverse_voltage_rating(VoltageRating::new_volts(5.1)?)
            .with_current_rating(CurrentRating::new_amperes(0.02)?);

        assert_eq!(spec.kind(), DiodeKind::Zener);
        assert_eq!(
            spec.current_rating().map(CurrentRating::amperes),
            Some(0.02)
        );
        Ok(())
    }
}
