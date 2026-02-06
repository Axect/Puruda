use puruda::*;

// =============================================================================
// Integer Numeric
// =============================================================================
#[test]
fn int_sum() {
    let v = vec![1i32, 2, 3, 4, 5];
    assert_eq!(v.sum(), 15);
}

#[test]
fn int_mean() {
    let v = vec![2i32, 4, 6];
    let m = v.mean();
    assert!((m - 4.0).abs() < 1e-10);
}

#[test]
fn int_min_max() {
    let v = vec![5i32, 1, 3, 2, 4];
    assert_eq!(v.min_val(), Some(&1));
    assert_eq!(v.max_val(), Some(&5));
}

#[test]
fn int_var_std() {
    // population variance of [2, 4, 4, 4, 5, 5, 7, 9] = 4.0
    let v = vec![2i32, 4, 4, 4, 5, 5, 7, 9];
    assert!((v.var() - 4.0).abs() < 1e-10);
    assert!((v.std_dev() - 2.0).abs() < 1e-10);
}

#[test]
fn empty_int_stats() {
    let v: Vec<i32> = vec![];
    assert_eq!(v.sum(), 0);
    assert_eq!(v.mean(), 0.0);
    assert_eq!(v.min_val(), None);
    assert_eq!(v.max_val(), None);
    assert_eq!(v.var(), 0.0);
}

// =============================================================================
// Float Numeric
// =============================================================================
#[test]
fn float_sum() {
    let v = vec![1.0f64, 2.0, 3.0];
    assert!((v.sum() - 6.0).abs() < 1e-10);
}

#[test]
fn float_mean() {
    let v = vec![1.0f64, 2.0, 3.0, 4.0];
    assert!((v.mean() - 2.5).abs() < 1e-10);
}

#[test]
fn float_min_max() {
    let v = vec![3.0f64, 1.0, 4.0, 1.5, 9.0];
    assert_eq!(v.min_val(), Some(&1.0));
    assert_eq!(v.max_val(), Some(&9.0));
}

#[test]
fn float_var_std() {
    let v = vec![2.0f64, 4.0, 4.0, 4.0, 5.0, 5.0, 7.0, 9.0];
    assert!((v.var() - 4.0).abs() < 1e-10);
    assert!((v.std_dev() - 2.0).abs() < 1e-10);
}

#[test]
fn f32_basic() {
    let v = vec![1.0f32, 2.0, 3.0];
    assert!((v.sum() - 6.0f32).abs() < 1e-5);
    assert!((v.mean() - 2.0).abs() < 1e-5);
}

// =============================================================================
// Unsigned & other int types
// =============================================================================
#[test]
fn u32_sum() {
    let v = vec![10u32, 20, 30];
    assert_eq!(v.sum(), 60);
}

#[test]
fn u64_mean() {
    let v = vec![100u64, 200, 300];
    assert!((v.mean() - 200.0).abs() < 1e-10);
}

#[test]
fn usize_min_max() {
    let v = vec![5usize, 3, 8, 1];
    assert_eq!(v.min_val(), Some(&1));
    assert_eq!(v.max_val(), Some(&8));
}

#[test]
fn i64_var() {
    let v = vec![10i64, 20, 30];
    // mean = 20, var = ((10-20)^2 + (20-20)^2 + (30-20)^2) / 3 = 200/3
    let expected = 200.0 / 3.0;
    assert!((v.var() - expected).abs() < 1e-10);
}
