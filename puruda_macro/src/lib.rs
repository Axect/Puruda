extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;

// Helper: generate identifiers for Col1..Col32
fn col_ident(n: usize) -> Ident {
    Ident::new(&format!("Col{}", n), Span::call_site())
}
fn type_ident(n: usize) -> Ident {
    Ident::new(&format!("T{}", n), Span::call_site())
}
fn field_ident(n: usize) -> Ident {
    Ident::new(&format!("col_{}", n), Span::call_site())
}
fn accessor_ident(n: usize) -> Ident {
    Ident::new(&format!("c{}", n), Span::call_site())
}
fn accessor_mut_ident(n: usize) -> Ident {
    Ident::new(&format!("c{}_mut", n), Span::call_site())
}
fn sort_by_ident(n: usize) -> Ident {
    Ident::new(&format!("sort_by_c{}", n), Span::call_site())
}

// =============================================================================
// multi_col_def!() — generate Col1..Col32 struct definitions
// =============================================================================
#[proc_macro]
pub fn multi_col_def(_item: TokenStream) -> TokenStream {
    let mut all = proc_macro2::TokenStream::new();

    for n in 1..=32 {
        let col_name = col_ident(n);
        let type_params: Vec<_> = (1..=n).map(type_ident).collect();
        let field_names: Vec<_> = (1..=n).map(field_ident).collect();
        let type_refs: Vec<_> = (1..=n).map(type_ident).collect();

        let fields = field_names.iter().zip(type_refs.iter()).map(|(f, t)| {
            quote! { pub #f: #t }
        });

        let where_clauses = type_params.iter().map(|t| {
            quote! { #t: Column }
        });

        let def = quote! {
            #[derive(Debug, Clone)]
            pub struct #col_name<#(#type_params),*> where #(#where_clauses),* {
                pub header: Vec<String>,
                #(#fields),*
            }
        };
        all.extend(def);
    }

    all.into()
}

// =============================================================================
// multi_col_impl!() — generate impl blocks (new, from_cols, accessors, etc.)
// =============================================================================
#[proc_macro]
pub fn multi_col_impl(_item: TokenStream) -> TokenStream {
    let mut all = proc_macro2::TokenStream::new();

    for n in 1..=32 {
        let col_name = col_ident(n);
        let type_params: Vec<_> = (1..=n).map(type_ident).collect();
        let field_names: Vec<_> = (1..=n).map(field_ident).collect();

        // where T1: Column + Default, T2: Column + Default, ...
        let where_clauses = type_params.iter().map(|t| {
            quote! { #t: Column + Default }
        });

        // Default field initializers: col_1: T1::default(), ...
        let default_fields = field_names.iter().zip(type_params.iter()).map(|(f, t)| {
            quote! { #f: #t::default() }
        });

        // from_cols parameters: col_1: T1, col_2: T2, ...
        let from_cols_params = field_names.iter().zip(type_params.iter()).map(|(f, t)| {
            quote! { #f: #t }
        });

        // from_cols field assignments: col_1, col_2, ...
        let from_cols_fields = field_names.iter().map(|f| {
            quote! { #f }
        });

        // Accessor methods
        let accessors = (1..=n).map(|j| {
            let acc = accessor_ident(j);
            let acc_mut = accessor_mut_ident(j);
            let field = field_ident(j);
            let ty = type_ident(j);
            quote! {
                pub fn #acc(&self) -> &#ty {
                    &self.#field
                }

                pub fn #acc_mut(&mut self) -> &mut #ty {
                    &mut self.#field
                }
            }
        });

        let n_lit = proc_macro2::Literal::usize_unsuffixed(n);

        let block = quote! {
            impl<#(#type_params),*> #col_name<#(#type_params),*>
            where #(#where_clauses),*
            {
                pub fn new() -> Self {
                    Self {
                        header: vec![],
                        #(#default_fields),*
                    }
                }

                pub fn set_header(&mut self, header: Vec<&str>) {
                    self.header = header.into_iter().map(|s| s.to_string()).collect();
                }

                pub fn header(&self) -> &Vec<String> {
                    &self.header
                }

                pub fn from_cols(#(#from_cols_params),*) -> Self {
                    Self {
                        header: vec![],
                        #(#from_cols_fields),*
                    }
                }

                #(#accessors)*

                pub fn nrows(&self) -> usize {
                    self.col_1.row()
                }

                pub fn ncols(&self) -> usize {
                    #n_lit
                }

                pub fn shape(&self) -> (usize, usize) {
                    (self.nrows(), self.ncols())
                }

                pub fn len(&self) -> usize {
                    self.nrows()
                }

                pub fn is_empty(&self) -> bool {
                    self.nrows() == 0
                }
            }
        };
        all.extend(block);
    }

    all.into()
}

// =============================================================================
// multi_col_extra_impl!() — head, tail, slice, filter, push_row,
//                            append, concat, reindex, sort_by_cN
// =============================================================================
#[proc_macro]
pub fn multi_col_extra_impl(_item: TokenStream) -> TokenStream {
    let mut all = proc_macro2::TokenStream::new();

    for n in 1..=32 {
        let col_name = col_ident(n);
        let type_params: Vec<_> = (1..=n).map(type_ident).collect();
        let field_names: Vec<_> = (1..=n).map(field_ident).collect();

        // where T1: Column + Default, T1::DType: Clone, Vec<T1::DType>: Into<T1>, ...
        let where_clauses = type_params.iter().map(|t| {
            quote! { #t: Column + Default }
        });
        let clone_clauses = type_params.iter().map(|t| {
            quote! { #t::DType: Clone }
        });
        let into_clauses = type_params.iter().map(|t| {
            quote! { Vec<#t::DType>: Into<#t> }
        });

        // head: self.col_j.to_vec()[..n].to_vec().into()
        let head_fields = field_names.iter().zip(type_params.iter()).map(|(f, _t)| {
            quote! {
                #f: self.#f.to_vec()[..take].to_vec().into()
            }
        });

        let tail_fields = field_names.iter().zip(type_params.iter()).map(|(f, _t)| {
            quote! {
                #f: self.#f.to_vec()[start..].to_vec().into()
            }
        });

        let slice_fields = field_names.iter().zip(type_params.iter()).map(|(f, _t)| {
            quote! {
                #f: self.#f.to_vec()[start..end].to_vec().into()
            }
        });

        let filter_fields = field_names.iter().zip(type_params.iter()).map(|(f, _t)| {
            quote! {
                #f: {
                    let v = self.#f.to_vec();
                    indices.iter().map(|&i| v[i].clone()).collect::<Vec<_>>().into()
                }
            }
        });

        // push_row parameters and body
        let push_params = (1..=n).map(|j| {
            let vname = Ident::new(&format!("v{}", j), Span::call_site());
            let ty = type_ident(j);
            quote! { #vname: #ty::DType }
        });
        let push_body = (1..=n).map(|j| {
            let vname = Ident::new(&format!("v{}", j), Span::call_site());
            let field = field_ident(j);
            quote! { self.#field.push(#vname); }
        });

        // append body
        let append_body = (1..=n).map(|j| {
            let field = field_ident(j);
            quote! {
                for i in 0..other.#field.row() {
                    self.#field.push(other.#field.to_vec()[i].clone());
                }
            }
        });

        // concat fields
        let concat_fields = field_names.iter().map(|f| {
            quote! {
                #f: {
                    let mut v: Vec<_> = self.#f.to_vec().clone();
                    v.extend(other.#f.to_vec().iter().cloned());
                    v.into()
                }
            }
        });

        // reindex fields
        let reindex_fields = field_names.iter().map(|f| {
            quote! {
                #f: {
                    let v = self.#f.to_vec();
                    indices.iter().map(|&i| v[i].clone()).collect::<Vec<_>>().into()
                }
            }
        });

        // sort_by_cN methods — one per column
        let sort_methods = (1..=n).map(|j| {
            let method_name = sort_by_ident(j);
            let field = field_ident(j);
            let ty = type_ident(j);
            quote! {
                pub fn #method_name(&self) -> Self where #ty::DType: Ord {
                    let v = self.#field.to_vec();
                    let mut indices: Vec<usize> = (0..v.len()).collect();
                    indices.sort_by(|&a, &b| v[a].cmp(&v[b]));
                    self.reindex(&indices)
                }
            }
        });

        let block = quote! {
            impl<#(#type_params),*> #col_name<#(#type_params),*>
            where
                #(#where_clauses,)*
                #(#clone_clauses,)*
                #(#into_clauses,)*
            {
                pub fn head(&self, n: usize) -> Self {
                    let take = n.min(self.nrows());
                    Self {
                        header: self.header.clone(),
                        #(#head_fields),*
                    }
                }

                pub fn tail(&self, n: usize) -> Self {
                    let rows = self.nrows();
                    let start = rows.saturating_sub(n);
                    Self {
                        header: self.header.clone(),
                        #(#tail_fields),*
                    }
                }

                pub fn slice(&self, start: usize, end: usize) -> Self {
                    let end = end.min(self.nrows());
                    Self {
                        header: self.header.clone(),
                        #(#slice_fields),*
                    }
                }

                pub fn filter<F: Fn(usize) -> bool>(&self, predicate: F) -> Self {
                    let indices: Vec<usize> = (0..self.nrows())
                        .filter(|&i| predicate(i))
                        .collect();
                    Self {
                        header: self.header.clone(),
                        #(#filter_fields),*
                    }
                }

                pub fn push_row(&mut self, #(#push_params),*) {
                    #(#push_body)*
                }

                pub fn append(&mut self, other: &Self) {
                    #(#append_body)*
                }

                pub fn concat(self, other: Self) -> Self {
                    Self {
                        header: self.header.clone(),
                        #(#concat_fields),*
                    }
                }

                pub fn reindex(&self, indices: &[usize]) -> Self {
                    Self {
                        header: self.header.clone(),
                        #(#reindex_fields),*
                    }
                }

                #(#sort_methods)*
            }
        };
        all.extend(block);
    }

    all.into()
}

// =============================================================================
// multi_col_display_impl!() — Display trait for ColN
// =============================================================================
#[proc_macro]
pub fn multi_col_display_impl(_item: TokenStream) -> TokenStream {
    let mut all = proc_macro2::TokenStream::new();

    for n in 1..=32 {
        let col_name = col_ident(n);
        let type_params: Vec<_> = (1..=n).map(type_ident).collect();

        let where_clauses = type_params.iter().map(|t| {
            quote! { #t: Column }
        });
        let display_clauses = type_params.iter().map(|t| {
            quote! { #t::DType: std::fmt::Display }
        });

        let col_indices: Vec<usize> = (1..=n).collect();
        let field_names_for_width: Vec<_> = (1..=n).map(field_ident).collect();

        // Build width calculations and row formatting
        let width_calcs = field_names_for_width.iter().enumerate().map(|(idx, f)| {
            let idx_lit = proc_macro2::Literal::usize_unsuffixed(idx);
            quote! {
                let mut w = if #idx_lit < self.header.len() {
                    self.header[#idx_lit].len()
                } else {
                    0
                };
                for i in 0..rows {
                    let s = format!("{}", self.#f.idx(i));
                    if s.len() > w { w = s.len(); }
                }
                widths.push(w);
            }
        });

        let header_cells = col_indices.iter().map(|&j| {
            let idx = j - 1;
            let idx_lit = proc_macro2::Literal::usize_unsuffixed(idx);
            quote! {
                let name = if #idx_lit < self.header.len() {
                    &self.header[#idx_lit]
                } else {
                    ""
                };
                write!(f, "{:>width$}", name, width = widths[#idx_lit])?;
                if #idx_lit < widths.len() - 1 {
                    write!(f, "  ")?;
                }
            }
        });

        let sep_cells = col_indices.iter().map(|&j| {
            let idx = j - 1;
            let idx_lit = proc_macro2::Literal::usize_unsuffixed(idx);
            quote! {
                write!(f, "{:->width$}", "", width = widths[#idx_lit])?;
                if #idx_lit < widths.len() - 1 {
                    write!(f, "  ")?;
                }
            }
        });

        let data_cells = field_names_for_width.iter().enumerate().map(|(idx, f)| {
            let idx_lit = proc_macro2::Literal::usize_unsuffixed(idx);
            let n_lit = proc_macro2::Literal::usize_unsuffixed(n);
            quote! {
                write!(f, "{:>width$}", format!("{}", self.#f.idx(i)), width = widths[#idx_lit])?;
                if #idx_lit < #n_lit - 1 {
                    write!(f, "  ")?;
                }
            }
        });

        let block = quote! {
            impl<#(#type_params),*> std::fmt::Display for #col_name<#(#type_params),*>
            where
                #(#where_clauses,)*
                #(#display_clauses,)*
            {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let rows = self.col_1.row();
                    let mut widths: Vec<usize> = Vec::new();
                    #(#width_calcs)*

                    // Header
                    #(#header_cells)*
                    writeln!(f)?;

                    // Separator
                    #(#sep_cells)*
                    writeln!(f)?;

                    // Data rows
                    for i in 0..rows {
                        #(#data_cells)*
                        if i < rows - 1 {
                            writeln!(f)?;
                        }
                    }

                    Ok(())
                }
            }
        };
        all.extend(block);
    }

    all.into()
}

// =============================================================================
// col_vec_impl!() — Column trait for Vec<T>
// =============================================================================
#[proc_macro]
pub fn col_vec_impl(item: TokenStream) -> TokenStream {
    let ty: proc_macro2::TokenStream = item.into();

    let expanded = quote! {
        impl Column for Vec<#ty> {
            type DType = #ty;

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

        impl ColumnApply for Vec<#ty> {
            fn apply<F: FnMut(&mut Self::DType)>(&mut self, mut f: F) {
                for item in self.iter_mut() {
                    f(item);
                }
            }
        }
    };

    expanded.into()
}

// =============================================================================
// multi_col_csv_impl!() — CSV trait implementation for ColN
// =============================================================================
#[proc_macro]
pub fn multi_col_csv_impl(_item: TokenStream) -> TokenStream {
    let mut all = proc_macro2::TokenStream::new();

    for n in 1..=32 {
        let col_name = col_ident(n);
        let type_params: Vec<_> = (1..=n).map(type_ident).collect();

        let where_default = type_params.iter().map(|t| {
            quote! { #t: Column + Default }
        });
        let where_tostr = type_params.iter().map(|t| {
            quote! { #t::DType: ToString + FromStr }
        });
        let where_err = type_params.iter().map(|t| {
            quote! { <#t::DType as FromStr>::Err: std::fmt::Debug + Error }
        });
        let where_into = type_params.iter().map(|t| {
            quote! { Vec<#t::DType>: Into<#t> }
        });

        // write_csv: let c1 = self.c1(); ...
        let write_lets = (1..=n).map(|j| {
            let acc = accessor_ident(j);
            let var = Ident::new(&format!("c{}", j), Span::call_site());
            quote! { let #var = self.#acc(); }
        });

        // write_csv: record[0] = c1.idx(i).to_string(); ...
        let write_records = (1..=n).map(|j| {
            let var = Ident::new(&format!("c{}", j), Span::call_site());
            let idx_lit = proc_macro2::Literal::usize_unsuffixed(j - 1);
            quote! { record[#idx_lit] = #var.idx(i).to_string(); }
        });

        // read_csv: let mut c1: Vec<T1::DType> = vec![]; ...
        let read_decls = (1..=n).map(|j| {
            let var = Ident::new(&format!("c{}", j), Span::call_site());
            let ty = type_ident(j);
            quote! { let mut #var: Vec<#ty::DType> = vec![]; }
        });

        // read_csv: c1.push(rec[0].parse().unwrap()); ...
        let read_pushes = (1..=n).map(|j| {
            let var = Ident::new(&format!("c{}", j), Span::call_site());
            let idx_lit = proc_macro2::Literal::usize_unsuffixed(j - 1);
            quote! { #var.push(rec[#idx_lit].parse().unwrap()); }
        });

        // from_cols args: c1.into(), c2.into(), ...
        let from_cols_args = (1..=n).map(|j| {
            let var = Ident::new(&format!("c{}", j), Span::call_site());
            quote! { #var.into() }
        });

        // Default header names: "c1", "c2", ...
        let header_names = (1..=n).map(|j| {
            let s = format!("c{}", j);
            quote! { #s }
        });

        let n_lit = proc_macro2::Literal::usize_unsuffixed(n);

        let block = quote! {
            impl<#(#type_params),*> CSV for #col_name<#(#type_params),*>
            where
                #(#where_default,)*
                #(#where_tostr,)*
                #(#where_err,)*
                #(#where_into,)*
            {
                fn write_csv(&self, file_path: &str, delimiter: char) -> Result<(), Box<dyn Error>> {
                    let mut wtr = WriterBuilder::new()
                        .delimiter(delimiter as u8)
                        .from_path(file_path)?;
                    #(#write_lets)*
                    let r: usize = self.col_1.row();
                    let c: usize = #n_lit;

                    wtr.write_record(self.header())?;

                    for i in 0..r {
                        let mut record: Vec<String> = vec!["".to_string(); c];
                        #(#write_records)*
                        wtr.write_record(record)?;
                    }
                    wtr.flush()?;
                    Ok(())
                }

                fn read_csv(file_path: &str, delimiter: char) -> Result<Self, Box<dyn Error>> {
                    let mut rdr = ReaderBuilder::new()
                        .has_headers(true)
                        .delimiter(delimiter as u8)
                        .trim(Trim::All)
                        .from_path(file_path)?;

                    #(#read_decls)*

                    for rec in rdr.records() {
                        let rec = rec?;
                        #(#read_pushes)*
                    }

                    let mut col = #col_name::from_cols(#(#from_cols_args),*);
                    col.set_header(vec![#(#header_names),*]);

                    Ok(col)
                }
            }
        };
        all.extend(block);
    }

    all.into()
}

// =============================================================================
// multi_col_json_impl!() — JSON I/O for ColN
// =============================================================================
#[proc_macro]
pub fn multi_col_json_impl(_item: TokenStream) -> TokenStream {
    let mut all = proc_macro2::TokenStream::new();

    for n in 1..=32 {
        let col_name = col_ident(n);
        let type_params: Vec<_> = (1..=n).map(type_ident).collect();
        let field_names: Vec<_> = (1..=n).map(field_ident).collect();

        let where_default = type_params.iter().map(|t| {
            quote! { #t: Column + Default }
        });
        let where_display = type_params.iter().map(|t| {
            quote! { #t::DType: std::fmt::Display + FromStr }
        });
        let where_err = type_params.iter().map(|t| {
            quote! { <#t::DType as FromStr>::Err: std::fmt::Debug + Error }
        });
        let where_into = type_params.iter().map(|t| {
            quote! { Vec<#t::DType>: Into<#t> }
        });

        // to_json_string: serialize each column as JSON array
        let json_col_writes = field_names.iter().enumerate().map(|(idx, f)| {
            let idx_lit = proc_macro2::Literal::usize_unsuffixed(idx);
            let n_lit = proc_macro2::Literal::usize_unsuffixed(n);
            quote! {
                {
                    let name = if #idx_lit < self.header.len() {
                        &self.header[#idx_lit]
                    } else {
                        ""
                    };
                    s.push_str(&format!("    \"{}\": [", name));
                    let v = self.#f.to_vec();
                    for i in 0..v.len() {
                        let val_str = format!("{}", v[i]);
                        // Try to parse as number; if not, quote it
                        if val_str.parse::<f64>().is_ok() {
                            s.push_str(&val_str);
                        } else if val_str == "true" || val_str == "false" {
                            s.push_str(&val_str);
                        } else {
                            s.push_str(&format!("\"{}\"", val_str));
                        }
                        if i < v.len() - 1 {
                            s.push_str(", ");
                        }
                    }
                    s.push(']');
                    if #idx_lit < #n_lit - 1 {
                        s.push(',');
                    }
                    s.push('\n');
                }
            }
        });

        // read_json: parse each column
        let read_decls = (1..=n).map(|j| {
            let var = Ident::new(&format!("c{}", j), Span::call_site());
            let ty = type_ident(j);
            quote! { let mut #var: Vec<#ty::DType> = vec![]; }
        });

        let read_fills = field_names.iter().enumerate().map(|(idx, _f)| {
            let idx_lit = proc_macro2::Literal::usize_unsuffixed(idx);
            let var = Ident::new(&format!("c{}", idx + 1), Span::call_site());
            quote! {
                if #idx_lit < headers.len() {
                    if let Some(arr) = data.get(&headers[#idx_lit]) {
                        for val in arr {
                            #var.push(val.parse().map_err(|e| format!("Parse error: {:?}", e))?);
                        }
                    }
                }
            }
        });

        let from_cols_args = (1..=n).map(|j| {
            let var = Ident::new(&format!("c{}", j), Span::call_site());
            quote! { #var.into() }
        });

        let block = quote! {
            impl<#(#type_params),*> JsonIO for #col_name<#(#type_params),*>
            where
                #(#where_default,)*
                #(#where_display,)*
                #(#where_err,)*
                #(#where_into,)*
            {
                fn to_json_string(&self) -> String {
                    let mut s = String::from("{\n");

                    // headers
                    s.push_str("  \"headers\": [");
                    for (i, h) in self.header.iter().enumerate() {
                        s.push_str(&format!("\"{}\"", h));
                        if i < self.header.len() - 1 {
                            s.push_str(", ");
                        }
                    }
                    s.push_str("],\n");

                    // data
                    s.push_str("  \"data\": {\n");
                    #(#json_col_writes)*
                    s.push_str("  }\n");
                    s.push('}');
                    s
                }

                fn write_json(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
                    std::fs::write(file_path, self.to_json_string())?;
                    Ok(())
                }

                fn read_json(file_path: &str) -> Result<Self, Box<dyn Error>> {
                    let content = std::fs::read_to_string(file_path)?;
                    Self::from_json_string(&content)
                }

                fn from_json_string(s: &str) -> Result<Self, Box<dyn Error>> {
                    let (headers, data) = parse_puruda_json(s)?;

                    #(#read_decls)*

                    #(#read_fills)*

                    let mut col = #col_name::from_cols(#(#from_cols_args),*);
                    let header_refs: Vec<&str> = headers.iter().map(|s| s.as_str()).collect();
                    col.set_header(header_refs);

                    Ok(col)
                }
            }
        };
        all.extend(block);
    }

    all.into()
}

// =============================================================================
// multi_col_describe_impl!() — describe() method for ColN
// =============================================================================
#[proc_macro]
pub fn multi_col_describe_impl(_item: TokenStream) -> TokenStream {
    let mut all = proc_macro2::TokenStream::new();

    for n in 1..=32 {
        let col_name = col_ident(n);
        let type_params: Vec<_> = (1..=n).map(type_ident).collect();

        let where_clauses = type_params.iter().map(|t| {
            quote! { #t: Column + Default }
        });
        let display_clauses = type_params.iter().map(|t| {
            quote! { #t::DType: std::fmt::Display }
        });

        let describe_cols = (1..=n).map(|j| {
            let field = field_ident(j);
            let idx_lit = proc_macro2::Literal::usize_unsuffixed(j - 1);
            quote! {
                {
                    let name = if #idx_lit < self.header.len() {
                        self.header[#idx_lit].clone()
                    } else {
                        format!("col_{}", #idx_lit + 1)
                    };
                    let count = self.#field.row();
                    summaries.push((name, count));
                }
            }
        });

        let block = quote! {
            impl<#(#type_params),*> #col_name<#(#type_params),*>
            where
                #(#where_clauses,)*
                #(#display_clauses,)*
            {
                pub fn describe(&self) {
                    let mut summaries: Vec<(String, usize)> = Vec::new();
                    #(#describe_cols)*

                    // Print describe table
                    let mut max_name = 4usize; // "Name"
                    for (name, _) in &summaries {
                        if name.len() > max_name { max_name = name.len(); }
                    }
                    let count_width = 5; // "Count"

                    println!("{:>nw$}  {:>cw$}", "Name", "Count", nw = max_name, cw = count_width);
                    println!("{:->nw$}  {:->cw$}", "", "", nw = max_name, cw = count_width);
                    for (name, count) in &summaries {
                        println!("{:>nw$}  {:>cw$}", name, count, nw = max_name, cw = count_width);
                    }
                }
            }
        };
        all.extend(block);
    }

    all.into()
}
