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
