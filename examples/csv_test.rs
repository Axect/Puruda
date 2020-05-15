extern crate puruda;

use puruda::*;

fn main() {
    let word = vec!["bovine", "divine", "fastidious"];
    let mean = vec!["소의", "신의", "까다로운"];

    let mut c2 = Col2::from_cols(word, mean);
    c2.set_header(vec!["word", "mean"]);
    c2.c1().print();
    c2.c2().print();

    c2.write_csv("test.csv", ',').expect("Can't write csv");
}
