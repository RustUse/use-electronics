#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, num::NonZeroU32, str::FromStr};
use std::error::Error;

use use_component::ReferenceDesignator;

/// Commonly used pin primitives.
pub mod prelude {
    pub use crate::{
        PinIdentifier, PinName, PinNameError, PinNumber, PinNumberError, PinPolarity,
        PinPolarityParseError, PinRef, PinRole, PinRoleParseError,
    };
}

/// A one-based package or component pin number.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PinNumber(NonZeroU32);

impl PinNumber {
    /// Creates a non-zero pin number.
    ///
    /// # Errors
    ///
    /// Returns [`PinNumberError::Zero`] when `value` is zero.
    pub fn new(value: u32) -> Result<Self, PinNumberError> {
        NonZeroU32::new(value).map(Self).ok_or(PinNumberError::Zero)
    }

    /// Returns the pin number.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0.get()
    }
}

impl From<NonZeroU32> for PinNumber {
    fn from(value: NonZeroU32) -> Self {
        Self(value)
    }
}

impl TryFrom<u32> for PinNumber {
    type Error = PinNumberError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl fmt::Display for PinNumber {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

/// Errors returned while constructing pin numbers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinNumberError {
    /// Pin number zero is not accepted.
    Zero,
}

impl fmt::Display for PinNumberError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => formatter.write_str("pin number must be non-zero"),
        }
    }
}

impl Error for PinNumberError {}

/// A descriptive pin name such as `VCC`, `GND`, `SDA`, or `RESET`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PinName(String);

impl PinName {
    /// Creates a pin name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`PinNameError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PinNameError> {
        let trimmed = value.as_ref().trim();
        if trimmed.is_empty() {
            Err(PinNameError::Empty)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    /// Returns the pin name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the pin name and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for PinName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PinName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PinName {
    type Err = PinNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors returned while constructing pin names.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinNameError {
    /// The pin name was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for PinNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("pin name cannot be empty"),
        }
    }
}

impl Error for PinNameError {}

/// Descriptive electronic pin roles.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PinRole {
    Input,
    Output,
    Bidirectional,
    Power,
    Ground,
    Clock,
    Reset,
    Enable,
    NoConnect,
    Unknown,
    Custom(String),
}

impl fmt::Display for PinRole {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Input => "input",
            Self::Output => "output",
            Self::Bidirectional => "bidirectional",
            Self::Power => "power",
            Self::Ground => "ground",
            Self::Clock => "clock",
            Self::Reset => "reset",
            Self::Enable => "enable",
            Self::NoConnect => "no-connect",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for PinRole {
    type Err = PinRoleParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(PinRoleParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "input" | "in" => Ok(Self::Input),
            "output" | "out" => Ok(Self::Output),
            "bidirectional" | "bidirectional-io" | "io" => Ok(Self::Bidirectional),
            "power" => Ok(Self::Power),
            "ground" | "gnd" => Ok(Self::Ground),
            "clock" | "clk" => Ok(Self::Clock),
            "reset" => Ok(Self::Reset),
            "enable" => Ok(Self::Enable),
            "no-connect" | "nc" => Ok(Self::NoConnect),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing pin roles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinRoleParseError {
    /// The role was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for PinRoleParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("pin role cannot be empty"),
        }
    }
}

impl Error for PinRoleParseError {}

/// Descriptive pin polarity vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PinPolarity {
    ActiveHigh,
    ActiveLow,
    NonInverting,
    Inverting,
    Unknown,
}

impl fmt::Display for PinPolarity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::ActiveHigh => "active-high",
            Self::ActiveLow => "active-low",
            Self::NonInverting => "non-inverting",
            Self::Inverting => "inverting",
            Self::Unknown => "unknown",
        })
    }
}

impl FromStr for PinPolarity {
    type Err = PinPolarityParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(PinPolarityParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "active-high" => Ok(Self::ActiveHigh),
            "active-low" => Ok(Self::ActiveLow),
            "non-inverting" => Ok(Self::NonInverting),
            "inverting" => Ok(Self::Inverting),
            "unknown" => Ok(Self::Unknown),
            _ => Err(PinPolarityParseError::Unknown),
        }
    }
}

