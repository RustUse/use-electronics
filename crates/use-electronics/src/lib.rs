#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

//! Thin facade for practical electronics primitive crates.

pub use use_board as board;
pub use use_capacitor as capacitor;
pub use use_circuit as circuit;
pub use use_component as component;
pub use use_diode as diode;
pub use use_net_label as net_label;
pub use use_package as package;
pub use use_pin as pin;
pub use use_rating as rating;
pub use use_resistor as resistor;
pub use use_transistor as transistor;

/// Common electronics primitive types from the focused crates.
pub mod prelude {
    pub use crate::board::{BoardLayer, BoardName, BoardSide, LayerCount};
    pub use crate::capacitor::{CapacitanceValue, CapacitorKind, CapacitorPolarity, CapacitorSpec};
    pub use crate::circuit::{Connection, NetId, NodeId, Terminal};
    pub use crate::component::{ComponentKind, ComponentValue, ReferenceDesignator};
    pub use crate::diode::{DiodeKind, DiodePolarity, DiodeSpec};
    pub use crate::net_label::{GroundKind, NetLabel, PowerRail, SignalName};
    pub use crate::package::{PackageKind, PackageName, PackagePitch, PinCount};
    pub use crate::pin::{PinName, PinNumber, PinPolarity, PinRef, PinRole};
    pub use crate::rating::{
        CurrentRating, FrequencyRating, PowerRating, TemperatureRating, Tolerance, VoltageRating,
    };
    pub use crate::resistor::{ResistanceValue, ResistorKind, ResistorSpec};
    pub use crate::transistor::{
        BjtKind, FetKind, TransistorKind, TransistorSpec, TransistorTerminal,
    };
}

#[cfg(test)]
mod tests {
    use super::{board, circuit, component, net_label, pin, rating, resistor};

    #[test]
    fn facade_exposes_composable_electronics_primitives() -> Result<(), Box<dyn std::error::Error>>
    {
        let reference = component::ReferenceDesignator::new("R1")?;
        let resistance = resistor::ResistanceValue::new_ohms(10_000.0)?;
        let tolerance = rating::Tolerance::from_percent(1.0)?;
        let spec = resistor::ResistorSpec::new(resistance, resistor::ResistorKind::Fixed)
            .with_tolerance(tolerance);
        let pin_one = pin::PinRef::numbered(reference.clone(), pin::PinNumber::new(1)?);
        let pin_two = pin::PinRef::numbered(reference, pin::PinNumber::new(2)?);
        let label = net_label::NetLabel::new("SENSE")?;
        let connection = circuit::Connection::to_net(
            circuit::Terminal::from_pin_ref(pin_one),
            circuit::NetId::new(label.as_str())?,
        );
        let layer = board::BoardLayer::TopCopper;

        assert_eq!(spec.tolerance().map(rating::Tolerance::percent), Some(1.0));
        assert_eq!(pin_two.pin().to_string(), "2");
        assert_eq!(connection.target().to_string(), "net:SENSE");
        assert_eq!(layer.to_string(), "top-copper");
        Ok(())
    }
}
