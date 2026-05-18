# use-capacitor

Primitive capacitor vocabulary.

`use-capacitor` describes capacitance values, capacitor kinds, polarity, and optional voltage ratings. It stores capacitance in farads and does not simulate charging, model ESR/ESL, or solve circuits.

## Example

```rust
use use_capacitor::{CapacitanceValue, CapacitorKind, CapacitorPolarity, CapacitorSpec};
use use_rating::VoltageRating;

let spec = CapacitorSpec::new(CapacitanceValue::new_farads(0.000_001)?, CapacitorKind::Ceramic)
    .with_polarity(CapacitorPolarity::NonPolarized)
    .with_voltage_rating(VoltageRating::new_volts(16.0)?);

assert_eq!(spec.kind(), CapacitorKind::Ceramic);
assert_eq!(spec.polarity(), CapacitorPolarity::NonPolarized);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for descriptive capacitor metadata. Circuit behavior and detailed electrical models are out of scope.
