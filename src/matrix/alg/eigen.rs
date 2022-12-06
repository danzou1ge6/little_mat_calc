use crate::Complex;
use crate::{matrix::Mat, DataMatrix, error::MatError, SliceMatrix};
use crate::element::AddZero;

use super::{col_normal_unchecked, col_normal_sqr_unchecked};


/// Calculate the Householder Matrix of a column vector.
/// However, the dimension of the input vector is not checked,
/// so the first column is taken.
/// 
/// # Householder Matrix:
/// Given a vector `v`, the corresponding Householder Matrix `A`
/// can reduce all elements of `v` except `v[0]` to zero.
pub unsafe fn householder_unchecked(v: &dyn Mat<Item=f64>) -> DataMatrix<f64> {
    let dim = v.rows();
    
    let c = -v.get_unchecked(0, 0).signum() * col_normal_unchecked(v, 0);

    let mut w = v.clone_data();
    *w.get_mut_unchecked(0, 0) -= c;

    let mut h = DataMatrix::identity(dim);
    h.sub_assign(
        w.dot_unchecked(&w.clone_data().transposed())
        .scale(&(2.0 / col_normal_sqr_unchecked(&w, 0)))
    );
    h
}

pub fn householder(v: &dyn Mat<Item = f64>) -> Result<DataMatrix<f64>, MatError> {
    if v.cols() != 1 {
        return Err(MatError::ExpectedVector);
    }
    if v.rows() == 0 {
        return Err(MatError::EmptyMatrix);
    }
    unsafe { Ok(householder_unchecked(v)) }
}

/// Calculate the Hessenberg Matrix of `m`, where all elements `m[i, j]` satisfying
/// `i >= j + 2` are zeros, while not altering `m`'s eigen values
pub unsafe fn hessenberg_unchecked(m: &mut dyn Mat<Item = f64>) {
    let n = m.rows();

    for k in 1..n - 1 {
        let v = SliceMatrix::new_unchecked(m, k, n - k, k - 1, 1);

        // If `v` is zero, skip the transoformation of this slice
        if col_normal_sqr_unchecked(&v, 0).is_add_zero() { continue; }

        let h = householder_unchecked(&v);

        let mut b = SliceMatrix::new_unchecked(m, 0, k, k, n - k);
        let mut c = SliceMatrix::new_unchecked(m, k, n - k, 0, k);
        let mut d = SliceMatrix::new_unchecked(m, k, n - k, k, n - k);

        let bh = b.dot_unchecked(&h);
        let hc = h.dot_unchecked(&c);
        let hdh = h.dot_unchecked(&d).dot_unchecked(&h);

        b.write_data_unchecked(&bh);
        c.write_data_unchecked(&hc);
        d.write_data_unchecked(&hdh);

   }
}

/// Check if m is square than call `hessengerb_unchecked`
pub fn hessengerb(m: &mut dyn Mat<Item = f64>) -> Result<(), MatError> {
    if m.dimensions() == (0, 0) {
        return Err(MatError::EmptyMatrix);
    }
    if !m.is_square() { return Err(MatError::NotSquare { dim: m.dimensions() }); }
    
    unsafe { hessenberg_unchecked(m) };
    Ok(())
}

/// Perform the QR dicomposition: 
/// Transform `m` to `R` and return `Q`
/// Where `R` is upper-triangle and `Q` is othogonal
pub unsafe fn qr_unchecked(m: &mut dyn Mat<Item = f64>) -> DataMatrix<f64> {
    let n = m.rows();
    let q = DataMatrix::identity(n);

    for k in 0..n - 1 {
        let v = SliceMatrix::new_unchecked(m, k, n - k, k, 1);

        if col_normal_sqr_unchecked(&v, 0).is_add_zero() { continue; }

        let h = householder_unchecked(&v);

        {
            let mut c = SliceMatrix::new_unchecked(m, k, n - k, 0, k);
            let mut d = SliceMatrix::new_unchecked(m, k, n - k, k, n - k);

            let hc = h.dot_unchecked(&c);
            let hd = h.dot_unchecked(&d);

            c.write_data_unchecked(&hc);
            d.write_data_unchecked(&hd);
        }
 
        {
            let mut b = SliceMatrix::new_unchecked(&q, 0, k, k, n  -k);
            let mut d = SliceMatrix::new_unchecked(&q, k, n - k, k, n - k);

            let bh = b.dot_unchecked(&h);
            let dh = d.dot_unchecked(&h);

            b.write_data_unchecked(&bh);
            d.write_data_unchecked(&dh);
        }
        
    }

    q
}

