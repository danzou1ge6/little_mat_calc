use mat::*;

fn example_inv() {
    let mut a: DataMatrix<Rational> = mat![
        1   4   6   8;
        2   7   9   4;
        3   2   0   1;
        4   6   2   3;
    ].convert();
    let result = alg::inv(&mut a).unwrap();

    println!("Inv result:\n{}", result);
}

fn example_solve() {
    let mut a: DataMatrix<Rational> = mat![
        1 2 3 4;
        4 1 2 0;
        2 3 3 2;
    ].convert();
    let mut b: DataMatrix<Rational> = mat![
        1;
        0;
        2;
    ].convert();

    println!("Solution:\n{}", solve(&mut a, &mut b).unwrap());
}

fn example_concat() {
    let mut a: DataMatrix<Rational> = mat![
        1 2 3 4;
        4 1 2 0;
        2 3 3 2;
    ].convert();
    let mut b: DataMatrix<Rational> = mat![
        1;
        0;
        2;
    ].convert();

    let augmented = concated_mat![(&mut a) (&mut b);].unwrap();
    println!("Augmented Mat:\n{}", augmented);

    let eliminated = augmented.eliminated();
    println!("Eliminated:\n{}", eliminated);

    let reduced = eliminated.reduced();
    println!("Reduced:\n{}", reduced);
}

fn example_swap() {
    let a: DataMatrix<Rational> = mat![
        1 2 3 4;
        4 1 2 0;
        2 3 3 2;
    ].convert();

    a.row(0).unwrap().swap(&mut a.row(1).unwrap()).unwrap();
    a.row(1).unwrap().swap(&mut a.row(2).unwrap()).unwrap();

    println!("Swaped:\n{}", a);
}

fn example_slice() {
    let a: DataMatrix<Rational> = mat![
        1 2 3 4;
        4 1 2 0;
        2 3 3 2;
    ].convert();

    let mut slice1 = SliceMatrix::new(
        &a, 0, 2, 0, 2
    ).unwrap();
    let slice2 = SliceMatrix::new(
        &a, 1, 2, 1, 2
    ).unwrap();

    slice1.add_assign(&slice2);
    slice1.scale(&Rational::new(2, 1));

    println!("After manipulating slice:\n{}", a);
}

fn example_blocked_mat() {
    let a: DataMatrix<Rational> = mat![
        1 2;
        3 4;
    ].convert();
    let b: DataMatrix<Rational> = mat![
        0 3;
        4 1;
    ].convert();
    
    let c: DataMatrix<Rational> = DataMatrix::identity(2);
    let d: DataMatrix<Rational> = DataMatrix::zeros(2, 2);

    let m: DataMatrix<MatBlock<Rational>> = mat![
        (mat_block!(a)) (mat_block!(b));
        (mat_block!(c)) (mat_block!(d));
    ];

    m.row(0).unwrap().add_assign(
        m.row(1).unwrap().clone_data().scale(&mat_block!(mat![2 0; 0 2;].convert()))
    );

    println!("Blocks after op:\n{}", m);
}

fn example_det() {
    let a: DataMatrix<Rational> = mat![
        1 2 3;
        4 1 2;
        2 3 3;
    ].convert();

    println!("Det of \n{}is {}", a, alg::det(&a).unwrap());
}

fn example_det_poly() {
    let mut a: DataMatrix<Polynomial<i32>> = mat![
        1 2 3;
        4 1 2;
        2 3 3;
    ].convert();
    a.sub_assign(DataMatrix::identity(3).scale(&polynomial!(0, 1)));

    println!("Det polyminal is {}", alg::det(&a).unwrap());
}


fn main() {
    example_concat();
    example_inv();
    example_slice();
    example_solve();
    example_swap();
    example_blocked_mat();
    example_det();
    example_det_poly();
}





