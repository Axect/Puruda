extern crate num_traits;
use num_traits::Num;
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

impl<T: Num> Index<usize> for dyn Column<DType=T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output { 
        self.index(index)
    }
}

impl<T: Num> IndexMut<usize> for dyn Column<DType=T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { 
        self.index_mut(index)
    }
}

// =============================================================================
// Column Implement for Various type
// =============================================================================
impl<T: Num> Column for Vec<T> {
    type DType = T;

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