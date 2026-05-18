# use-diode

Primitive diode vocabulary.

`use-diode` describes diode kinds, anode/cathode polarity, and optional ratings. It does not simulate I/V curves, model diode equations, implement semiconductor physics, or handle LED color science.

## Example

```rust
use use_diode::{DiodeKind, DiodePolarity, DiodeSpec};
use use_rating::{CurrentRating, VoltageRating};

let spec = DiodeSpec::new(DiodeKind::Schottky)
    .with_forward_voltage(VoltageRating::new_volts(0.3)?)
    .with_current_rating(CurrentRating::new_amperes(1.0)?);

assert_eq!(spec.kind(), DiodeKind::Schottky);
assert_eq!(DiodePolarity::Cathode.to_string(), "cathode");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for descriptive diode metadata. Semiconductor equations and circuit simulation are out of scope.
