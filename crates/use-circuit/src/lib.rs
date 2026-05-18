#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::{fmt, str::FromStr};
use std::error::Error;

use use_component::ReferenceDesignator;
use use_pin::{PinIdentifier, PinRef};

/// Commonly used circuit primitives.
pub mod prelude {
    pub use crate::{
        CircuitId, CircuitName, CircuitTextError, Connection, ConnectionTarget, NetId, NodeId,
        Terminal,
    };
}

/// Errors returned by non-empty circuit text wrappers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CircuitTextError {
    /// The text was empty after trimming whitespace.
    Empty,
}

impl fmt::Display for CircuitTextError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("circuit text cannot be empty"),
        }
    }
}

impl Error for CircuitTextError {}

/// A stable circuit identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CircuitId(String);

impl CircuitId {
    /// Creates a circuit ID from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`CircuitTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CircuitTextError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the ID text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CircuitId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CircuitId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CircuitId {
    type Err = CircuitTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A human-readable circuit name.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CircuitName(String);

impl CircuitName {
    /// Creates a circuit name from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`CircuitTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CircuitTextError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the name text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for CircuitName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for CircuitName {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CircuitName {
    type Err = CircuitTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A circuit node identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeId(String);

impl NodeId {
    /// Creates a node ID from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`CircuitTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CircuitTextError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the node ID text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for NodeId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for NodeId {
    type Err = CircuitTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A net identifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NetId(String);

impl NetId {
    /// Creates a net ID from non-empty text.
    ///
    /// # Errors
    ///
    /// Returns [`CircuitTextError::Empty`] when the trimmed value is empty.
    pub fn new(value: impl AsRef<str>) -> Result<Self, CircuitTextError> {
        non_empty_text(value).map(Self)
    }

    /// Returns the net ID text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for NetId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for NetId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for NetId {
    type Err = CircuitTextError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

/// A component terminal represented as a component pin reference.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Terminal {
    pin_ref: PinRef,
}

impl Terminal {
    /// Creates a terminal from a pin reference.
    #[must_use]
    pub const fn from_pin_ref(pin_ref: PinRef) -> Self {
        Self { pin_ref }
    }

    /// Creates a terminal from a component and pin identifier.
    #[must_use]
    pub const fn new(component: ReferenceDesignator, pin: PinIdentifier) -> Self {
        Self::from_pin_ref(PinRef::new(component, pin))
    }

    /// Returns the underlying pin reference.
    #[must_use]
    pub const fn pin_ref(&self) -> &PinRef {
        &self.pin_ref
    }
}

impl fmt::Display for Terminal {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.pin_ref.fmt(formatter)
    }
}

/// The graph target for a terminal connection.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ConnectionTarget {
    Net(NetId),
    Node(NodeId),
}

impl fmt::Display for ConnectionTarget {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Net(net) => write!(formatter, "net:{net}"),
            Self::Node(node) => write!(formatter, "node:{node}"),
        }
    }
}

/// A descriptive connection from a terminal to a net or node.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Connection {
    terminal: Terminal,
    target: ConnectionTarget,
}

impl Connection {
    /// Creates a connection from a terminal to a target.
    #[must_use]
    pub const fn new(terminal: Terminal, target: ConnectionTarget) -> Self {
        Self { terminal, target }
    }

    /// Creates a terminal-to-net connection.
    #[must_use]
    pub const fn to_net(terminal: Terminal, net: NetId) -> Self {
        Self::new(terminal, ConnectionTarget::Net(net))
    }

    /// Creates a terminal-to-node connection.
    #[must_use]
    pub const fn to_node(terminal: Terminal, node: NodeId) -> Self {
        Self::new(terminal, ConnectionTarget::Node(node))
    }

    /// Returns the connected terminal.
    #[must_use]
    pub const fn terminal(&self) -> &Terminal {
        &self.terminal
    }

    /// Returns the connection target.
    #[must_use]
    pub const fn target(&self) -> &ConnectionTarget {
        &self.target
    }
}

fn non_empty_text(value: impl AsRef<str>) -> Result<String, CircuitTextError> {
    let trimmed = value.as_ref().trim();
    if trimmed.is_empty() {
        Err(CircuitTextError::Empty)
    } else {
        Ok(trimmed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{CircuitName, CircuitTextError, Connection, NetId, Terminal};
    use use_component::ReferenceDesignator;
    use use_pin::{PinNumber, PinRef};

    #[test]
    fn accepts_valid_circuit_names() -> Result<(), CircuitTextError> {
        let name = CircuitName::new("input filter")?;

        assert_eq!(name.as_str(), "input filter");
        Ok(())
    }

    #[test]
    fn rejects_empty_circuit_names() {
        assert_eq!(CircuitName::new(" "), Err(CircuitTextError::Empty));
    }

    #[test]
    fn creates_terminals() -> Result<(), Box<dyn std::error::Error>> {
        let pin = PinRef::numbered(ReferenceDesignator::new("R1")?, PinNumber::new(1)?);
        let terminal = Terminal::from_pin_ref(pin);

        assert_eq!(terminal.to_string(), "R1:1");
        Ok(())
    }

    #[test]
    fn creates_connections() -> Result<(), Box<dyn std::error::Error>> {
        let pin = PinRef::numbered(ReferenceDesignator::new("R1")?, PinNumber::new(1)?);
        let connection = Connection::to_net(Terminal::from_pin_ref(pin), NetId::new("SENSE")?);

        assert_eq!(connection.target().to_string(), "net:SENSE");
        Ok(())
    }

    #[test]
    fn sorts_connections_deterministically() -> Result<(), Box<dyn std::error::Error>> {
        let connections = BTreeSet::from([
            Connection::to_net(
                Terminal::from_pin_ref(PinRef::numbered(
                    ReferenceDesignator::new("R2")?,
                    PinNumber::new(1)?,
                )),
                NetId::new("B")?,
            ),
            Connection::to_net(
                Terminal::from_pin_ref(PinRef::numbered(
                    ReferenceDesignator::new("R1")?,
                    PinNumber::new(1)?,
                )),
                NetId::new("A")?,
            ),
        ]);
        let ordered: Vec<_> = connections
            .iter()
            .map(|connection| connection.terminal().to_string())
            .collect();

        assert_eq!(ordered, vec!["R1:1", "R2:1"]);
        Ok(())
    }
}
