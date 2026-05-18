# use-component

Primitive electronic component identity and classification.

`use-component` stores labels such as `R1`, `C4`, `U2`, `D3`, `J1`, and `SW1` without turning them into schematic, BOM, inventory, or component database records.

## Example

```rust
use use_component::{ComponentKind, ReferenceDesignator};

let reference = ReferenceDesignator::new("R1")?;
let kind: ComponentKind = "resistor".parse()?;

assert_eq!(reference.as_str(), "R1");
assert_eq!(kind.to_string(), "resistor");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for small component labels, values, and classification. It does not model schematics, BOMs, inventory, or component databases.
