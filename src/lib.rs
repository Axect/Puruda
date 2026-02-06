extern crate puruda_macro;
extern crate csv;
use puruda_macro::*;
use csv::{ReaderBuilder, WriterBuilder, Trim};
use std::error::Error;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::string::ToString;
use std::collections::HashMap;

// =============================================================================
// Column Main Declaration
// =============================================================================
pub trait Column {
    type DType;
    fn row(&self) -> usize;
    fn idx(&self, n: usize) -> &Self::DType;
    fn idx_mut(&mut self, n: usize) -> &mut Self::DType;
    fn to_vec(&self) -> &Vec<Self::DType>;
    fn push(&mut self, val: Self::DType);
}

pub trait ColumnApply: Column {
    fn apply<F: FnMut(&mut Self::DType)>(&mut self, f: F);
}

impl<T> Index<usize> for dyn Column<DType = T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.idx(index)
    }
}

impl<T> IndexMut<usize> for dyn Column<DType = T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.idx_mut(index)
    }
}

// =============================================================================
// Column Implement for Various types
// =============================================================================
col_vec_impl!(bool);
col_vec_impl!(u32);
col_vec_impl!(u64);
col_vec_impl!(usize);
col_vec_impl!(i32);
col_vec_impl!(i64);
col_vec_impl!(isize);
col_vec_impl!(f32);
col_vec_impl!(f64);
col_vec_impl!(String);

impl<'a> Column for Vec<&'a str> {
    type DType = &'a str;

    fn row(&self) -> usize {
        self.len()
    }

    fn idx(&self, n: usize) -> &Self::DType {
        &self[n]
    }

    fn idx_mut(&mut self, n: usize) -> &mut Self::DType {
        &mut self[n]
    }

    fn to_vec(&self) -> &Vec<Self::DType> {
        &self
    }

    fn push(&mut self, val: Self::DType) {
        Vec::push(self, val);
    }
}

impl<'a> ColumnApply for Vec<&'a str> {
    fn apply<F: FnMut(&mut Self::DType)>(&mut self, mut f: F) {
        for item in self.iter_mut() {
            f(item);
        }
    }
}

// =============================================================================
// ColumnDisplay trait
// =============================================================================
pub trait ColumnDisplay: Column where Self::DType: std::fmt::Display {
    fn print(&self) {
        let v = self.to_vec();
        print!("[");
        for i in 0..v.len() {
            if i > 0 { print!(", "); }
            print!("{}", v[i]);
        }
        println!("]");
    }
}

impl<C: Column> ColumnDisplay for C where C::DType: std::fmt::Display {}

// =============================================================================
// Numeric trait
// =============================================================================
pub trait Numeric: Column {
    fn sum(&self) -> Self::DType;
    fn mean(&self) -> f64;
    fn min_val(&self) -> Option<&Self::DType>;
    fn max_val(&self) -> Option<&Self::DType>;
    fn var(&self) -> f64;
    fn std_dev(&self) -> f64;
}

macro_rules! impl_numeric_int {
    ($($t:ty),*) => {
        $(
            impl Numeric for Vec<$t> {
                fn sum(&self) -> $t {
                    self.iter().copied().sum()
                }

                fn mean(&self) -> f64 {
                    if self.is_empty() { return 0.0; }
                    self.iter().copied().map(|x| x as f64).sum::<f64>() / self.len() as f64
                }

                fn min_val(&self) -> Option<&$t> {
                    self.iter().min()
                }

                fn max_val(&self) -> Option<&$t> {
                    self.iter().max()
                }

                fn var(&self) -> f64 {
                    if self.is_empty() { return 0.0; }
                    let m = self.mean();
                    let n = self.len() as f64;
                    self.iter().copied().map(|x| {
                        let d = x as f64 - m;
                        d * d
                    }).sum::<f64>() / n
                }

                fn std_dev(&self) -> f64 {
                    self.var().sqrt()
                }
            }
        )*
    };
}

macro_rules! impl_numeric_float {
    ($($t:ty),*) => {
        $(
            impl Numeric for Vec<$t> {
                fn sum(&self) -> $t {
                    self.iter().copied().sum()
                }

                fn mean(&self) -> f64 {
                    if self.is_empty() { return 0.0; }
                    self.iter().copied().map(|x| x as f64).sum::<f64>() / self.len() as f64
                }

                fn min_val(&self) -> Option<&$t> {
                    self.iter().reduce(|a, b| if a <= b { a } else { b })
                }

                fn max_val(&self) -> Option<&$t> {
                    self.iter().reduce(|a, b| if a >= b { a } else { b })
                }

                fn var(&self) -> f64 {
                    if self.is_empty() { return 0.0; }
                    let m = self.mean();
                    let n = self.len() as f64;
                    self.iter().copied().map(|x| {
                        let d = x as f64 - m;
                        d * d
                    }).sum::<f64>() / n
                }

                fn std_dev(&self) -> f64 {
                    self.var().sqrt()
                }
            }
        )*
    };
}

impl_numeric_int!(u32, u64, usize, i32, i64, isize);
impl_numeric_float!(f32, f64);

// =============================================================================
// ColumnUnique trait
// =============================================================================
pub trait ColumnUnique: Column where Self::DType: Clone + Eq + std::hash::Hash {
    fn unique(&self) -> Vec<Self::DType> {
        let v = self.to_vec();
        let mut seen = std::collections::HashSet::new();
        let mut result = Vec::new();
        for item in v.iter() {
            if seen.insert(item.clone()) {
                result.push(item.clone());
            }
        }
        result
    }

