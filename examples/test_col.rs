extern crate puruda;
use puruda::*;

fn main() {
    let a = vec![1, 2, 3];
    let b = vec![4f64, 5f64, 6f64];
    let c: Vec<String> = vec!["a", "b", "c"].into_iter().map(|s| s.to_string()).collect();

    let mut col3 = Col3::from_cols(a, b, c);
    col3.set_header(vec!["x", "y", "z"]);

    // Display
    println!("{}", col3);

    // Shape
    println!("shape: {:?}", col3.shape());
    println!("nrows: {}, ncols: {}", col3.nrows(), col3.ncols());

    // Head / Tail
    println!("\n--- head(2) ---");
    println!("{}", col3.head(2));

    println!("\n--- tail(2) ---");
    println!("{}", col3.tail(2));

    // Slice
    println!("\n--- slice(1, 3) ---");
    println!("{}", col3.slice(1, 3));

    // Numeric operations on column
    println!("\nsum of c1: {}", col3.c1().sum());
    println!("mean of c2: {}", col3.c2().mean());
    println!("std_dev of c2: {}", col3.c2().std_dev());

    // Unique
    println!("unique c1: {:?}", col3.c1().unique());

    // Filter
    let filtered = col3.filter(|i| *col3.c1().idx(i) > 1);
    println!("\n--- filter(c1 > 1) ---");
    println!("{}", filtered);

    // Sort
    let sorted = col3.sort_by_c1();
    println!("\n--- sort by c1 ---");
    println!("{}", sorted);

    // Push row
    col3.push_row(4, 7.0, "d".to_string());
    println!("\n--- after push_row ---");
    println!("{}", col3);

    // Describe
    println!("\n--- describe ---");
    col3.describe();

    // Apply
    col3.c2_mut().apply(|x| *x *= 2.0);
    println!("\n--- after apply(c2 * 2) ---");
    println!("{}", col3);

    // map_column
    let doubled: Vec<i32> = map_column(col3.c1(), |x| x * 10);
    println!("c1 * 10: {:?}", doubled);
}
