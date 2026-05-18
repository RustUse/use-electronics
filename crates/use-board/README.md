# use-board

Primitive board-level vocabulary.

`use-board` describes board IDs, names, sides, layers, layer counts, and assembly sides. It does not implement PCB layout, routing, board-file parsing, geometry calculation, or manufacturing output generation.

## Example

```rust
use use_board::{BoardLayer, BoardName, BoardSide, LayerCount};

let board = BoardName::new("sensor board")?;
let layers = LayerCount::new(4)?;
let side: BoardSide = "top".parse()?;
let layer: BoardLayer = "inner-copper-1".parse()?;

assert_eq!(board.as_str(), "sensor board");
assert_eq!(layers.get(), 4);
assert_eq!(side.to_string(), "top");
assert_eq!(layer.to_string(), "inner-copper-1");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

Use this crate for simple board vocabulary. PCB layout, routing, geometry, file parsing, and manufacturing outputs are out of scope.
