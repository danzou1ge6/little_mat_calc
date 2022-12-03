extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;

fn transform_item(items: TokenStream) -> (proc_macro2::TokenStream, usize, usize) {

    use proc_macro2::{TokenStream, TokenTree, Punct, Spacing};

    let items: TokenStream = items.into();

    let mut rows: usize = 0;
    let mut cols: usize = 0;
    let mut last_cols = 0;

    let mut cnt = 0;

    let mut v = Vec::new();

    for token in items {
        match token {
            TokenTree::Punct(punct) => {
                if punct.as_char() == ',' || punct.as_char() == ';' {
                    rows += 1;
                    if last_cols == 0 { last_cols = cols };
                    if last_cols != cols {
                        panic!("Matrix must have same number of elements each row")
                    }
                    cols = 0;
                } else if punct.as_char() == '-' {
                    v.push(TokenTree::Punct(Punct::new('-', Spacing::Joint)));
                } else if punct.as_char() == '+' {
                } else {
                    panic!("Use `TokenTree::Punct` of ',' or ';' to seperate rows and brackets to group expressions; other `Punct` are not accepted")
                }
            },
            _ => {
                v.push(token);
                v.push(TokenTree::Punct(Punct::new(',', Spacing::Alone)));
                cols += 1;
                cnt += 1;
            },
        }
    }
    cols = last_cols;

    if cols * rows != cnt {
        panic!("rows*cols={}*{} is inconsistent with element number({}); Did you forgot a `;`?", rows, cols, cnt);
    }

    let mut vec_items = TokenStream::new();
    vec_items.extend(v.into_iter());

    (vec_items, rows, cols)

}

#[proc_macro]
/// For use within the crate
pub fn mat_(items: TokenStream) -> TokenStream {

    let (vec_items, rows, cols) = transform_item(items);

    let ret = quote!(
        unsafe {
            crate::DataMatrix::new_unchecked(
                vec![
                    #vec_items
                ],
                #rows, #cols
            )
        }
    ).into();

    ret
}

#[proc_macro]
pub fn concated_mat_(items: TokenStream) -> TokenStream {

    let (vec_items, rows, cols) = transform_item(items);

    let ret = quote!(
        crate::ConcatedMatrix::new(
            vec![
                #vec_items
            ],
            #rows, #cols
        )
    ).into();

    ret
}

#[proc_macro]
/// Create a matrix owning the data
/// 
/// # Example:
/// ```
/// let m = mat![1 2; 3 4;].unwrap();
/// assert_eq!(m.get(0, 0).unwrap(), 1);
/// ```
/// Note that a parenthess is required for expression, since spaces are used to seperate elements
pub fn mat(items: TokenStream) -> TokenStream {

    let (vec_items, rows, cols) = transform_item(items);

    let ret = quote!(unsafe {
        mat::DataMatrix::new_unchecked(
            vec![
                #vec_items
            ],
            #rows, #cols
        )
    }).into();

    ret
}

#[proc_macro]
/// Create a concated matrix given a matrix of matrix
pub fn concated_mat(items: TokenStream) -> TokenStream {

    let (vec_items, rows, cols) = transform_item(items);

    let ret = quote!(
        mat::ConcatedMatrix::new(
            vec![
                #vec_items
            ],
            #rows, #cols
        )
    ).into();

    ret
}

use std::process::Command;


fn get_compiler_version() -> String {
    let output = Command::new("rustc")
        .args(["--version"])
        .output()
        .unwrap()
        .stdout;
    let output = String::from_utf8(output);
    let output = output.unwrap();
    let pieces: Vec<&str> = output.trim().split(' ').collect();
    pieces[1].to_string()
}

fn get_host() -> String {
    let output = Command::new("rustc")
        .args(["--version", "-v"])
        .output()
        .unwrap()
        .stdout;
    let output = String::from_utf8(output).unwrap();
    for line in output.lines() {
        if line.starts_with("host") {
            let sp = line.find(": ").unwrap();
            return line[sp + 2..].to_string();
        }
    }
    panic!("No `host` entry found")
}

#[proc_macro]
pub fn compiler_version(_: TokenStream) -> TokenStream {
    let v = get_compiler_version();
    quote! { #v }.into()
}

#[proc_macro]
pub fn compiler_host(_: TokenStream) -> TokenStream {
    let host = get_host();
    quote!(#host).into()
}
