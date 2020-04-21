extern crate col_macro;
use col_macro::*;
use std::ops::{Index, IndexMut};

// =============================================================================
// Column Main Declaration
// =============================================================================
pub trait Column {
    type DType;
    fn row(&self) -> usize;
    fn index(&self, n: usize) -> &Self::DType;
    fn index_mut(&mut self, n: usize) -> &mut Self::DType;
}

impl<T> Index<usize> for dyn Column<DType=T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output { 
        self.index(index)
    }
}

impl<T> IndexMut<usize> for dyn Column<DType=T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { 
        self.index_mut(index)
    }
}

// =============================================================================
// MultiCol
// =============================================================================

// MultiCol definition by `proc_macro`
// 
// # Example implements
// ```no-run
// #[derive(Debug, Clone)]
// pub struct Col2<T1, T2> where T1: Column, T2: Column {
//     pub col_1: T1,
//     pub col_2: T2,
// }
// ```
multi_col_def!();

// =============================================================================
// Implements for MultiCol
// =============================================================================
// impl<T1, T2> Col2<T1, T2> where T1: Column + Default, T2: Column + Default {
//     pub fn new() -> Self {
//         Self {
//             col_1: T1::default(),
//             col_2: T2::default(),
//         }
//     }
//
//     pub fn from_cols(c1: T1, c2: T2) -> Self {
//         Self {
//             col_1: c1,
//             col_2: c2,
//         }
//     }
//
//     pub fn c1(&self) -> &T1 {
//         &self.col_1
//     }
//
//     pub fn c2(&self) -> &T2 {
//         &self.col_2
//     }
// }
multi_col_impl!();

// =============================================================================
// Column Implement for Various type
// =============================================================================
impl Column for Vec<u32> {
    type DType = u32;

    fn row(&self) -> usize {
        self.len()
    }

    fn index(&self, n: usize) -> &Self::DType { 
        &self[n] 
    }

    fn index_mut(&mut self, n: usize) -> &mut Self::DType {
        &mut self[n]
    }
}

impl Column for Vec<u64> {
    type DType = u64;

    fn row(&self) -> usize {
        self.len()
    }

    fn index(&self, n: usize) -> &Self::DType { 
        &self[n] 
    }

    fn index_mut(&mut self, n: usize) -> &mut Self::DType {
        &mut self[n]
    }
}

impl Column for Vec<usize> {
    type DType = usize;

    fn row(&self) -> usize {
        self.len()
    }

    fn index(&self, n: usize) -> &Self::DType { 
        &self[n] 
    }

    fn index_mut(&mut self, n: usize) -> &mut Self::DType {
        &mut self[n]
    }
}

impl Column for Vec<i32> {
    type DType = i32;

    fn row(&self) -> usize {
        self.len()
    }

    fn index(&self, n: usize) -> &Self::DType { 
        &self[n] 
    }

    fn index_mut(&mut self, n: usize) -> &mut Self::DType {
        &mut self[n]
    }
}

impl Column for Vec<i64> {
    type DType = i64;

    fn row(&self) -> usize {
        self.len()
    }

    fn index(&self, n: usize) -> &Self::DType { 
        &self[n] 
    }

    fn index_mut(&mut self, n: usize) -> &mut Self::DType {
        &mut self[n]
    }
}

impl Column for Vec<isize> {
    type DType = isize;

    fn row(&self) -> usize {
        self.len()
    }

    fn index(&self, n: usize) -> &Self::DType { 
        &self[n] 
    }

    fn index_mut(&mut self, n: usize) -> &mut Self::DType {
        &mut self[n]
    }
}

impl Column for Vec<f32> {
    type DType = f32;

    fn row(&self) -> usize {
        self.len()
    }

    fn index(&self, n: usize) -> &Self::DType { 
        &self[n] 
    }

    fn index_mut(&mut self, n: usize) -> &mut Self::DType {
        &mut self[n]
    }
}

impl Column for Vec<f64> {
    type DType = f64;

    fn row(&self) -> usize {
        self.len()
    }

    fn index(&self, n: usize) -> &Self::DType { 
        &self[n] 
    }

    fn index_mut(&mut self, n: usize) -> &mut Self::DType {
        &mut self[n]
    }
}

impl<'a> Column for Vec<&'a str> {
    type DType = &'a str;

    fn row(&self) -> usize {
        self.len()
    }

    fn index(&self, n: usize) -> &Self::DType { 
        &self[n] 
    }

    fn index_mut(&mut self, n: usize) -> &mut Self::DType {
        &mut self[n]
    }
}

impl Column for Vec<String> {
    type DType = String;

    fn row(&self) -> usize {
        self.len()
    }

    fn index(&self, n: usize) -> &Self::DType { 
        &self[n] 
    }

    fn index_mut(&mut self, n: usize) -> &mut Self::DType {
        &mut self[n]
    }
}