/// Check the dimension of the input matrix, then call `qr_unchecked`
pub fn qr(m: &mut dyn Mat<Item = f64>) -> Result<DataMatrix<f64>, MatError> {
    if m.dimensions() == (0, 0) { return Err(MatError::EmptyMatrix); }
    if !m.is_square() {
        return Err(MatError::NotSquare { dim: m.dimensions() });
    }

    Ok(unsafe { qr_unchecked(m) })
}

fn eigval_2dim(a: f64, b: f64, c: f64, d: f64) -> (Complex, Complex) {
    let re = (a + d) / 2.0;
    let det =  a * a + d * d + 4.0 * b * c - 2.0 * a * d;
    if det >= 0.0 {
        (Complex(re + det.sqrt() / 2.0, 0.0), Complex(re - det.sqrt() / 2.0, 0.0))
    } else {
        (Complex(re, (-det).sqrt() / 2.0), Complex(re, -(-det).sqrt() / 2.0))
    }
}


/// To solve eigen values
pub struct EigenValueSolver {
    pub mat: DataMatrix<f64>,
}

impl EigenValueSolver {
    /// Create a new solver. `mat` is transformed into hessenberg in this method
    pub fn new(mat: DataMatrix<f64>) -> Result<Self, MatError> {
        if mat.dimensions() == (0, 0) { return Err(MatError::EmptyMatrix); }
        if !mat.is_square() {
            return Err(MatError::NotSquare { dim: mat.dimensions() });
        }

        Ok(unsafe { Self::new_unchecked(mat, )} )
    }
    pub unsafe fn new_unchecked(mut mat: DataMatrix<f64>) -> Self {
        hessenberg_unchecked(&mut mat);
        Self { mat, }
    }
    /// Read the diagnoal of the matrix. Caution that this is not the eigen values
    /// because there might be complex eigen values.
    pub fn read_diag(&self) -> Vec<f64> {
        let mut v = Vec::with_capacity(self.mat.rows());
        for i in 0..self.mat.rows() {
            unsafe { v.push(*self.mat.get_unchecked(i, i)); }
        }
        v
    }
    /// Iter until the delta of elements on the diagnol is smaller than `epsilon`
    /// or, after a `max_iter`, then return the matrix
    pub fn eigen_mat(mut self, epsilon: f64, max_iter: usize) -> DataMatrix<f64>{
        let mut cnt = 0;
        while let Some(delta) = self.next() {
            if delta < epsilon { break; }
            if cnt > max_iter { break; }
            cnt += 1;
        } 
        self.mat
    }
    /// Call `eigen_mat` first, then calculate eigen values from the result of
    /// `eigen_mat`
    pub fn eigen_values(self, epsilon: f64, max_iter: usize) -> Vec<Complex> {

        let mat = self.eigen_mat(epsilon, max_iter);

        let n = mat.rows();
        let mut k = 0;
        let mut eigvals = Vec::new();
        unsafe {
            while k < n  {
                if k < n - 1 && !mat.get_unchecked(k + 1, k).is_add_zero() {
                    let a = mat.get_unchecked(k, k);
                    let b = mat.get_unchecked(k, k + 1);
                    let c = mat.get_unchecked(k + 1, k);
                    let d = mat.get_unchecked(k + 1, k + 1);
                    let (ev1, ev2) = eigval_2dim(*a, *b, *c, *d);
                    eigvals.push(ev1);
                    eigvals.push(ev2);
                    k += 2;
                } else {
                    eigvals.push(Complex::from(*mat.get_unchecked(k, k)));
                    k += 1;
                }
            }
        }

        eigvals
    }
}

