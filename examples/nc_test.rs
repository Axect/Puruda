extern crate peroxide;
extern crate puruda;

use peroxide::fuga::*;
use puruda::*;

fn main() -> Result<(), Box<dyn Error>> {
    let x = seq(0,10,1);
    let y = x.fmap(|t| t.powi(2));
    let x = x.into_iter().map(|t| t as i64).collect::<Vec<i64>>();

    let mut c2 = Col2::from_cols(x, y);
    c2.set_header(vec!["x", "y"]);
    c2.write_nc("example_data/test.nc")?;

    let df = DataFrame::read_nc("example_data/test.nc")?;
    df.print();

    Ok(())
}
