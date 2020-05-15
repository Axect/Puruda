extern crate peroxide;
extern crate puruda;

use peroxide::*;
use puruda::*;

fn main() {
    let word: Vec<String> = vec!["bovine", "divine", "fastidious"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();
    let mean: Vec<String> = vec!["소의", "신의", "까다로운"]
        .into_iter()
        .map(|x| x.to_string())
        .collect();

    let mut c2 = Col2::from_cols(word, mean);
    c2.set_header(vec!["word", "mean"]);
    c2.c1().print();
    c2.c2().print();

    c2.write_csv("test.csv", ',').expect("Can't write csv");

    let c2_new =
        Col2::<Vec<String>, Vec<String>>::read_csv("test.csv", ',').expect("Can't read csv");
    c2_new.c1().print();
    c2_new.c2().print();
}
