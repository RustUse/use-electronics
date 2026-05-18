# use-resistor

Primitive resistor vocabulary.

`use-resistor` describes resistor values, kinds, and optional ratings. It stores resistance in ohms and does not calculate circuit behavior, decode color bands, or duplicate a broad unit system.

## Example

```rust
use use_rating::{PowerRating, Tolerance};
use use_resistor::{ResistanceValue, ResistorKind, ResistorSpec};

let spec = ResistorSpec::new(ResistanceValue::new_ohms(10_000.0)?, ResistorKind::Fixed)
    .with_tolerance(Tolerance::from_percent(1.0)?)
    .with_power_rating(PowerRating::new_watts(0.25)?);

assert_eq!(spec.kind(), ResistorKind::Fixed);
assert_eq!(spec.tolerance().map(Tolerance::percent), Some(1.0));
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for descriptive resistor metadata. Circuit solving, color-code decoding, and simulation are out of scope.
