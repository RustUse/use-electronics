# use-net-label

Primitive electrical net labeling vocabulary.

`use-net-label` stores labels such as `GND`, `AGND`, `DGND`, `VCC`, `3V3`, `5V`, `VIN`, `SDA`, `SCL`, `MISO`, `MOSI`, `CLK`, and `RESET`. It preserves label text and does not infer electrical behavior, assign voltages, or parse complete schematic netlists.

## Example

```rust
use use_net_label::{GroundKind, NetLabel, SignalName};

let ground = NetLabel::new("GND")?;
let signal = SignalName::new("SDA")?;

assert!(ground.is_ground());
assert_eq!(signal.as_str(), "SDA");
assert_eq!("analog-ground".parse::<GroundKind>()?, GroundKind::AnalogGround);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for descriptive net labels and signal names. Circuit netlist parsing, voltage assignment, and electrical inference are out of scope.