    fn n_unique(&self) -> usize {
        self.unique().len()
    }
}

impl<C: Column> ColumnUnique for C where C::DType: Clone + Eq + std::hash::Hash {}

// =============================================================================
// map_column utility function
// =============================================================================
pub fn map_column<C: Column, U, F: Fn(&C::DType) -> U>(col: &C, f: F) -> Vec<U> {
    let v = col.to_vec();
    v.iter().map(|x| f(x)).collect()
}

// =============================================================================
// MultiCol definition by proc_macro
// =============================================================================
multi_col_def!();

// =============================================================================
// Implements for MultiCol
// =============================================================================
multi_col_impl!();

// =============================================================================
// Extra impl: head, tail, slice, filter, push_row, append, concat,
//             reindex, sort_by_cN
// =============================================================================
multi_col_extra_impl!();

// =============================================================================
// Display trait for ColN
// =============================================================================
multi_col_display_impl!();

// =============================================================================
// Describe for ColN
// =============================================================================
multi_col_describe_impl!();

// =============================================================================
// CSV Implementation
// =============================================================================
pub trait CSV: Sized {
    fn write_csv(&self, file_path: &str, delimiter: char) -> Result<(), Box<dyn Error>>;
    fn read_csv(file_path: &str, delimiter: char) -> Result<Self, Box<dyn Error>>;
}

multi_col_csv_impl!();

// =============================================================================
// JSON I/O
// =============================================================================
pub trait JsonIO: Sized {
    fn write_json(&self, file_path: &str) -> Result<(), Box<dyn Error>>;
    fn read_json(file_path: &str) -> Result<Self, Box<dyn Error>>;
    fn to_json_string(&self) -> String;
    fn from_json_string(s: &str) -> Result<Self, Box<dyn Error>>;
}

/// Minimal JSON parser for Puruda's specific format.
/// Parses: {"headers": [...], "data": {"key": [values...], ...}}
pub fn parse_puruda_json(s: &str) -> Result<(Vec<String>, HashMap<String, Vec<String>>), Box<dyn Error>> {
    let s = s.trim();
    // Extract headers array
    let headers_start = s.find("\"headers\"")
        .ok_or("Missing 'headers' key")?;
    let arr_start = s[headers_start..].find('[')
        .ok_or("Missing headers array")?;
    let arr_end = s[headers_start + arr_start..].find(']')
        .ok_or("Missing headers array end")?;
    let headers_str = &s[headers_start + arr_start + 1..headers_start + arr_start + arr_end];
    let headers: Vec<String> = headers_str
        .split(',')
        .map(|h| h.trim().trim_matches('"').to_string())
        .filter(|h| !h.is_empty())
        .collect();

    // Extract data object
    let data_start = s.find("\"data\"")
        .ok_or("Missing 'data' key")?;
    let data_brace = s[data_start..].find('{')
        .ok_or("Missing data object")?;
    let data_section = &s[data_start + data_brace..];

    // Find matching closing brace
    let mut depth = 0;
    let mut data_end = 0;
    for (i, ch) in data_section.char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    data_end = i;
                    break;
                }
            }
            _ => {}
        }
    }
    let data_inner = &data_section[1..data_end];

    let mut data: HashMap<String, Vec<String>> = HashMap::new();

    // Parse each key: [values] pair
    let mut pos = 0;
    let bytes = data_inner.as_bytes();
    while pos < bytes.len() {
        // Find next key
        let key_start = match data_inner[pos..].find('"') {
            Some(i) => pos + i + 1,
            None => break,
        };
        let key_end = match data_inner[key_start..].find('"') {
            Some(i) => key_start + i,
            None => break,
        };
        let key = data_inner[key_start..key_end].to_string();

        // Find array
        let arr_start = match data_inner[key_end..].find('[') {
            Some(i) => key_end + i + 1,
            None => break,
        };
        let arr_end = match data_inner[arr_start..].find(']') {
            Some(i) => arr_start + i,
            None => break,
        };
        let arr_str = &data_inner[arr_start..arr_end];

        // Parse values — handle quoted strings and bare values
        let values = parse_json_array_values(arr_str);
        data.insert(key, values);

        pos = arr_end + 1;
    }

    Ok((headers, data))
}

fn parse_json_array_values(s: &str) -> Vec<String> {
    let s = s.trim();
    if s.is_empty() {
        return vec![];
    }
    let mut values = Vec::new();
    let mut i = 0;
    let chars: Vec<char> = s.chars().collect();
    while i < chars.len() {
        // Skip whitespace and commas
        while i < chars.len() && (chars[i] == ' ' || chars[i] == ',' || chars[i] == '\n' || chars[i] == '\r' || chars[i] == '\t') {
            i += 1;
        }
        if i >= chars.len() { break; }

        if chars[i] == '"' {
            // Quoted string
            i += 1;
            let start = i;
            while i < chars.len() && chars[i] != '"' {
                if chars[i] == '\\' { i += 1; } // skip escaped char
                i += 1;
            }
            let val: String = chars[start..i].iter().collect();
            values.push(val);
            if i < chars.len() { i += 1; } // skip closing quote
        } else {
            // Bare value (number, bool)
            let start = i;
            while i < chars.len() && chars[i] != ',' && chars[i] != ']' && chars[i] != ' ' && chars[i] != '\n' {
                i += 1;
            }
            let val: String = chars[start..i].iter().collect();
            if !val.is_empty() {
                values.push(val);
            }
        }
    }
    values
}

multi_col_json_impl!();
