extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn multi_col_def(_item: TokenStream) -> TokenStream {
    let mut f = "".to_string();
    for i in 1 .. 33 {
        let mut ts = "T1".to_string();
        let mut ws = "T1: Column".to_string();
        let mut vs = "col_1: T1,\n".to_string();
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
        let mut ds = "col_1: T1::default(),\n".to_string();
        let mut ps = "col_1: T1".to_string();
        let mut vs = "col_1,\n".to_string();
        let mut cs = "pub fn c1(&self) -> &T1 {{
            &self.col_1
        }}\n".to_string();

        for j in 2 .. i+1 {
            let t = format!(", T{}", j);
            let w = format!(", T{}: Column + Default", j);
            let d = format!("col_{}: T{}::default(),\n", j, j);
            let p = format!(", col_{}: T{}", j, j);
            let v = format!("col_{},\n", j);
            let c = format!("pub fn c{}(&self) -> &T{} {{
                &self.col_{}
            }}\n", j, j, j);

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