# use-rating

Primitive electronic rating vocabulary.

`use-rating` keeps units simple for v0: volts, amperes, watts, percent, Celsius, and hertz are documented numeric wrappers. It does not duplicate `use-units`, implement electrical simulation, calculate derating, or model safety certification.

## Example

```rust
use use_rating::{PowerRating, Tolerance, VoltageRating};

let voltage = VoltageRating::new_volts(16.0)?;
let power = PowerRating::new_watts(0.25)?;
let tolerance = Tolerance::from_percent(1.0)?;

assert_eq!(voltage.to_string(), "16 V");
assert_eq!(power.to_string(), "0.25 W");
assert_eq!(tolerance.to_string(), "+/-1%");
# Ok::<(), use_rating::RatingError>(())
```

## Scope

Use this crate for descriptive rating metadata. Broader unit systems, electrical formulas, derating calculators, and safety logic are out of scope.
