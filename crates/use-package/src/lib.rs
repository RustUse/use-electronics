#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, num::NonZeroU32, str::FromStr};
use std::error::Error;

/// Commonly used package primitives.
pub mod prelude {
    pub use crate::{
        PackageKind, PackageKindParseError, PackageName, PackageNameError, PackagePitch,
        PackagePitchError, PinCount, PinCountError,
    };
}

/// A non-empty package name such as `DIP-8`, `SOIC-16`, `TO-220`, or `0603`.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PackageName(String);

impl PackageName {
    /// Creates a package name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`PackageNameError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, PackageNameError> {
        let trimmed = value.as_ref().trim();
        if trimmed.is_empty() {
            Err(PackageNameError::Empty)
        } else {
            Ok(Self(trimmed.to_string()))
        }
    }

    /// Returns the package name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes the package name and returns the owned string.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl AsRef<str> for PackageName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for PackageName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for PackageName {
    type Err = PackageNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Errors returned while constructing package names.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageNameError {
    /// The package name was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for PackageNameError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("package name cannot be empty"),
        }
    }
}

impl Error for PackageNameError {}

/// Common electronic package families.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PackageKind {
    Dip,
    Soic,
    Sop,
    Tssop,
    Qfp,
    Qfn,
    Bga,
    Sot,
    To,
    Chip,
    ThroughHole,
    Unknown,
    Custom(String),
}

impl fmt::Display for PackageKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Dip => "dip",
            Self::Soic => "soic",
            Self::Sop => "sop",
            Self::Tssop => "tssop",
            Self::Qfp => "qfp",
            Self::Qfn => "qfn",
            Self::Bga => "bga",
            Self::Sot => "sot",
            Self::To => "to",
            Self::Chip => "chip",
            Self::ThroughHole => "through-hole",
            Self::Unknown => "unknown",
            Self::Custom(value) => value.as_str(),
        })
    }
}

impl FromStr for PackageKind {
    type Err = PackageKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(PackageKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "dip" => Ok(Self::Dip),
            "soic" => Ok(Self::Soic),
            "sop" => Ok(Self::Sop),
            "tssop" => Ok(Self::Tssop),
            "qfp" => Ok(Self::Qfp),
            "qfn" => Ok(Self::Qfn),
            "bga" => Ok(Self::Bga),
            "sot" => Ok(Self::Sot),
            "to" => Ok(Self::To),
            "chip" => Ok(Self::Chip),
            "through-hole" => Ok(Self::ThroughHole),
            "unknown" => Ok(Self::Unknown),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing package kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackageKindParseError {
    /// The kind was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for PackageKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("package kind cannot be empty"),
        }
    }
}

impl Error for PackageKindParseError {}

/// A non-zero package pin count.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PinCount(NonZeroU32);

impl PinCount {
    /// Creates a non-zero pin count.
    ///
    /// # Errors
    ///
    /// Returns [`PinCountError::Zero`] when `value` is zero.
    pub fn new(value: u32) -> Result<Self, PinCountError> {
        NonZeroU32::new(value).map(Self).ok_or(PinCountError::Zero)
    }

    /// Returns the pin count.
    #[must_use]
    pub const fn get(self) -> u32 {
        self.0.get()
    }
}

impl fmt::Display for PinCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

/// Errors returned while constructing pin counts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PinCountError {
    /// Zero is not a valid pin count.
    Zero,
}

impl fmt::Display for PinCountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => formatter.write_str("pin count must be non-zero"),
        }
    }
}

impl Error for PinCountError {}

/// A simple package pitch value in millimeters.
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct PackagePitch {
    millimeters: f64,
}

impl PackagePitch {
    /// Creates a positive pitch in millimeters.
    ///
    /// # Errors
    ///
    /// Returns [`PackagePitchError`] when the value is not finite or not positive.
    pub fn from_millimeters(value: f64) -> Result<Self, PackagePitchError> {
        if !value.is_finite() {
            return Err(PackagePitchError::NonFinite);
        }

        if value <= 0.0 {
            return Err(PackagePitchError::NonPositive);
        }

        Ok(Self { millimeters: value })
    }

    /// Returns the pitch in millimeters.
    #[must_use]
    pub const fn millimeters(self) -> f64 {
        self.millimeters
    }
}

impl fmt::Display for PackagePitch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} mm", self.millimeters)
    }
}

/// Errors returned while constructing package pitch values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PackagePitchError {
    /// The pitch was not finite.
    NonFinite,
    /// The pitch was zero or negative.
    NonPositive,
}

impl fmt::Display for PackagePitchError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFinite => formatter.write_str("package pitch must be finite"),
            Self::NonPositive => formatter.write_str("package pitch must be positive"),
        }
    }
}

impl Error for PackagePitchError {}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[cfg(test)]
mod tests {
    use super::{
        PackageKind, PackageKindParseError, PackageName, PackageNameError, PinCount, PinCountError,
    };

    #[test]
    fn accepts_valid_package_names() -> Result<(), PackageNameError> {
        let name = PackageName::new("QFN-32")?;

        assert_eq!(name.as_str(), "QFN-32");
        assert_eq!(name.to_string(), "QFN-32");
        Ok(())
    }

    #[test]
    fn rejects_empty_package_names() {
        assert_eq!(PackageName::new(""), Err(PackageNameError::Empty));
    }

    #[test]
    fn accepts_valid_pin_counts() -> Result<(), PinCountError> {
        let count = PinCount::new(8)?;

        assert_eq!(count.get(), 8);
        assert_eq!(count.to_string(), "8");
        Ok(())
    }

    #[test]
    fn rejects_zero_pin_counts() {
        assert_eq!(PinCount::new(0), Err(PinCountError::Zero));
    }

    #[test]
    fn displays_and_parses_package_kinds() -> Result<(), PackageKindParseError> {
        assert_eq!("SOIC".parse::<PackageKind>()?, PackageKind::Soic);
        assert_eq!(
            "through hole".parse::<PackageKind>()?,
            PackageKind::ThroughHole
        );
        assert_eq!(PackageKind::Bga.to_string(), "bga");
        Ok(())
    }

    #[test]
    fn supports_custom_package_kinds() -> Result<(), PackageKindParseError> {
        assert_eq!(
            "wafer-level".parse::<PackageKind>()?,
            PackageKind::Custom("wafer-level".to_string())
        );
        Ok(())
    }
}