/// Errors returned while parsing pin polarity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinPolarityParseError {
    /// The polarity text was empty after trimming whitespace.
    Empty,
    /// The polarity was not part of the fixed vocabulary.
    Unknown,
}

impl fmt::Display for PinPolarityParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("pin polarity cannot be empty"),
            Self::Unknown => formatter.write_str("unknown pin polarity"),
        }
    }
}

impl Error for PinPolarityParseError {}

/// A pin identified by number or name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PinIdentifier {
    Number(PinNumber),
    Name(PinName),
}

impl From<PinNumber> for PinIdentifier {
    fn from(value: PinNumber) -> Self {
        Self::Number(value)
    }
}

impl From<PinName> for PinIdentifier {
    fn from(value: PinName) -> Self {
        Self::Name(value)
    }
}

impl fmt::Display for PinIdentifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(number) => number.fmt(formatter),
            Self::Name(name) => name.fmt(formatter),
        }
    }
}

/// A reference to a component pin, such as `U2:VCC` or `R1:1`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PinRef {
    component: ReferenceDesignator,
    pin: PinIdentifier,
}

impl PinRef {
    /// Creates a pin reference from a component designator and pin identifier.
    #[must_use]
    pub const fn new(component: ReferenceDesignator, pin: PinIdentifier) -> Self {
        Self { component, pin }
    }

    /// Creates a pin reference from a numeric pin.
    #[must_use]
    pub const fn numbered(component: ReferenceDesignator, pin: PinNumber) -> Self {
        Self::new(component, PinIdentifier::Number(pin))
    }

    /// Creates a pin reference from a named pin.
    #[must_use]
    pub const fn named(component: ReferenceDesignator, pin: PinName) -> Self {
        Self::new(component, PinIdentifier::Name(pin))
    }

    /// Returns the component reference designator.
    #[must_use]
    pub const fn component(&self) -> &ReferenceDesignator {
        &self.component
    }

    /// Returns the pin identifier.
    #[must_use]
    pub const fn pin(&self) -> &PinIdentifier {
        &self.pin
    }
}

impl fmt::Display for PinRef {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}:{}", self.component, self.pin)
    }
}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[cfg(test)]
mod tests {
    use super::{
        PinName, PinNameError, PinNumber, PinNumberError, PinPolarity, PinRole, PinRoleParseError,
    };

    #[test]
    fn accepts_valid_pin_numbers() -> Result<(), PinNumberError> {
        let number = PinNumber::new(1)?;

        assert_eq!(number.get(), 1);
        assert_eq!(number.to_string(), "1");
        Ok(())
    }

    #[test]
    fn rejects_zero_pin_numbers() {
        assert_eq!(PinNumber::new(0), Err(PinNumberError::Zero));
    }

    #[test]
    fn accepts_valid_pin_names() -> Result<(), PinNameError> {
        let name = PinName::new("RESET")?;

        assert_eq!(name.as_str(), "RESET");
        assert_eq!(name.to_string(), "RESET");
        Ok(())
    }

    #[test]
    fn rejects_empty_pin_names() {
        assert_eq!(PinName::new(" "), Err(PinNameError::Empty));
    }

    #[test]
    fn displays_and_parses_pin_roles() -> Result<(), PinRoleParseError> {
        assert_eq!("input".parse::<PinRole>()?, PinRole::Input);
        assert_eq!("NC".parse::<PinRole>()?, PinRole::NoConnect);
        assert_eq!(PinRole::Power.to_string(), "power");
        Ok(())
    }

    #[test]
    fn displays_and_parses_pin_polarity() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!("active low".parse::<PinPolarity>()?, PinPolarity::ActiveLow);
        assert_eq!(PinPolarity::NonInverting.to_string(), "non-inverting");
        Ok(())
    }
}
