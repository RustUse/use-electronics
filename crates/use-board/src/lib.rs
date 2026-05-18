#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, num::NonZeroU8, str::FromStr};
use std::error::Error;

/// Commonly used board primitives.
pub mod prelude {
    pub use crate::{
        AssemblySide, BoardId, BoardLayer, BoardLayerParseError, BoardName, BoardSide,
        BoardSideParseError, BoardTextError, LayerCount, LayerCountError,
    };
}

/// Errors returned by non-empty board text wrappers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoardTextError {
    /// The text was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for BoardTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("board text cannot be empty"),
        }
    }
}

impl Error for BoardTextError {}

/// A stable board identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BoardId(String);

impl BoardId {
    /// Creates a board ID from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`BoardTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, BoardTextError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the ID text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for BoardId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for BoardId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for BoardId {
    type Err = BoardTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A non-empty board name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BoardName(String);

impl BoardName {
    /// Creates a board name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`BoardTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, BoardTextError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the board name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for BoardName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for BoardName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for BoardName {
    type Err = BoardTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// Board side vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BoardSide {
    Top,
    Bottom,
}

impl fmt::Display for BoardSide {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
        })
    }
}

impl FromStr for BoardSide {
    type Err = BoardSideParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(BoardSideParseError::Empty);
        }

        match trimmed.to_ascii_lowercase().as_str() {
            "top" => Ok(Self::Top),
            "bottom" => Ok(Self::Bottom),
            _ => Err(BoardSideParseError::Unknown),
        }
    }
}

/// Errors returned while parsing board sides.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoardSideParseError {
    /// The side was empty after trimming whitespace.
    Empty,
    /// The side was not part of the fixed vocabulary.
    Unknown,
}

impl fmt::Display for BoardSideParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("board side cannot be empty"),
            Self::Unknown => formatter.write_str("unknown board side"),
        }
    }
}

impl Error for BoardSideParseError {}

/// Simple board layer vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BoardLayer {
    TopCopper,
    BottomCopper,
    InnerCopper(u8),
    SilkscreenTop,
    SilkscreenBottom,
    SolderMaskTop,
    SolderMaskBottom,
    Mechanical,
    Custom(String),
}

impl fmt::Display for BoardLayer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TopCopper => formatter.write_str("top-copper"),
            Self::BottomCopper => formatter.write_str("bottom-copper"),
            Self::InnerCopper(index) => write!(formatter, "inner-copper-{index}"),
            Self::SilkscreenTop => formatter.write_str("silkscreen-top"),
            Self::SilkscreenBottom => formatter.write_str("silkscreen-bottom"),
            Self::SolderMaskTop => formatter.write_str("solder-mask-top"),
            Self::SolderMaskBottom => formatter.write_str("solder-mask-bottom"),
            Self::Mechanical => formatter.write_str("mechanical"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

impl FromStr for BoardLayer {
    type Err = BoardLayerParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(BoardLayerParseError::Empty);
        }

        let normalized = normalized_token(trimmed);
        if let Some(index) = normalized.strip_prefix("inner-copper-") {
            return index
                .parse::<NonZeroU8>()
                .map(|value| Self::InnerCopper(value.get()))
                .map_err(|_| BoardLayerParseError::InvalidInnerCopperIndex);
        }

        match normalized.as_str() {
            "top-copper" => Ok(Self::TopCopper),
            "bottom-copper" => Ok(Self::BottomCopper),
            "silkscreen-top" => Ok(Self::SilkscreenTop),
            "silkscreen-bottom" => Ok(Self::SilkscreenBottom),
            "solder-mask-top" => Ok(Self::SolderMaskTop),
            "solder-mask-bottom" => Ok(Self::SolderMaskBottom),
            "mechanical" => Ok(Self::Mechanical),
            _ => Ok(Self::Custom(trimmed.to_string())),
        }
    }
}

/// Errors returned while parsing board layers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoardLayerParseError {
    /// The layer was empty after trimming whitespace.
    Empty,
    /// An inner copper layer index was zero or not numeric.
    InvalidInnerCopperIndex,
}

impl fmt::Display for BoardLayerParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("board layer cannot be empty"),
            Self::InvalidInnerCopperIndex => {
                formatter.write_str("inner copper layer index must be non-zero")
            },
        }
    }
}

impl Error for BoardLayerParseError {}

/// A non-zero board layer count.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct LayerCount(NonZeroU8);

impl LayerCount {
    /// Creates a non-zero board layer count.
    ///
    /// # Errors
    ///
    /// Returns [`LayerCountError::Zero`] when `value` is zero.
    pub fn new(value: u8) -> Result<Self, LayerCountError> {
        NonZeroU8::new(value).map(Self).ok_or(LayerCountError::Zero)
    }

    /// Returns the layer count.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0.get()
    }
}

impl fmt::Display for LayerCount {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

/// Errors returned while constructing layer counts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LayerCountError {
    /// Zero is not a valid layer count.
    Zero,
}

impl fmt::Display for LayerCountError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Zero => formatter.write_str("layer count must be non-zero"),
        }
    }
}

impl Error for LayerCountError {}

/// Assembly side vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum AssemblySide {
    Top,
    Bottom,
    Both,
    Unknown,
}

impl fmt::Display for AssemblySide {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Both => "both",
            Self::Unknown => "unknown",
        })
    }
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, BoardTextError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(BoardTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[cfg(test)]
mod tests {
    use super::{BoardLayer, BoardName, BoardSide, BoardTextError, LayerCount, LayerCountError};

    #[test]
    fn accepts_valid_board_names() -> Result<(), BoardTextError> {
        let name = BoardName::new("main board")?;

        assert_eq!(name.as_str(), "main board");
        Ok(())
    }

    #[test]
    fn rejects_empty_board_names() {
        assert_eq!(BoardName::new(""), Err(BoardTextError::Empty));
    }

    #[test]
    fn accepts_valid_layer_counts() -> Result<(), LayerCountError> {
        let count = LayerCount::new(2)?;

        assert_eq!(count.get(), 2);
        Ok(())
    }

    #[test]
    fn rejects_zero_layer_counts() {
        assert_eq!(LayerCount::new(0), Err(LayerCountError::Zero));
    }

    #[test]
    fn displays_and_parses_board_sides() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!("top".parse::<BoardSide>()?, BoardSide::Top);
        assert_eq!(BoardSide::Bottom.to_string(), "bottom");
        Ok(())
    }

    #[test]
    fn displays_and_parses_board_layers() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!("top copper".parse::<BoardLayer>()?, BoardLayer::TopCopper);
        assert_eq!(
            "inner-copper-2".parse::<BoardLayer>()?,
            BoardLayer::InnerCopper(2)
        );
        assert_eq!(BoardLayer::SolderMaskTop.to_string(), "solder-mask-top");
        Ok(())
    }
}
