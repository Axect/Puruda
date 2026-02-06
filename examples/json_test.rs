extern crate puruda;
use puruda::*;

fn main() {
    let x = vec![1, 2, 3, 4, 5];
    let y = vec![2.0f64, 4.0, 6.0, 8.0, 10.0];
    let z: Vec<String> = vec!["a", "b", "c", "d", "e"]
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let mut df = Col3::from_cols(x, y, z);
    df.set_header(vec!["x", "y", "label"]);

    println!("=== Original DataFrame ===");
    println!("{}", df);

    // Write to JSON
    let json = df.to_json_string();
    println!("\n=== JSON output ===");
    println!("{}", json);

    // Write to file and read back
    df.write_json("example_data/test.json").expect("Can't write json");

    let df2 = Col3::<Vec<i32>, Vec<f64>, Vec<String>>::read_json("example_data/test.json")
        .expect("Can't read json");
    println!("\n=== Read back from JSON ===");
    println!("{}", df2);

    // Numeric stats
    println!("\n=== Numeric stats ===");
    println!("x sum: {}", df2.c1().sum());
    println!("y mean: {}", df2.c2().mean());
    println!("y var: {}", df2.c2().var());

    // Concat
    let extra = Col3::from_cols(vec![6, 7], vec![12.0, 14.0],
        vec!["f".to_string(), "g".to_string()]);
    let combined = df2.concat(extra);
    println!("\n=== After concat ===");
    println!("{}", combined);
}
