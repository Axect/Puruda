extern crate proc_macro;
use proc_macro::TokenStream;

// =============================================================================
// Multicol
// =============================================================================
#[proc_macro]
pub fn multi_col_def(_item: TokenStream) -> TokenStream {
    let mut f = "".to_string();
    for i in 1 .. 33 {
        let mut ts = "T1".to_string();
        let mut ws = "T1: Column".to_string();
        let mut vs = "header: Vec<&'static str>,\ncol_1: T1,\n".to_string();
        for j in 2 .. i+1 {
            let mut t = ", T".to_string();
            t.push_str(&j.to_string());
            let mut w = ", T".to_string();
            w.push_str(&j.to_string());
            w.push_str(": Column");
            let v = format!("col_{}: T{},\n", j, j);
            ts.push_str(&t);
            ws.push_str(&w);
            vs.push_str(&v);
        }
        let g = format!("#[derive(Debug, Clone)]
        pub struct Col{}<{}> where {} {{
            {}
        }}\n", i, ts, ws, vs);
        f.push_str(g.as_str());
    }
    f.parse().unwrap()
}

#[proc_macro]
pub fn multi_col_impl(_item: TokenStream) -> TokenStream {
    let mut f = "".to_string();
    for i in 1 .. 33 {
        let mut ts = "T1".to_string();
        let mut ws = "T1: Column + Default".to_string();
        let mut ds = "header: vec![],\ncol_1: T1::default(),\n".to_string();
        let mut ps = "col_1: T1".to_string();
        let mut vs = "header: vec![],\ncol_1,\n".to_string();
        let mut cs = "pub fn c1(&self) -> &T1 {
            &self.col_1
        }
        
        pub fn c1_mut(&mut self) -> &mut T1 {
            &mut self.col_1
        }".to_string();

        for j in 2 .. i+1 {
            let t = format!(", T{}", j);
            let w = format!(", T{}: Column + Default", j);
            let d = format!("col_{}: T{}::default(),\n", j, j);
            let p = format!(", col_{}: T{}", j, j);
            let v = format!("col_{},\n", j);
            let c = format!("pub fn c{}(&self) -> &T{} {{
                &self.col_{}
            }}
            
            pub fn c{}_mut(&mut self) -> &mut T{} {{
                &mut self.col_{}
            }}", j, j, j, j, j, j);

            ts.push_str(&t);
            ws.push_str(&w);
            ds.push_str(&d);
            ps.push_str(&p);
            vs.push_str(&v);
            cs.push_str(&c);
        }
        let g = format!("impl<{}> Col{}<{}> where {} {{
            pub fn new() -> Self {{
                Self {{
                    {}
                }}
            }}

            pub fn set_header(&mut self, header: Vec<&'static str>) {{
                self.header = header;
            }} 

            pub fn header(&self) -> &Vec<&'static str> {{
                &self.header
            }}

            pub fn from_cols({}) -> Self {{
                Self {{
                    {}
                }}
            }}

            {}
        }}\n", ts, i, ts, ws, ds, ps, vs, cs);
        f.push_str(g.as_str());
    }
    f.parse().unwrap()
}


// =============================================================================
// Column Implementation
// =============================================================================
#[proc_macro]
pub fn col_vec_impl(item: TokenStream) -> TokenStream {
    format!(
        "impl Column for Vec<{}> {{
            type DType = {};

            fn row(&self) -> usize {{
                self.len()
            }}
        
            fn idx(&self, n: usize) -> &Self::DType {{ 
                &self[n] 
            }}
        
            fn idx_mut(&mut self, n: usize) -> &mut Self::DType {{
                &mut self[n]
            }}

            fn to_vec(&self) -> &Vec<Self::DType> {{
                &self
            }}
        }}",
        item,
        item
    ).parse().unwrap()
}

// =============================================================================
// CSV Implementation
// =============================================================================
#[proc_macro]
pub fn multi_col_csv_impl(_item: TokenStream) -> TokenStream {
    let mut f = "".to_string();
    for i in 1 .. 33 {
        let mut ts = "T1".to_string();
        let mut ws1 = "T1: Column + Default".to_string();
        let mut ws2 = "T1::DType: ToString + FromStr".to_string();
        let mut ws3 = "<T1::DType as FromStr>::Err: std::fmt::Debug + Error".to_string();
        let mut ws4 = "Vec<T1::DType>: Into<T1>".to_string();
        let mut w_body_1 = "let c1 = self.c1();\n".to_string();
        let mut w_body_2 = "record[0] = c1.idx(i).to_string();\n".to_string();
        let mut r_body_1 = "let mut c1: Vec<T1::DType> = vec![];\n".to_string();
        let mut r_body_2 = "c1.push(rec[0].parse().unwrap());\n".to_string();
        let mut r_body_3 = "c1.into()".to_string();
        let mut r_body_4 = "\"c1\"".to_string();
        for j in 2 .. i+1 {
            ts.push_str(&format!(", T{}", j));
            ws1.push_str(&format!(", T{}: Column + Default", j));
            ws2.push_str(&format!(", T{}::DType: ToString + FromStr", j));
            ws3.push_str(&format!(", <T{}::DType as FromStr>::Err: std::fmt::Debug + Error", j));
            ws4.push_str(&format!(", Vec<T{}::DType>: Into<T{}>", j, j));
            w_body_1.push_str(&format!("let c{} = self.c{}();\n", j, j));
            w_body_2.push_str(&format!("record[{}] = c{}.idx(i).to_string();\n", j-1, j));
            r_body_1.push_str(&format!("let mut c{}: Vec<T{}::DType> = vec![];\n", j, j));
            r_body_2.push_str(&format!("c{}.push(rec[{}].parse().unwrap());\n", j, j-1));
            r_body_3.push_str(&format!(", c{}.into()", j));
            r_body_4.push_str(&format!(", \"c{}\"", j));
        }

        let read = format!("fn read_csv(file_path: &str, delimiter: char) -> Result<Self, Box<dyn Error>> {{
            let mut rdr = ReaderBuilder::new()
                .has_headers(true)
                .delimiter(delimiter as u8)
                .from_path(file_path)?;

            {}

            for rec in rdr.records() {{
                let rec = rec?;
                {}
            }}

            let mut col = Col{}::from_cols({});
            col.set_header(vec![{}]);

            Ok(col)
        }}\n",
        r_body_1,
        r_body_2,
        i,
        r_body_3,
        r_body_4);

        let write = format!("#[cfg(feature=\"csv\")]
        impl<{}> CSV for Col{}<{}>
        where
            {},
            {},
            {},
            {},
        {{
            fn write_csv(&self, file_path: &str, delimiter: char) -> Result<(), Box<dyn Error>> {{
                let mut wtr = WriterBuilder::new()
                    .delimiter(delimiter as u8)
                    .from_path(file_path)?;
                {}
                let r: usize = c1.row();
                let c: usize = {};

                wtr.write_record(self.header())?;

                for i in 0 .. r {{
                    let mut record: Vec<String> = vec![\"\".to_string(); c];
                    {}
                    wtr.write_record(record)?;
                }}
                wtr.flush()?;
                Ok(())
            }}

            {}
        }}\n",
        ts,
        i,
        ts,
        ws1,
        ws2,
        ws3,
        ws4,
        w_body_1,
        i,
        w_body_2,
        read);

        f.push_str(&write);
    }
    f.parse().unwrap()
}
