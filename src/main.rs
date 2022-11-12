use mat::{DataMatrix, Rational, mat, solve, Mat};


fn main() {
    let mut a: DataMatrix<Rational> = mat![
        1 2 3 4;
        4 1 2 1;
        2 3 3 2;
    ].unwrap().convert();
    let mut b: DataMatrix<Rational> = mat![
        1;
        0;
        2;
    ].unwrap().convert();

    print!("{}", a.clone_data().eliminated());
    println!("{}", solve(&mut a, &mut b).unwrap());

 }



