use puruda::*;

// =============================================================================
// Construction & Accessors
// =============================================================================
#[test]
fn new_creates_empty() {
    let df = Col2::<Vec<i32>, Vec<f64>>::new();
    assert_eq!(df.nrows(), 0);
    assert_eq!(df.ncols(), 2);
    assert!(df.is_empty());
}

#[test]
fn from_cols_and_accessors() {
    let df = Col3::from_cols(vec![1, 2, 3], vec![4.0, 5.0, 6.0], vec![7u32, 8, 9]);
    assert_eq!(df.c1().to_vec(), &vec![1, 2, 3]);
    assert_eq!(df.c2().to_vec(), &vec![4.0, 5.0, 6.0]);
    assert_eq!(df.c3().to_vec(), &vec![7, 8, 9]);
}

#[test]
fn header_set_and_get() {
    let mut df = Col2::from_cols(vec![1], vec![2.0]);
    assert_eq!(df.header().len(), 0);

    df.set_header(vec!["x", "y"]);
    assert_eq!(df.header(), &vec!["x".to_string(), "y".to_string()]);
}

#[test]
fn mutate_column() {
    let mut df = Col2::from_cols(vec![1, 2], vec![3, 4]);
    *df.c1_mut() = vec![10, 20];
    assert_eq!(df.c1().to_vec(), &vec![10, 20]);
}

// =============================================================================
// Shape
// =============================================================================
#[test]
fn shape_len_is_empty() {
    let df = Col2::from_cols(vec![1, 2, 3], vec![4.0, 5.0, 6.0]);
    assert_eq!(df.shape(), (3, 2));
    assert_eq!(df.nrows(), 3);
    assert_eq!(df.ncols(), 2);
    assert_eq!(df.len(), 3);
    assert!(!df.is_empty());
}

#[test]
fn col1_shape() {
    let df = Col1::from_cols(vec![10, 20, 30, 40]);
    assert_eq!(df.shape(), (4, 1));
    assert_eq!(df.ncols(), 1);
}
