# PURUDA

**PU**re **RU**st **DA**taframe

Compile-time type-safe DataFrame library for Rust. Provides `Col1`–`Col32` generic structs with zero-cost column access, numeric statistics, filtering, sorting, and CSV/JSON I/O.

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
puruda = "0.2"
```

## Example

```rust
use puruda::*;

fn main() {
    let x = vec![1, 2, 3];
    let y = vec![4.0f64, 5.0, 6.0];
    let z: Vec<String> = vec!["a", "b", "c"]
        .into_iter().map(|s| s.to_string()).collect();

    let mut df = Col3::from_cols(x, y, z);
    df.set_header(vec!["x", "y", "z"]);

    // Pretty-print table
    println!("{}", df);
    //  x  y  z
    //  -  -  -
    //  1  4  a
    //  2  5  b
    //  3  6  c

    // Shape
    println!("{:?}", df.shape()); // (3, 3)

    // Numeric statistics
    println!("sum: {}", df.c1().sum());       // 6
    println!("mean: {}", df.c2().mean());     // 5.0
    println!("std: {}", df.c2().std_dev());   // 0.8165...

    // Slicing
    let top2 = df.head(2);
    let bottom2 = df.tail(2);

    // Filter rows
    let filtered = df.filter(|i| *df.c1().idx(i) > 1);

    // Sort by column
    let sorted = df.sort_by_c1();

    // Add a row
    df.push_row(4, 7.0, "d".to_string());

    // Transform a column in-place
    df.c2_mut().apply(|v| *v *= 2.0);

    // CSV I/O
    df.write_csv("data.csv", ',').unwrap();
    let df2 = Col3::<Vec<i32>, Vec<f64>, Vec<String>>::read_csv("data.csv", ',').unwrap();

    // JSON I/O
    df.write_json("data.json").unwrap();
    let df3 = Col3::<Vec<i32>, Vec<f64>, Vec<String>>::read_json("data.json").unwrap();
}
```

## API Overview

### Column Access & Shape

| Method | Description |
|--------|-------------|
| `c1()` .. `cN()` | Immutable reference to column |
| `c1_mut()` .. `cN_mut()` | Mutable reference to column |
| `nrows()` | Number of rows |
| `ncols()` | Number of columns |
| `shape()` | `(nrows, ncols)` tuple |
| `len()` | Alias for `nrows()` |
| `is_empty()` | `true` if no rows |

### Slicing

| Method | Description |
|--------|-------------|
| `head(n)` | First `n` rows |
| `tail(n)` | Last `n` rows |
| `slice(start, end)` | Rows in `start..end` |

### Row Operations

| Method | Description |
|--------|-------------|
| `push_row(v1, .., vN)` | Append a single row |
| `filter(predicate)` | Keep rows where `predicate(row_index)` is true |
| `append(&other)` | In-place vertical concatenation |
| `concat(other)` | Returns a new concatenated DataFrame |
| `reindex(&[usize])` | Rearrange rows by index array |

### Sorting

| Method | Description |
|--------|-------------|
| `sort_by_c1()` .. `sort_by_cN()` | Sort all rows by column (ascending, requires `Ord`) |

### Column-Level Traits

**`Numeric`** (for `Vec<i32>`, `Vec<f64>`, etc.)

| Method | Description |
|--------|-------------|
| `sum()` | Sum of elements |
| `mean()` | Arithmetic mean (`f64`) |
| `min_val()` | Minimum value reference |
| `max_val()` | Maximum value reference |
| `var()` | Population variance (`f64`) |
| `std_dev()` | Population standard deviation (`f64`) |

**`ColumnUnique`** (for types implementing `Eq + Hash`)

| Method | Description |
|--------|-------------|
| `unique()` | Deduplicated values (preserving order) |
| `n_unique()` | Count of unique values |

**`ColumnApply`**

| Method | Description |
|--------|-------------|
| `apply(f)` | Mutate each element in-place |

**`ColumnDisplay`**

| Method | Description |
|--------|-------------|
| `print()` | Print column as `[v1, v2, ...]` |

### Utility

| Function | Description |
|----------|-------------|
| `map_column(col, f)` | Map column to a new `Vec<U>` (type conversion) |
| `describe()` | Print summary table (name, count per column) |

### I/O

| Trait | Methods | Format |
|-------|---------|--------|
| `CSV` | `write_csv(path, delimiter)`, `read_csv(path, delimiter)` | CSV |
| `JsonIO` | `write_json(path)`, `read_json(path)`, `to_json_string()`, `from_json_string(s)` | JSON |

JSON format:

```json
{
  "headers": ["x", "y"],
  "data": {
    "x": [1, 2, 3],
    "y": [4.0, 5.0, 6.0]
  }
}
```

## Supported Column Types

`Vec<bool>`, `Vec<u32>`, `Vec<u64>`, `Vec<usize>`, `Vec<i32>`, `Vec<i64>`, `Vec<isize>`, `Vec<f32>`, `Vec<f64>`, `Vec<String>`, `Vec<&str>`

Any type implementing the `Column` trait can be used as a column.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
