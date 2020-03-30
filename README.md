# stl_rotate

C++ stl::rotate implementation in Rust (uses range instead of first, .., last).

Selects elements from a vector using a range and rotates the selected elements such that the chosen index becomes the first index in that range, with all prior elements being pushed back to the end of the range.

## Example

```rust
// [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
let mut v: Vec<usize> = (0..10).collect();

// Rotate all elements with indices 0 to 9 such that index 5 becomes first element:
v.rotate(0..10, 5);

// [5, 6, 7, 8, 9, 0, 1, 2, 3, 4]
assert!(v == [5, 6, 7, 8, 9, 0, 1, 2, 3, 4]);
```
