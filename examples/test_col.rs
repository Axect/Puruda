extern crate peroxide;
extern crate puruda;
use puruda::*;
use peroxide::*;

fn main() {
    let a = vec![1, 2, 3];
    let b = vec![4f64, 5f64, 6f64];

    let mut col2 = Col2::from_cols(a, b);
    col2.set_header(vec!["a", "b"]);

    // col2.write_nc("test.nc").expect("Can't write nc");

    let df = DataFrame::read_nc("test.nc").expect("Can't read nc");
    df.print();
}