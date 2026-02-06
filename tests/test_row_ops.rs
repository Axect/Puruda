use puruda::*;

// =============================================================================
// push_row
// =============================================================================
#[test]
fn push_row_appends() {
    let mut df = Col2::from_cols(vec![1, 2], vec![10.0, 20.0]);
    df.push_row(3, 30.0);
    assert_eq!(df.nrows(), 3);
    assert_eq!(df.c1().to_vec(), &vec![1, 2, 3]);
    assert_eq!(df.c2().to_vec(), &vec![10.0, 20.0, 30.0]);
}

#[test]
fn push_row_to_empty() {
    let mut df = Col2::<Vec<i32>, Vec<i32>>::new();
    df.push_row(1, 2);
    df.push_row(3, 4);
    assert_eq!(df.nrows(), 2);
    assert_eq!(df.c1().to_vec(), &vec![1, 3]);
}

// =============================================================================
// filter
// =============================================================================
#[test]
fn filter_basic() {
    let df = Col2::from_cols(vec![1, 2, 3, 4, 5], vec![10, 20, 30, 40, 50]);
    let filtered = df.filter(|i| *df.c1().idx(i) > 3);
    assert_eq!(filtered.nrows(), 2);
    assert_eq!(filtered.c1().to_vec(), &vec![4, 5]);
    assert_eq!(filtered.c2().to_vec(), &vec![40, 50]);
}

#[test]
fn filter_none_match() {
    let df = Col2::from_cols(vec![1, 2, 3], vec![4, 5, 6]);
    let filtered = df.filter(|i| *df.c1().idx(i) > 100);
    assert_eq!(filtered.nrows(), 0);
}

#[test]
fn filter_all_match() {
    let df = Col2::from_cols(vec![1, 2, 3], vec![4, 5, 6]);
    let filtered = df.filter(|_| true);
    assert_eq!(filtered.nrows(), 3);
    assert_eq!(filtered.c1().to_vec(), df.c1().to_vec());
}

#[test]
fn filter_preserves_header() {
    let mut df = Col2::from_cols(vec![1, 2], vec![3, 4]);
    df.set_header(vec!["a", "b"]);
    let filtered = df.filter(|_| true);
    assert_eq!(filtered.header(), df.header());
}

// =============================================================================
// append
// =============================================================================
#[test]
fn append_in_place() {
    let mut df1 = Col2::from_cols(vec![1, 2], vec![3.0, 4.0]);
    let df2 = Col2::from_cols(vec![5, 6], vec![7.0, 8.0]);
    df1.append(&df2);
    assert_eq!(df1.nrows(), 4);
    assert_eq!(df1.c1().to_vec(), &vec![1, 2, 5, 6]);
    assert_eq!(df1.c2().to_vec(), &vec![3.0, 4.0, 7.0, 8.0]);
}

// =============================================================================
// concat
// =============================================================================
#[test]
fn concat_creates_new() {
    let df1 = Col2::from_cols(vec![1, 2], vec![3.0, 4.0]);
    let df2 = Col2::from_cols(vec![5, 6], vec![7.0, 8.0]);
    let combined = df1.concat(df2);
    assert_eq!(combined.nrows(), 4);
    assert_eq!(combined.c1().to_vec(), &vec![1, 2, 5, 6]);
    assert_eq!(combined.c2().to_vec(), &vec![3.0, 4.0, 7.0, 8.0]);
}

#[test]
fn concat_preserves_header() {
    let mut df1 = Col2::from_cols(vec![1], vec![2]);
    df1.set_header(vec!["a", "b"]);
    let df2 = Col2::from_cols(vec![3], vec![4]);
    let combined = df1.concat(df2);
    assert_eq!(combined.header(), &vec!["a".to_string(), "b".to_string()]);
}

// =============================================================================
// Column push
// =============================================================================
#[test]
fn column_push() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    Column::push(&mut v, 4);
    assert_eq!(v, vec![1, 2, 3, 4]);
}
