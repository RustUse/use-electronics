#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Commonly used component primitives.
pub mod prelude {
    pub use crate::{
        ComponentId, ComponentKind, ComponentKindParseError, ComponentTextError, ComponentValue,
        ReferenceDesignator,
    };
}

/// Errors returned by non-empty component text wrappers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentTextError {
    /// The provided text was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ComponentTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("component text cannot be empty"),
        }
    }
}

impl Error for ComponentTextError {}

/// A stable component identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComponentId(String);

impl ComponentId {
    /// Creates a component ID from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ComponentTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ComponentTextError> {
        non_empty_component_text(value).map(Self)
    }

    /// Returns the stored identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the ID and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ComponentId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ComponentId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ComponentId {
    type Err = ComponentTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A reference designator such as `R1`, `C4`, `U2`, `D3`, `J1`, or `SW1`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReferenceDesignator(String);

impl ReferenceDesignator {
    /// Creates a reference designator from non-empty text.
    ///
    /// Casing is preserved; the value is not normalized beyond trimming edge whitespace.
    ///
    /// # Errors
    ///
    /// Returns [`ComponentTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ComponentTextError> {
        non_empty_component_text(value).map(Self)
    }

    /// Returns the reference designator text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the designator and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ReferenceDesignator {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ReferenceDesignator {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ReferenceDesignator {
    type Err = ComponentTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A descriptive component value such as `10k`, `100nF`, or `STM32F4`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComponentValue(String);

impl ComponentValue {
    /// Creates a component value from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ComponentTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, ComponentTextError> {
        non_empty_component_text(value).map(Self)
    }

    /// Returns the stored value text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the value and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for ComponentValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for ComponentValue {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for ComponentValue {
    type Err = ComponentTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A small electronic component classification vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ComponentKind {
    Resistor,
    Capacitor,
    Inductor,
    Diode,
    Transistor,
    IntegratedCircuit,
    Connector,
    Switch,
    Sensor,
    PowerSupply,
    Unknown,
    Custom(String),
}

impl ComponentKind {
    /// Creates a custom component kind from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`ComponentKindParseError::Empty`] when the trimmed value is empty.
    pub fn custom(value: impl AsRef<str>) -> Result<Self, ComponentKindParseError> {
        let trimmed = value.as_ref().trim();
        if trimmed.is_empty() {
            Err(ComponentKindParseError::Empty)
        } else {
            Ok(Self::Custom(trimmed.to_string()))
        }
    }
}

impl fmt::Display for ComponentKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Resistor => "resistor",
            Self::Capacitor => "capacitor",
            Self::Inductor => "inductor",
            Self::Diode => "diode",
            Self::Transistor => "transistor",
            Self::IntegratedCircuit => "integrated-circuit",
            Self::Connector => "connector",
            Self::Switch => "switch",
            Self::Sensor => "sensor",
            Self::PowerSupply => "power-supply",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for ComponentKind {
    type Err = ComponentKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(ComponentKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "resistor" => Ok(Self::Resistor),
            "capacitor" => Ok(Self::Capacitor),
            "inductor" => Ok(Self::Inductor),
            "diode" => Ok(Self::Diode),
            "transistor" => Ok(Self::Transistor),
            "integrated-circuit" | "ic" => Ok(Self::IntegratedCircuit),
            "connector" => Ok(Self::Connector),
            "switch" => Ok(Self::Switch),
            "sensor" => Ok(Self::Sensor),
            "power-supply" => Ok(Self::PowerSupply),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing component kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ComponentKindParseError {
    /// The kind text was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for ComponentKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("component kind cannot be empty"),
        }
    }
}

impl Error for ComponentKindParseError {}

fn non_empty_component_text(value: impl AsRef<str>) -> Result<String, ComponentTextError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(ComponentTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{ComponentKind, ComponentKindParseError, ComponentTextError, ReferenceDesignator};

    #[test]
    fn accepts_valid_reference_designators() -> Result<(), ComponentTextError> {
        let reference = ReferenceDesignator::new("R1")?;

        assert_eq!(reference.as_str(), "R1");
        assert_eq!(reference.to_string(), "R1");
        Ok(())
    }

    #[test]
    fn rejects_empty_reference_designators() {
        assert_eq!(
            ReferenceDesignator::new("  "),
            Err(ComponentTextError::Empty)
        );
    }

    #[test]
    fn displays_and_parses_component_kinds() -> Result<(), ComponentKindParseError> {
        assert_eq!(
            "resistor".parse::<ComponentKind>()?,
            ComponentKind::Resistor
        );
        assert_eq!(
            "Integrated Circuit".parse::<ComponentKind>()?,
            ComponentKind::IntegratedCircuit
        );
        assert_eq!(ComponentKind::PowerSupply.to_string(), "power-supply");
        Ok(())
    }

    #[test]
    fn supports_custom_component_kinds() -> Result<(), ComponentKindParseError> {
        let kind = ComponentKind::custom("fuse")?;

        assert_eq!(kind, ComponentKind::Custom("fuse".to_string()));
        assert_eq!("relay".parse::<ComponentKind>()?.to_string(), "relay");
        Ok(())
    }

    #[test]
    fn sorts_reference_designators_deterministically() -> Result<(), ComponentTextError> {
        let references = BTreeSet::from([
            ReferenceDesignator::new("R2")?,
            ReferenceDesignator::new("R1")?,
            ReferenceDesignator::new("C1")?,
        ]);
        let ordered: Vec<_> = references.iter().map(ReferenceDesignator::as_str).collect();

        assert_eq!(ordered, vec!["C1", "R1", "R2"]);
        Ok(())
    }
}
