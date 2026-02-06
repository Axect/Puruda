use puruda::*;

// =============================================================================
// reindex
// =============================================================================
#[test]
fn reindex_reverses() {
    let df = Col2::from_cols(vec![10, 20, 30], vec![1.0, 2.0, 3.0]);
    let reversed = df.reindex(&[2, 1, 0]);
    assert_eq!(reversed.c1().to_vec(), &vec![30, 20, 10]);
    assert_eq!(reversed.c2().to_vec(), &vec![3.0, 2.0, 1.0]);
}

#[test]
fn reindex_duplicates() {
    let df = Col2::from_cols(vec![10, 20, 30], vec![1.0, 2.0, 3.0]);
    let dup = df.reindex(&[0, 0, 2, 2]);
    assert_eq!(dup.nrows(), 4);
    assert_eq!(dup.c1().to_vec(), &vec![10, 10, 30, 30]);
}

#[test]
fn reindex_empty() {
    let df = Col2::from_cols(vec![1, 2, 3], vec![4, 5, 6]);
    let empty = df.reindex(&[]);
    assert_eq!(empty.nrows(), 0);
}

// =============================================================================
// sort_by_cN
// =============================================================================
#[test]
fn sort_by_c1_ascending() {
    let df = Col2::from_cols(vec![30, 10, 20], vec![3.0, 1.0, 2.0]);
    let sorted = df.sort_by_c1();
    assert_eq!(sorted.c1().to_vec(), &vec![10, 20, 30]);
    assert_eq!(sorted.c2().to_vec(), &vec![1.0, 2.0, 3.0]);
}

#[test]
fn sort_by_c2_with_strings() {
    let df = Col2::from_cols(
        vec![3, 1, 2],
        vec!["c".to_string(), "a".to_string(), "b".to_string()],
    );
    let sorted = df.sort_by_c2();
    assert_eq!(sorted.c2().to_vec(), &vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    assert_eq!(sorted.c1().to_vec(), &vec![1, 2, 3]);
}

#[test]
fn sort_preserves_header() {
    let mut df = Col2::from_cols(vec![2, 1], vec![20, 10]);
    df.set_header(vec!["a", "b"]);
    let sorted = df.sort_by_c1();
    assert_eq!(sorted.header(), df.header());
}

#[test]
fn sort_already_sorted() {
    let df = Col2::from_cols(vec![1, 2, 3], vec![10, 20, 30]);
    let sorted = df.sort_by_c1();
    assert_eq!(sorted.c1().to_vec(), &vec![1, 2, 3]);
}

// =============================================================================
// apply
// =============================================================================
#[test]
fn apply_doubles_values() {
    let mut df = Col2::from_cols(vec![1, 2, 3], vec![10.0, 20.0, 30.0]);
    df.c1_mut().apply(|x| *x *= 2);
    assert_eq!(df.c1().to_vec(), &vec![2, 4, 6]);
}

#[test]
fn apply_on_floats() {
    let mut df = Col1::from_cols(vec![1.0f64, 4.0, 9.0]);
    df.c1_mut().apply(|x: &mut f64| *x = x.sqrt());
    assert!((*df.c1().idx(0) - 1.0f64).abs() < 1e-10);
    assert!((*df.c1().idx(1) - 2.0f64).abs() < 1e-10);
    assert!((*df.c1().idx(2) - 3.0f64).abs() < 1e-10);
}

#[test]
fn apply_on_strings() {
    let mut df = Col1::from_cols(vec!["hello".to_string(), "world".to_string()]);
    df.c1_mut().apply(|s| *s = s.to_uppercase());
    assert_eq!(df.c1().to_vec(), &vec!["HELLO".to_string(), "WORLD".to_string()]);
}

// =============================================================================
// map_column
// =============================================================================
#[test]
fn map_column_type_conversion() {
    let df = Col1::from_cols(vec![1, 2, 3]);
    let mapped: Vec<f64> = map_column(df.c1(), |&x| x as f64 * 1.5);
    assert!((mapped[0] - 1.5).abs() < 1e-10);
    assert!((mapped[1] - 3.0).abs() < 1e-10);
    assert!((mapped[2] - 4.5).abs() < 1e-10);
}

#[test]
fn map_column_to_string() {
    let df = Col1::from_cols(vec![10, 20, 30]);
    let labels: Vec<String> = map_column(df.c1(), |x| format!("val_{}", x));
    assert_eq!(labels, vec!["val_10", "val_20", "val_30"]);
}

// =============================================================================
// ColumnUnique
// =============================================================================
#[test]
fn unique_preserves_order() {
    let v = vec![3, 1, 2, 1, 3, 2, 4];
    assert_eq!(v.unique(), vec![3, 1, 2, 4]);
}

#[test]
fn n_unique_count() {
    let v = vec![1, 1, 2, 2, 3];
    assert_eq!(v.n_unique(), 3);
}

#[test]
fn unique_all_same() {
    let v = vec![7, 7, 7, 7];
    assert_eq!(v.unique(), vec![7]);
    assert_eq!(v.n_unique(), 1);
}

#[test]
fn unique_strings() {
    let v = vec!["a".to_string(), "b".to_string(), "a".to_string(), "c".to_string()];
    assert_eq!(v.unique(), vec!["a".to_string(), "b".to_string(), "c".to_string()]);
    assert_eq!(v.n_unique(), 3);
}

#[test]
fn unique_empty() {
    let v: Vec<i32> = vec![];
    assert_eq!(v.unique(), Vec::<i32>::new());
    assert_eq!(v.n_unique(), 0);
}
