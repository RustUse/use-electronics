# use-electronics

RustUse is "Composable sets of primitive Rust utility crates for fellow crustaceans."

`use-electronics` is a primitive electronics vocabulary set. It provides small, composable Rust primitives for describing electronic components, pins, packages, circuits, nets, ratings, common parts, and board-level relationships.

This set is not a circuit simulator, SPICE replacement, EDA tool, PCB layout engine, schematic editor, firmware framework, embedded HAL replacement, hardware abstraction layer, component inventory system, or BOM manager.

## Boundary

`use-physics/use-electricity` owns physical electrical concepts such as charge, current, voltage, resistance, power, and field-level theory.

`use-electronics` owns practical electronics vocabulary such as components, pins, packages, circuits, nets, ratings, and board-level relationships. It describes electronics concepts; it does not solve or execute them.

## Crates

| Crate             | Purpose                                                                |
| ----------------- | ---------------------------------------------------------------------- |
| `use-electronics` | Thin facade over the focused electronics crates.                       |
| `use-component`   | Component identity, reference designators, values, and classification. |
| `use-pin`         | Pin numbers, names, roles, polarity, and pin references.               |
| `use-package`     | Package names, package families, pin counts, and pitch labels.         |
| `use-circuit`     | Circuit, terminal, connection, node, and net relationship vocabulary.  |
| `use-net-label`   | Net labels, signal names, power rails, and ground kinds.               |
| `use-rating`      | Simple documented numeric wrappers for ratings.                        |
| `use-resistor`    | Resistor values, kinds, and descriptive specs.                         |
| `use-capacitor`   | Capacitor values, kinds, polarity, and descriptive specs.              |
| `use-diode`       | Diode kind, polarity, and descriptive specs.                           |
| `use-transistor`  | BJT/FET kind, transistor terminals, and descriptive specs.             |
| `use-board`       | Board names, sides, layers, layer counts, and assembly sides.          |

## Example

```rust
use use_electronics::{board, circuit, component, net_label, pin, rating, resistor};

let reference = component::ReferenceDesignator::new("R1")?;
let resistance = resistor::ResistanceValue::new_ohms(10_000.0)?;
let tolerance = rating::Tolerance::from_percent(1.0)?;
let resistor = resistor::ResistorSpec::new(resistance, resistor::ResistorKind::Fixed)
    .with_tolerance(tolerance);

let pin_one = pin::PinRef::numbered(reference.clone(), pin::PinNumber::new(1)?);
let pin_two = pin::PinRef::numbered(reference, pin::PinNumber::new(2)?);
let net = net_label::NetLabel::new("SENSE")?;
let connection = circuit::Connection::to_net(
    circuit::Terminal::from_pin_ref(pin_one),
    circuit::NetId::new(net.as_str())?,
);
let layer = board::BoardLayer::TopCopper;

assert_eq!(resistor.resistance().ohms(), 10_000.0);
assert_eq!(pin_two.pin().to_string(), "2");
assert_eq!(connection.target().to_string(), "net:SENSE");
assert_eq!(layer.to_string(), "top-copper");
# Ok::<(), Box<dyn std::error::Error>>(())
```

The example composes primitives that downstream crates can store, compare, serialize, or transform. It does not simulate circuit behavior or generate design files.

## Related Sets

- `use-physics`
- `use-units`
- `use-measure`
- `use-materials`
- `use-signal`
- `use-data`
- `use-validate`

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0, in `LICENSE-APACHE`
- MIT license, in `LICENSE-MIT`
