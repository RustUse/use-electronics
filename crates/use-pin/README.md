# use-pin

Primitive electronic pin vocabulary.

`use-pin` describes pin numbers, pin names, roles, polarity, and references to component pins. It does not implement electrical simulation, embedded HAL traits, MCU pin configuration, or GPIO control.

## Example

```rust
use use_component::ReferenceDesignator;
use use_pin::{PinName, PinNumber, PinRef, PinRole};

let component = ReferenceDesignator::new("U2")?;
let numbered = PinRef::numbered(component.clone(), PinNumber::new(1)?);
let named = PinRef::named(component, PinName::new("VCC")?);

assert_eq!(numbered.to_string(), "U2:1");
assert_eq!(named.pin().to_string(), "VCC");
assert_eq!("power".parse::<PinRole>()?, PinRole::Power);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for descriptive pin metadata only. Use embedded, HAL, firmware, or simulation crates for behavior.
