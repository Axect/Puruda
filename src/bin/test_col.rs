extern crate peroxide;
extern crate purude;
use purude::*;
use peroxide::*;

fn main() {
    let a = vec![1, 2, 3];
    let b = vec!["a", "b", "c"];

    let c2 = Col2::from_cols(a, b);
    c2.c1().print();
    c2.c2().print();
}