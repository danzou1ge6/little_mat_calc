use super::*;
use std::fmt::{Display, Debug};
use unicode_width::UnicodeWidthStr;

/// Use [`prettytable`] to format the matrix into string
pub trait MatPrint<T>: Mat<Item=T> where T: LinearElem + Display {
    fn mat_print_fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.mat_print_buf(f)
    }

    fn mat_print_buf(&self, buf: &mut impl std::fmt::Write) -> std::fmt::Result {

        let mut cell_width = 0;
        let mut s_vec = Vec::new();

        for i in 0..self.rows() { for j in 0..self.cols() {
            let s = self.get(i, j).unwrap().to_string();
            let width = UnicodeWidthStr::width(&s[..]) + 2;
            if width > cell_width { cell_width = width };
            s_vec.push(s);
        }}

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                let s = &s_vec[i * self.cols() + j];
                write!(buf, "{}{}", " ".repeat(cell_width - UnicodeWidthStr::width(&s[..])), s)?;
            }
            writeln!(buf, "")?;
        }
        write!(buf, "")
    }

    fn mat_to_string(&self) -> String {
        let mut buf = String::new();
        self.mat_print_buf(&mut buf).unwrap();
        buf
    }
}

impl<T> MatPrint<T> for dyn Mat<Item=T> where T: LinearElem + Display { }
impl<T, M> MatPrint<T> for M where M: Mat<Item=T>, T: LinearElem + Display { }

impl<T> Display for dyn Mat<Item=T> where T: LinearElem + Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.mat_print_fmt(f) }
}
impl<T> Debug for dyn Mat<Item=T> where T: LinearElem + Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.mat_print_fmt(f) }
}


