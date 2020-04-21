# Puruda

**PU**re **RU**st **Da**taframe

## Example

```rust
extern crate puruda;
use puruda::*;

fn main() {
    let a = vec![1, 2, 3];
    let b = vec!["a", "b", "c"];

    let mut c2 = Col2::from_cols(a, b);
    c2.c1().print();
    c2.c2().print();

    let c = c2.c1_mut();
    *c = vec![4, 5, 6];

    c2.c1().print();
}
```