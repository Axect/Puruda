# PURUDA

**PU**re **RU**st **DA**taframe

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
    (*c) = vec![4, 5, 6];

    assert_eq!(c2.c1(), &vec![4, 5, 6]);

    c2.write_csv("hello.csv", ',').expect("Can't write csv");
}
```

## Congruous Data Format

* [x] CSV Trait
    * [x] `write_csv`
    * [x] `read_csv`
* [ ] HDF5
* [ ] JSON
