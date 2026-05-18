#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Commonly used net-label primitives.
pub mod prelude {
    pub use crate::{
        GroundKind, GroundKindParseError, NetLabel, PowerRail, SignalName, TextLabelError,
    };
}

/// Errors returned by non-empty label wrappers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TextLabelError {
    /// The label was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for TextLabelError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("label text cannot be empty"),
        }
    }
}

impl Error for TextLabelError {}

/// A non-empty net label.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NetLabel(String);

impl NetLabel {
    /// Creates a net label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`TextLabelError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, TextLabelError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the label text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns whether this label is one of the obvious common ground labels.
    #[must_use]
    pub fn is_ground(&self) -> bool {
        matches!(
            self.0.to_ascii_uppercase().as_str(),
            "GND" | "AGND" | "DGND" | "CHASSIS_GND" | "EARTH_GND"
        )
    }

    /// Returns whether this label is one of the obvious common power-like labels.
    #[must_use]
    pub fn is_power_like(&self) -> bool {
        matches!(
            self.0.to_ascii_uppercase().as_str(),
            "VCC" | "VDD" | "VSS" | "VIN" | "VBAT" | "3V3" | "5V" | "1V8" | "12V"
        )
    }
}

impl AsRef<str> for NetLabel {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for NetLabel {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for NetLabel {
    type Err = TextLabelError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A descriptive signal name such as `SDA`, `SCL`, `CLK`, or `RESET`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SignalName(String);

impl SignalName {
    /// Creates a signal name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`TextLabelError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, TextLabelError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the signal name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for SignalName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for SignalName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for SignalName {
    type Err = TextLabelError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A descriptive power rail label such as `VCC`, `3V3`, or `VIN`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PowerRail(String);

impl PowerRail {
    /// Creates a power rail label from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`TextLabelError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, TextLabelError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the power rail text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for PowerRail {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PowerRail {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PowerRail {
    type Err = TextLabelError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Common ground label vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum GroundKind {
    Ground,
    AnalogGround,
    DigitalGround,
    ChassisGround,
    EarthGround,
    Unknown,
    Custom(String),
}

impl fmt::Display for GroundKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Ground => "ground",
            Self::AnalogGround => "analog-ground",
            Self::DigitalGround => "digital-ground",
            Self::ChassisGround => "chassis-ground",
            Self::EarthGround => "earth-ground",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for GroundKind {
    type Err = GroundKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(GroundKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "ground" | "gnd" => Ok(Self::Ground),
            "analog-ground" | "agnd" => Ok(Self::AnalogGround),
            "digital-ground" | "dgnd" => Ok(Self::DigitalGround),
            "chassis-ground" => Ok(Self::ChassisGround),
            "earth-ground" => Ok(Self::EarthGround),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing ground kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GroundKindParseError {
    /// The ground kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for GroundKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("ground kind cannot be empty"),
        }
    }
}

impl Error for GroundKindParseError {}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, TextLabelError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(TextLabelError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[cfg(test)]
mod tests {
    use super::{GroundKind, GroundKindParseError, NetLabel, SignalName, TextLabelError};

    #[test]
    fn accepts_valid_net_labels() -> Result<(), TextLabelError> {
        let label = NetLabel::new("3V3")?;

        assert_eq!(label.as_str(), "3V3");
        assert!(label.is_power_like());
        Ok(())
    }

    #[test]
    fn rejects_empty_net_labels() {
        assert_eq!(NetLabel::new("  "), Err(TextLabelError::Empty));
    }

    #[test]
    fn displays_and_parses_ground_kinds() -> Result<(), GroundKindParseError> {
        assert_eq!("AGND".parse::<GroundKind>()?, GroundKind::AnalogGround);
        assert_eq!(GroundKind::DigitalGround.to_string(), "digital-ground");
        Ok(())
    }

    #[test]
    fn constructs_signal_names() -> Result<(), TextLabelError> {
        let signal = SignalName::new("SCL")?;

        assert_eq!(signal.as_str(), "SCL");
        assert_eq!(signal.to_string(), "SCL");
        Ok(())
    }

    #[test]
    fn preserves_common_label_text() -> Result<(), TextLabelError> {
        let ground = NetLabel::new("GND")?;
        let reset = NetLabel::new("RESET")?;

        assert_eq!(ground.as_str(), "GND");
        assert!(ground.is_ground());
        assert_eq!(reset.as_str(), "RESET");
        assert!(!reset.is_power_like());
        Ok(())
    }
}
