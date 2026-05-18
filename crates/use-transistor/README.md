# use-transistor

Primitive transistor vocabulary.

`use-transistor` describes BJT/FET kinds, transistor terminal names, and simple specs. It does not simulate transistor behavior, calculate gain, model switching behavior, or implement semiconductor physics.

## Example

```rust
use use_transistor::{BjtKind, TransistorKind, TransistorSpec, TransistorTerminal};

let spec = TransistorSpec::new(TransistorKind::Bjt(BjtKind::Npn));

assert_eq!(spec.kind(), TransistorKind::Bjt(BjtKind::Npn));
assert_eq!("collector".parse::<TransistorTerminal>()?, TransistorTerminal::Collector);
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for descriptive transistor metadata. Behavior models and semiconductor calculations are out of scope.
