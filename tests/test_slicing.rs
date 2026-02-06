use puruda::*;

fn make_df() -> Col3<Vec<i32>, Vec<f64>, Vec<String>> {
    let mut df = Col3::from_cols(
        vec![10, 20, 30, 40, 50],
        vec![1.1, 2.2, 3.3, 4.4, 5.5],
        vec!["a", "b", "c", "d", "e"].into_iter().map(|s| s.to_string()).collect(),
    );
    df.set_header(vec!["x", "y", "z"]);
    df
}

#[test]
fn head_normal() {
    let df = make_df();
    let h = df.head(3);
    assert_eq!(h.nrows(), 3);
    assert_eq!(h.c1().to_vec(), &vec![10, 20, 30]);
    assert_eq!(h.header(), df.header());
}

#[test]
fn head_exceeds_rows() {
    let df = make_df();
    let h = df.head(100);
    assert_eq!(h.nrows(), 5); // clamped to nrows
}

#[test]
fn head_zero() {
    let df = make_df();
    let h = df.head(0);
    assert_eq!(h.nrows(), 0);
}

#[test]
fn tail_normal() {
    let df = make_df();
    let t = df.tail(2);
    assert_eq!(t.nrows(), 2);
    assert_eq!(t.c1().to_vec(), &vec![40, 50]);
    assert_eq!(t.c3().to_vec(), &vec!["d".to_string(), "e".to_string()]);
}

#[test]
fn tail_exceeds_rows() {
    let df = make_df();
    let t = df.tail(100);
    assert_eq!(t.nrows(), 5);
}

#[test]
fn slice_normal() {
    let df = make_df();
    let s = df.slice(1, 4);
    assert_eq!(s.nrows(), 3);
    assert_eq!(s.c1().to_vec(), &vec![20, 30, 40]);
    assert_eq!(s.c2().to_vec(), &vec![2.2, 3.3, 4.4]);
}

#[test]
fn slice_clamped() {
    let df = make_df();
    let s = df.slice(3, 100);
    assert_eq!(s.nrows(), 2);
    assert_eq!(s.c1().to_vec(), &vec![40, 50]);
}
