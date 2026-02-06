use puruda::*;
use std::fs;

fn test_dir() -> String {
    let dir = "target/test_data".to_string();
    fs::create_dir_all(&dir).unwrap();
    dir
}

// =============================================================================
// CSV
// =============================================================================
#[test]
fn csv_write_and_read() {
    let dir = test_dir();
    let path = format!("{}/csv_basic.csv", dir);

    let mut df = Col3::from_cols(
        vec![1i32, 2, 3],
        vec![1.5f64, 2.5, 3.5],
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
    );
    df.set_header(vec!["x", "y", "z"]);
    df.write_csv(&path, ',').unwrap();

    let df2 = Col3::<Vec<i32>, Vec<f64>, Vec<String>>::read_csv(&path, ',').unwrap();
    assert_eq!(df2.nrows(), 3);
    assert_eq!(df2.c1().to_vec(), &vec![1, 2, 3]);
    assert_eq!(df2.c2().to_vec(), &vec![1.5, 2.5, 3.5]);
    assert_eq!(df2.c3().to_vec(), &vec!["a".to_string(), "b".to_string(), "c".to_string()]);
}

#[test]
fn csv_tab_delimiter() {
    let dir = test_dir();
    let path = format!("{}/csv_tab.tsv", dir);

    let mut df = Col2::from_cols(vec![10, 20], vec![30, 40]);
    df.set_header(vec!["a", "b"]);
    df.write_csv(&path, '\t').unwrap();

    let df2 = Col2::<Vec<i32>, Vec<i32>>::read_csv(&path, '\t').unwrap();
    assert_eq!(df2.c1().to_vec(), &vec![10, 20]);
    assert_eq!(df2.c2().to_vec(), &vec![30, 40]);
}

#[test]
fn csv_roundtrip_preserves_data() {
    let dir = test_dir();
    let path = format!("{}/csv_roundtrip.csv", dir);

    let mut df = Col2::from_cols(
        vec![-1i32, 0, 999],
        vec![0.001f64, -3.14, 1e10],
    );
    df.set_header(vec!["int_col", "float_col"]);
    df.write_csv(&path, ',').unwrap();

    let df2 = Col2::<Vec<i32>, Vec<f64>>::read_csv(&path, ',').unwrap();
    assert_eq!(df2.c1().to_vec(), df.c1().to_vec());
    for i in 0..3 {
        assert!((df2.c2().idx(i) - df.c2().idx(i)).abs() < 1e-10);
    }
}

// =============================================================================
// JSON
// =============================================================================
#[test]
fn json_write_and_read() {
    let dir = test_dir();
    let path = format!("{}/json_basic.json", dir);

    let mut df = Col2::from_cols(vec![1, 2, 3], vec![4.0, 5.0, 6.0]);
    df.set_header(vec!["x", "y"]);
    df.write_json(&path).unwrap();

    let df2 = Col2::<Vec<i32>, Vec<f64>>::read_json(&path).unwrap();
    assert_eq!(df2.nrows(), 3);
    assert_eq!(df2.c1().to_vec(), &vec![1, 2, 3]);
    assert_eq!(df2.c2().to_vec(), &vec![4.0, 5.0, 6.0]);
}

#[test]
fn json_string_columns() {
    let dir = test_dir();
    let path = format!("{}/json_strings.json", dir);

    let mut df = Col2::from_cols(
        vec!["hello".to_string(), "world".to_string()],
        vec![10, 20],
    );
    df.set_header(vec!["word", "num"]);
    df.write_json(&path).unwrap();

    let df2 = Col2::<Vec<String>, Vec<i32>>::read_json(&path).unwrap();
    assert_eq!(df2.c1().to_vec(), &vec!["hello".to_string(), "world".to_string()]);
    assert_eq!(df2.c2().to_vec(), &vec![10, 20]);
}

#[test]
fn json_to_string_format() {
    let mut df = Col2::from_cols(vec![1, 2], vec![3.0, 4.0]);
    df.set_header(vec!["a", "b"]);
    let json = df.to_json_string();

    assert!(json.contains("\"headers\""));
    assert!(json.contains("\"data\""));
    assert!(json.contains("\"a\""));
    assert!(json.contains("\"b\""));
}

#[test]
fn json_from_string_roundtrip() {
    let mut df = Col2::from_cols(vec![10, 20, 30], vec![1.1, 2.2, 3.3]);
    df.set_header(vec!["x", "y"]);
    let json = df.to_json_string();

    let df2 = Col2::<Vec<i32>, Vec<f64>>::from_json_string(&json).unwrap();
    assert_eq!(df2.c1().to_vec(), df.c1().to_vec());
    assert_eq!(df2.c2().to_vec(), df.c2().to_vec());
    assert_eq!(df2.header(), df.header());
}

#[test]
fn json_empty_df() {
    let mut df = Col2::<Vec<i32>, Vec<i32>>::new();
    df.set_header(vec!["a", "b"]);
    let json = df.to_json_string();

    let df2 = Col2::<Vec<i32>, Vec<i32>>::from_json_string(&json).unwrap();
    assert_eq!(df2.nrows(), 0);
}
