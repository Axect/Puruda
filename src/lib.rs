extern crate puruda_macro;
extern crate csv;
use puruda_macro::*;
use csv::{ReaderBuilder, WriterBuilder, Trim};
use std::error::Error;
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use std::string::ToString;

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
//#[cfg(feature = "netcdf")]
//pub trait NetCDF: Sized {
//    fn write_nc(&self, file_path: &str) -> Result<(), Box<dyn Error>>;
//    fn read_nc(file_path: &str) -> Result<Self, Box<dyn Error>>;
//    fn read_nc_by_header(file_path: &str, header: Vec<&str>) -> Result<Self, Box<dyn Error>>;
//}
//
//#[cfg(feature = "netcdf")]
//impl<T, S> NetCDF for Col2<T, S>
//where
//    T: Column + Default,
//    S: Column + Default,
//    T::DType: netcdf::Numeric,
//    S::DType: netcdf::Numeric,
//{
//    fn write_nc(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
//        let mut f = netcdf::create(file_path)?;
//
//        let dim_name = "col_1".to_string();
//        let dim = self.col_1.row();
//        f.add_dimension(&dim_name, dim)?;
//        let var = &mut f.add_variable::<T::DType>(self.header[0], &[&dim_name])?;
//        var.put_values(&self.c1().to_vec()[..], None, None)?;
//
//        let dim_name = "col_2".to_string();
//        let dim = self.col_2.row();
//        f.add_dimension(&dim_name, dim)?;
//        let var = &mut f.add_variable::<T::DType>(self.header[0], &[&dim_name])?;
//        var.put_values(&self.c2().to_vec()[..], None, None)?;
//
//        Ok(())
//    }
//    fn read_nc(file_path: &str) -> Result<Self, Box<dyn Error>> {
//        unimplemented!()
//    }
//    fn read_nc_by_header(file_path: &str, header: Vec<&str>) -> Result<Self, Box<dyn Error>> {
//        unimplemented!()
//    }
//}

// =============================================================================
// CSV Implementation
// =============================================================================
#[cfg(feature = "csv")]
pub trait CSV: Sized {
    fn write_csv(&self, file_path: &str, delimiter: char) -> Result<(), Box<dyn Error>>;
    fn read_csv(file_path: &str, delimiter: char) -> Result<Self, Box<dyn Error>>;
}

//#[cfg(feature = "csv")]
//impl<T, S> CSV for Col2<T, S>
//where
//    T: Column + Default,
//    S: Column + Default,
//    T::DType: ToString + FromStr,
//    S::DType: ToString + FromStr,
//    <T::DType as FromStr>::Err: std::fmt::Debug + Error,
//    <S::DType as FromStr>::Err: std::fmt::Debug + Error,
//    Vec<T::DType>: Into<T>,
//    Vec<S::DType>: Into<S>,
//{
//    fn write_csv(&self, file_path: &str, delimiter: char) -> Result<(), Box<dyn Error>> {
//        let mut wtr = WriterBuilder::new()
//            .delimiter(delimiter as u8)
//            .from_path(file_path)?;
//        let c1 = self.c1();
//        let c2 = self.c2();
//        let r: usize = c1.row();
//        let c: usize = 2; // Col2
//
//        wtr.write_record(self.header())?;
//
//        for i in 0..r {
//            let mut record: Vec<String> = vec!["".to_string(); c];
//            record[0] = c1.idx(i).to_string();
//            record[1] = c2.idx(i).to_string();
//            wtr.write_record(record)?;
//        }
//        wtr.flush()?;
//
//        Ok(())
//    }
//
//    fn read_csv(file_path: &str, delimiter: char) -> Result<Self, Box<dyn Error>> {
//        let mut rdr = ReaderBuilder::new()
//            .has_headers(true)
//            .delimiter(delimiter as u8)
//            .from_path(file_path)?;
//
//        let mut c1: Vec<T::DType> = vec![];
//        let mut c2: Vec<S::DType> = vec![];
//
//        for rec in rdr.records() {
//            let rec = rec?;
//            c1.push(rec[0].parse().unwrap());
//            c2.push(rec[1].parse().unwrap());
//        }
//
//        let mut col = Col2::from_cols(c1.into(), c2.into());
//        col.set_header(vec!["c1", "c2"]);
//
//        Ok(col)
//    }
//}
multi_col_csv_impl!();

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
