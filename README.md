# libhexers - iterate over hex encoded bytes

This library provides utilities for automatically converting a byte
sequence to a hex encoded one via an iterator.

## Example

```rust
use hexers::hexes;

let bytes = &[0xbe_u8, 0xef_u8];
let mut it = hexes(bytes);

assert_eq!(it.next(), Some('b'));
assert_eq!(it.next(), Some('e'));
assert_eq!(it.next(), Some('e'));
assert_eq!(it.next(), Some('f'));
```
