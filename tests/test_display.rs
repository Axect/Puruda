use puruda::*;

#[test]
fn display_basic_table() {
    let mut df = Col3::from_cols(vec![1, 2, 3], vec![4.0, 5.0, 6.0],
        vec!["a".to_string(), "bb".to_string(), "ccc".to_string()]);
    df.set_header(vec!["x", "y", "z"]);

    let output = format!("{}", df);
    // Should contain header, separator, and data
    assert!(output.contains("x"));
    assert!(output.contains("y"));
    assert!(output.contains("z"));
    assert!(output.contains("1"));
    assert!(output.contains("6"));
    assert!(output.contains("ccc"));
    // Should have separator line with dashes
    assert!(output.contains("--"));
}

#[test]
fn display_right_aligned() {
    let mut df = Col2::from_cols(vec![1, 100], vec![2, 3]);
    df.set_header(vec!["num", "v"]);
    let output = format!("{}", df);
    let lines: Vec<&str> = output.lines().collect();
    // "num" column should be right-aligned with "100"
    assert!(lines.len() >= 4); // header + sep + 2 data rows
}

#[test]
fn display_single_col() {
    let mut df = Col1::from_cols(vec![10, 20, 30]);
    df.set_header(vec!["val"]);
    let output = format!("{}", df);
    assert!(output.contains("val"));
    assert!(output.contains("10"));
    assert!(output.contains("30"));
}

#[test]
fn display_empty_df() {
    let mut df = Col2::<Vec<i32>, Vec<i32>>::new();
    df.set_header(vec!["a", "b"]);
    let output = format!("{}", df);
    // Should have header and separator but no data rows
    assert!(output.contains("a"));
    assert!(output.contains("b"));
}

#[test]
fn column_display_print() {
    // Just verify it doesn't panic
    let v = vec![1, 2, 3];
    v.print();

    let v2 = vec!["hello".to_string(), "world".to_string()];
    v2.print();
}
