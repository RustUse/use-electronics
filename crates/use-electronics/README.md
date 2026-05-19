# use-electronics

Facade crate for `RustUse` practical electronics primitives.

`use-electronics` is a thin re-export layer over focused child crates for components, pins, packages, circuits, net labels, ratings, common parts, and board-level relationships. Most implementation lives in the focused crates.

This crate is not an electronics framework, circuit simulator, SPICE replacement, EDA system, PCB layout library, schematic editor, firmware framework, embedded HAL replacement, hardware abstraction layer, component inventory system, or BOM manager.

## Example

```rust
use use_electronics::{board, circuit, component, net_label, pin, rating, resistor};

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

assert_eq!(spec.resistance().ohms(), 10_000.0);
assert_eq!(pin_two.pin().to_string(), "2");
assert_eq!(connection.target().to_string(), "net:SENSE");
assert_eq!(layer.to_string(), "top-copper");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Modules

- `component` re-exports `use-component`
- `pin` re-exports `use-pin`
- `package` re-exports `use-package`
- `circuit` re-exports `use-circuit`
- `net_label` re-exports `use-net-label`
- `rating` re-exports `use-rating`
- `resistor` re-exports `use-resistor`
- `capacitor` re-exports `use-capacitor`
- `diode` re-exports `use-diode`
- `transistor` re-exports `use-transistor`
- `board` re-exports `use-board`

## Scope

Use the facade when one dependency and one import surface are useful. Use focused crates directly when a library only needs one primitive area. Physical electrical formulas belong in `use-physics/use-electricity`; `use-electronics` stays focused on practical electronics vocabulary.
