use mat::{DataMatrix, Rational, mat, solve, Mat};


fn main() {
    // let mut a: DataMatrix<Rational> = mat![
    //     1 2 3 4;
    //     4 1 2 0;
    //     2 3 3 2;
    // ].convert();
    // let mut b: DataMatrix<Rational> = mat![
    //     1;
    //     0;
    //     2;
    // ].convert();

    // print!("{}", a.clone_data().eliminated());
    // println!("{}", solve(&mut a, &mut b).unwrap());

    let a: DataMatrix<Rational> = mat![
        1 4 1;
        2 -1 -3;
        1 -5 -4;
        3 -6 -7;
    ].convert().transposed();

    let a = a.eliminated().simplified().transposed();

    println!("{}", a);

 }



