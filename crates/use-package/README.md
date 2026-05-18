# use-package

Primitive electronic package vocabulary.

`use-package` describes names such as `DIP-8`, `SOIC-16`, `QFN-32`, `BGA-256`, `TO-220`, `SOT-23`, `0603`, and `0805`. It does not model exact footprint geometry, parse EDA footprint files, or implement PCB layout behavior.

## Example

```rust
use use_package::{PackageKind, PackageName, PinCount};

let name = PackageName::new("SOIC-16")?;
let count = PinCount::new(16)?;
let kind: PackageKind = "soic".parse()?;

assert_eq!(name.as_str(), "SOIC-16");
assert_eq!(count.get(), 16);
assert_eq!(kind.to_string(), "soic");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for compact package metadata. Full geometry, EDA file formats, and PCB layout behavior are out of scope.
