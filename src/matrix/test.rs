use super::*;
extern crate mat_macro;
use mat_macro::mat_;

#[test]
fn test_crate() {
    let m: DataMatrix<i32> = mat_![1 2; 3 4;];
    assert_eq!(*m.get(0, 0).unwrap(), 1);
    assert_eq!(*m.get(0, 1).unwrap(), 2);
    assert_eq!(*m.get(1, 0).unwrap(), 3);
    assert_eq!(*m.get(1, 1).unwrap(), 4);

}

#[test]
fn test_add() {
    let mut a: DataMatrix<i32> = mat_![ 1 2; 3 4;];
    let b: DataMatrix<i32> = mat_![4 3; 2 1;];
    a.add_assign(&b);
    assert_eq!(a, mat_![5 5; 5 5;]);
}

#[test]
fn test_boradcast_add() {
    let mut a: DataMatrix<i32> = mat_![1 2; 3 4;];
    let b: DataMatrix<i32> = mat_![1 2;];
    a.add_assign(&b);
    assert_eq!(a, mat_![2 4; 4 6;]);
}

#[test]
fn test_sub() {
    let mut a: DataMatrix<i32> = mat_![ 1 2; 3 4;];
    let b: DataMatrix<i32> = mat_![4 3; 2 1;];
    a.sub_assign(&b);
    assert_eq!(a, mat_![-3 -1; 1 3;]);
}

#[test]
fn test_boradcast_sub() {
    let mut a: DataMatrix<i32> = mat_![1 2; 3 4;];
    let b: DataMatrix<i32> = mat_![1 2;];
    a.sub_assign(&b);
    assert_eq!(a, mat_![0 0; 2 2;]);
}

#[test]
fn test_scale() {
    let mut a = mat_![1 2; 3 4;];
    a.scale(&2);
    assert_eq!(
        a,
        mat_![2 4; 6 8;],
    )
}

#[test]
fn test_dot() {
    let a: DataMatrix<i32> = mat_![1 0; 0 1;];
    let b = mat_![1 2; 3 4;];
    assert_eq!(a.dot(&b).unwrap(), mat_![1 2; 3 4;]);
}

#[test]
fn test_transpose() {
    let a: DataMatrix<i32> = mat_![
        1 2 3;
        4 5 6;
    ].transposed();
    assert_eq!(a, mat_![1 4; 2 5; 3 6;]);
}

#[test]
fn test_inv() {
    use crate::Rational;

    let mut a: DataMatrix<Rational> = mat_![
        1 2;
        3 1;
    ].convert();
    let inv = a.inv().unwrap();

    assert_eq!(inv, mat_![
        (Rational(-1, 5)) (Rational(2, 5));
        (Rational(3, 5)) (Rational(-1, 5));
    ]);
}

