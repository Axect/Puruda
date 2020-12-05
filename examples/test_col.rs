extern crate peroxide;
extern crate puruda;
use puruda::*;
use peroxide::fuga::*;

fn main() {
    let a = vec![1, 2, 3];
    let b = vec![4f64, 5f64, 6f64];
    let c = vec!["a", "b", "c"];

    let mut col3 = Col3::from_cols(a, b, c);
    col3.set_header(vec!["a", "b", "c"]);

    println!("{:?}", col3);

    col3.c1().print();
    col3.c2().print();
    col3.c3().print();

    // col2.write_nc("test.nc").expect("Can't write nc");

    //let df = DataFrame::read_nc("test.nc").expect("Can't read nc");
    //df.print();
}