impl Iterator for EigenValueSolver {
    type Item = f64;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let old_diag = DataMatrix::new_unchecked(self.read_diag(), 1, self.mat.rows());

            let q = qr_unchecked(&mut self.mat);
            let r = &self.mat;

            self.mat = r.dot_unchecked(&q);

            let diag = DataMatrix::new_unchecked(self.read_diag(), 1, self.mat.rows());
            let delta = col_normal_sqr_unchecked(&old_diag.sub(&diag), 0) ;
            Some(delta)
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use mat_macro::mat_;

    #[test]
    fn test_householder() {
        let v = mat_![1.0; 2.0; 3.0; 4.0;];
        let h = householder(&v).unwrap();
        let r = h.dot(&v).unwrap();

        assert!(r.get(0, 0).unwrap().abs() > 1e-3);
        assert!(r.get(1, 0).unwrap().abs() < 1e-3);
        assert!(r.get(2, 0).unwrap().abs() < 1e-3);
    }

    #[test]
    fn test_hessenburg() {
        let mut m = mat_![1.0 2.0 3.0; 2.0 3.0 4.0; 5.0 2.0 1.0;];
        hessengerb(&mut m).unwrap();
        assert!(m.get(2, 0).unwrap().abs() < 1e-4);
    }

    #[test]
    fn test_qr() {
        let mut m = mat_![1.0 2.0 3.0; 2.0 3.0 4.0; 5.0 2.0 1.0;];
        let old_m = m.clone_data();

        let q = qr(&mut m).unwrap();
        assert!(m.get(1, 0).unwrap().abs() < 1e-4);
        assert!(m.get(2, 0).unwrap().abs() < 1e-4);
        assert!(m.get(2, 1).unwrap().abs() < 1e-4);
        
        let back = old_m.sub(&q.dot(&m).unwrap());
        for i in 0..back.rows() { for j in 0..back.cols() {
            assert!(back.get(i, j).unwrap().abs() < 1e-4);
        }}
    }

    #[test]
    fn test_eigenmat() {
        let m = mat_![1.0 4.0; 5.0 2.0;];
        let es = EigenValueSolver::new(m).unwrap();

        let r =  es.eigen_mat(1e-6, 999);
        let mut evs = vec![
            r.get(0, 0).unwrap(),
            r.get(1, 1).unwrap()
        ];
        if evs[0] > evs[1] { evs.swap(0, 1) };
        assert!((evs[0] - (-3.0)).abs() < 1e-2);
        assert!((evs[1] - 6.0).abs() < 1e-2)
    }

    #[test]
    fn test_eigenvals() {
        let m = mat_![3.0 5.0;-1.0 -1.0;];
        let es = EigenValueSolver::new(m).unwrap();

        let mut evs = es.eigen_values(1e-2, 999);
        if evs[0].im() > evs[1].im() { evs.swap(0, 1) };
        if (evs[0] - &Complex(1.0, -1.0)).normal() < 1e-2 &&
            (evs[1] - &Complex(1.0, 1.0)).normal() < 1e-2 {}
        else {
            panic!("{:?}", &evs);
        }
 
    }

    #[test]
    fn test_eigenvals2() {
        let m: DataMatrix<f64> = mat_![3 5 0 0;-1 -1 0 0;0 0 1 4; 0 0 5 2;].convert();

        let mut evs = EigenValueSolver::new(m.clone_data())
            .unwrap()
            .eigen_values(1e-2, 999);
        
        if evs[0].im() > evs[1].im() { evs.swap(0, 1) };
        if (evs[0] - &Complex(1.0, -1.0)).normal() < 1e-2 &&
             (evs[1] - &Complex(1.0, 1.0)).normal() < 1e-2 {}
        else {
            panic!("{:?}", &evs);
        }
    }
}
