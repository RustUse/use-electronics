# use-circuit

Primitive circuit relationship vocabulary.

`use-circuit` describes circuit IDs, names, nodes, nets, terminals, and connections. A terminal refers to a component pin, and a connection ties a terminal to a net or node. This crate does not simulate circuits, solve equations, apply Kirchhoff laws, render schematics, calculate resistance or capacitance, or autoroute anything.

## Example

```rust
use use_circuit::{Connection, NetId, Terminal};
use use_component::ReferenceDesignator;
use use_pin::{PinNumber, PinRef};

let pin = PinRef::numbered(ReferenceDesignator::new("R1")?, PinNumber::new(1)?);
let connection = Connection::to_net(Terminal::from_pin_ref(pin), NetId::new("SENSE")?);

assert_eq!(connection.target().to_string(), "net:SENSE");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for descriptive, graph-like circuit relationships. Circuit solving, schematic rendering, and design automation are out of scope.
