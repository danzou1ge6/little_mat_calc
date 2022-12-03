use super::*;
use std::fmt::{Debug, Display};
use unicode_width::UnicodeWidthStr;

/// Writre formated matrix to buffer
pub fn mat_print_buf<T>(mat: &dyn Mat<Item = T>, buf: &mut impl std::fmt::Write) -> std::fmt::Result
where
    T: LinearElem + Display,
{
    let mut cell_width = 0;
    let mut s_vec = Vec::new();

    for i in 0..mat.rows() {
        for j in 0..mat.cols() {
            let s = mat.get(i, j).unwrap().to_string();
            let width = UnicodeWidthStr::width(&s[..]) + 2;
            if width > cell_width {
                cell_width = width
            };
            s_vec.push(s);
        }
    }

    for i in 0..mat.rows() {
        for j in 0..mat.cols() {
            let s = &s_vec[i * mat.cols() + j];
            write!(
                buf,
                "{}{}",
                " ".repeat(cell_width - UnicodeWidthStr::width(&s[..])),
                s
            )?;
        }
        writeln!(buf, "")?;
    }
    write!(buf, "")
}

pub fn mat_to_string<T>(mat: &dyn Mat<Item = T>) -> String
where
    T: LinearElem + Display,
{
    let mut buf = String::new();
    mat_print_buf(mat, &mut buf).unwrap();
    buf
}

impl<T> Display for dyn Mat<Item = T>
where
    T: LinearElem + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        mat_print_buf(self, f)
    }
}
impl<T> Debug for dyn Mat<Item = T>
where
    T: LinearElem + Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        mat_print_buf(self, f)
    }
}
