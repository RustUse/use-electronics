#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

/// Commonly used transistor primitives.
pub mod prelude {
    pub use crate::{
        BjtKind, BjtKindParseError, FetKind, FetKindParseError, TerminalParseError, TransistorKind,
        TransistorSpec, TransistorTerminal,
    };
}

/// Bipolar junction transistor kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum BjtKind {
    Npn,
    Pnp,
}

impl fmt::Display for BjtKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Npn => "npn",
            Self::Pnp => "pnp",
        })
    }
}

impl FromStr for BjtKind {
    type Err = BjtKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(BjtKindParseError::Empty);
        }

        match trimmed.to_ascii_lowercase().as_str() {
            "npn" => Ok(Self::Npn),
            "pnp" => Ok(Self::Pnp),
            _ => Err(BjtKindParseError::Unknown),
        }
    }
}

/// Errors returned while parsing BJT kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BjtKindParseError {
    /// The kind was empty after trimming whitespace.
    Empty,
    /// The kind was not part of the fixed vocabulary.
    Unknown,
}

impl fmt::Display for BjtKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("BJT kind cannot be empty"),
            Self::Unknown => formatter.write_str("unknown BJT kind"),
        }
    }
}

impl Error for BjtKindParseError {}

/// Field-effect transistor kind.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FetKind {
    Nmos,
    Pmos,
    JfetN,
    JfetP,
}

impl fmt::Display for FetKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Nmos => "nmos",
            Self::Pmos => "pmos",
            Self::JfetN => "jfet-n",
            Self::JfetP => "jfet-p",
        })
    }
}

impl FromStr for FetKind {
    type Err = FetKindParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(FetKindParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "nmos" => Ok(Self::Nmos),
            "pmos" => Ok(Self::Pmos),
            "jfet-n" => Ok(Self::JfetN),
            "jfet-p" => Ok(Self::JfetP),
            _ => Err(FetKindParseError::Unknown),
        }
    }
}

/// Errors returned while parsing FET kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FetKindParseError {
    /// The kind was empty after trimming whitespace.
    Empty,
    /// The kind was not part of the fixed vocabulary.
    Unknown,
}

impl fmt::Display for FetKindParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("FET kind cannot be empty"),
            Self::Unknown => formatter.write_str("unknown FET kind"),
        }
    }
}

impl Error for FetKindParseError {}

/// Descriptive transistor kind vocabulary.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TransistorKind {
    Bjt(BjtKind),
    Fet(FetKind),
    Unknown,
    Custom(String),
}

impl fmt::Display for TransistorKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bjt(kind) => write!(formatter, "bjt:{kind}"),
            Self::Fet(kind) => write!(formatter, "fet:{kind}"),
            Self::Unknown => formatter.write_str("unknown"),
            Self::Custom(value) => formatter.write_str(value),
        }
    }
}

/// Transistor terminal vocabulary.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TransistorTerminal {
    Base,
    Collector,
    Emitter,
    Gate,
    Drain,
    Source,
    Body,
}

impl fmt::Display for TransistorTerminal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Base => "base",
            Self::Collector => "collector",
            Self::Emitter => "emitter",
            Self::Gate => "gate",
            Self::Drain => "drain",
            Self::Source => "source",
            Self::Body => "body",
        })
    }
}

impl FromStr for TransistorTerminal {
    type Err = TerminalParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(TerminalParseError::Empty);
        }

        match normalized_token(trimmed).as_str() {
            "base" => Ok(Self::Base),
            "collector" => Ok(Self::Collector),
            "emitter" => Ok(Self::Emitter),
            "gate" => Ok(Self::Gate),
            "drain" => Ok(Self::Drain),
            "source" => Ok(Self::Source),
            "body" => Ok(Self::Body),
            _ => Err(TerminalParseError::Unknown),
        }
    }
}

/// Errors returned while parsing transistor terminals.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalParseError {
    /// The terminal was empty after trimming whitespace.
    Empty,
    /// The terminal was not part of the fixed vocabulary.
    Unknown,
}

impl fmt::Display for TerminalParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("transistor terminal cannot be empty"),
            Self::Unknown => formatter.write_str("unknown transistor terminal"),
        }
    }
}

impl Error for TerminalParseError {}

/// A descriptive transistor specification.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TransistorSpec {
    kind: TransistorKind,
}

impl TransistorSpec {
    /// Creates a transistor spec from a transistor kind.
    #[must_use]
    pub const fn new(kind: TransistorKind) -> Self {
        Self { kind }
    }

    /// Returns the transistor kind.
    #[must_use]
    pub fn kind(&self) -> TransistorKind {
        self.kind.clone()
    }
}

fn normalized_token(value: &str) -> String {
    value.trim().to_ascii_lowercase().replace(['_', ' '], "-")
}

#[cfg(test)]
mod tests {
    use super::{BjtKind, FetKind, TransistorKind, TransistorSpec, TransistorTerminal};

    #[test]
    fn displays_and_parses_bjt_kinds() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!("npn".parse::<BjtKind>()?, BjtKind::Npn);
        assert_eq!(BjtKind::Pnp.to_string(), "pnp");
        Ok(())
    }

    #[test]
    fn displays_and_parses_fet_kinds() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!("jfet n".parse::<FetKind>()?, FetKind::JfetN);
        assert_eq!(FetKind::Pmos.to_string(), "pmos");
        Ok(())
    }

    #[test]
    fn displays_and_parses_transistor_terminals() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            "collector".parse::<TransistorTerminal>()?,
            TransistorTerminal::Collector
        );
        assert_eq!(TransistorTerminal::Gate.to_string(), "gate");
        Ok(())
    }

    #[test]
    fn creates_transistor_specs() {
        let spec = TransistorSpec::new(TransistorKind::Fet(FetKind::Nmos));

        assert_eq!(spec.kind(), TransistorKind::Fet(FetKind::Nmos));
    }
}
