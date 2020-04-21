extern crate col_macro;
#[cfg(feature="netcdf")]
extern crate netcdf;
use col_macro::*;
use std::ops::{Index, IndexMut};
use std::slice::SliceIndex;
#[cfg(feature="netcdf")]
use std::error::Error;

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
// Multi-Column implements
// 
// # Example of implements
// ```no-run
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
//     pub fn c1_mut(&mut self) -> &mut T1 {
//         &mut self.col_1
//     } 
//
//     pub fn c2(&self) -> &T2 {
//         &self.col_2
//     }
// 
//     pub fn c2_mut(&mut self) -> &mut T2 {
//         &mut self.col_2
//     } 
// }
// ```
multi_col_impl!();

// =============================================================================
// Netcdf support for MultiCol
// =============================================================================
#[cfg(feature="netcdf")]
pub trait NetCDF: Sized {
    fn write_nc(&self, file_path: &str) -> Result<(), Box<dyn Error>>;
    fn read_nc(file_path: &str) -> Result<Self, Box<dyn Error>>;
    fn read_nc_by_header(file_path: &str, header: Vec<&str>) -> Result<Self, Box<dyn Error>>;
}

impl<T, S> NetCDF for Col2<T, S> where T: Column + Default, S: Column + Default, T::DType: netcdf::Numeric, S::DType: netcdf::Numeric {
    fn write_nc(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let mut f = netcdf::create(file_path)?;

        let dim_name = "col_1".to_string();
        let dim = self.col_1.row();
        f.add_dimension(&dim_name, dim)?;
        let var = &mut f.add_variable::<T::DType>(self.header[0], &[&dim_name])?;
        var.put_values(&self.c1().to_vec()[..], None, None)?;

        let dim_name = "col_2".to_string();
        let dim = self.col_2.row();
        f.add_dimension(&dim_name, dim)?;
        let var = &mut f.add_variable::<T::DType>(self.header[0], &[&dim_name])?;
        var.put_values(&self.c2().to_vec()[..], None, None)?;

        Ok(())
    }
    fn read_nc(file_path: &str) -> Result<Self, Box<dyn Error>> {
        unimplemented!()
    }
    fn read_nc_by_header(file_path: &str, header: Vec<&str>) -> Result<Self, Box<dyn Error>> {
        unimplemented!()
    }
}

// =============================================================================
// Column Main Declaration
// =============================================================================
pub trait Column {
    type DType;
    fn row(&self) -> usize;
    fn idx(&self, n: usize) -> &Self::DType;
    fn idx_mut(&mut self, n: usize) -> &mut Self::DType;
    fn to_vec(&self) -> &Vec<Self::DType>;
}

impl<T> Index<usize> for dyn Column<DType=T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output { 
        self.idx(index)
    }
}

impl<T> IndexMut<usize> for dyn Column<DType=T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { 
        self.idx_mut(index)
    }
}

// impl<T, I> Index<I> for dyn Column<DType=T> where I: SliceIndex<[T]> {
//     type Output = I::Output;

//     fn index(&self, index: I) -> &Self::Output {
//         &self.idx_slice(index)
//     }
// } 

// =============================================================================
// Column Implement for Various type
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
}