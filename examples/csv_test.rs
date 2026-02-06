extern crate puruda;
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

    println!("{}", c2);

    c2.write_csv("example_data/test.csv", ',').expect("Can't write csv");

    let c2_new =
        Col2::<Vec<String>, Vec<String>>::read_csv("example_data/test.csv", ',').expect("Can't read csv");
    println!("\nRead back from CSV:");
    println!("{}", c2_new);
}
