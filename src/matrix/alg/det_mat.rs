use super::super::{Mat, MatError};
use crate::element::LinearElem;
use MatError::*;

/// Encapsules the logic for ennumerating all permutations, as well as the symbol of the permutation
struct Permutation {
    perm: Vec<usize>,
    neg: bool,
}

impl Permutation {
    fn new(len: usize) -> Self {
        Permutation {
            perm: (0..len).collect(),
            neg: false,
        }
    }
    unsafe fn get_unchecked(&self, i: usize) -> usize {
        *self.perm.get_unchecked(i)
    }
    fn neg(&self) -> bool {
        self.neg
    }
}

impl Iterator for Permutation {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let mut ptwo_idx = self.perm.len() - 1;

            let mut found_ptwo = false;
            loop {
                ptwo_idx -= 1;

                if self.perm.get_unchecked(ptwo_idx) < self.perm.get_unchecked(ptwo_idx + 1) {
                    found_ptwo = true;
                    break;
                }
                if ptwo_idx == 0 {
                    break;
                };
            }

            if !found_ptwo {
                return None;
            };

            let ptwo = self.perm.get_unchecked(ptwo_idx);
            let mut swap_idx = self.perm.len() - 1;
            while self.perm.get_unchecked(swap_idx) <= ptwo {
                swap_idx -= 1;
            }
            self.perm.swap(ptwo_idx, swap_idx);

            let mut swap_begin = ptwo_idx + 1;
            let mut swap_end = self.perm.len() - 1;
            while swap_begin < swap_end {
                self.perm.swap(swap_begin, swap_end);
                swap_begin += 1;
                swap_end -= 1;
            }

            self.neg ^= ((self.perm.len() - 1 - ptwo_idx) / 2 + 1) % 2 == 1;

            Some(())
        }
    }
}

pub fn det<T>(mat: &dyn Mat<Item = T>) -> Result<T, MatError>
where
    T: LinearElem,
{
    if !mat.is_square() {
        return Err(NotSquare {
            dim: mat.dimensions(),
        });
    }

    if mat.rows() == 0 {
        return Err(EmptyMatrix);
    }

    let mut perm = Permutation::new(mat.rows());

    let mut result = T::add_zero();

    unsafe {
        loop {
            let mut tmp = if perm.neg() {
                let mut t = T::add_zero();
                t.sub_assign(&T::mul_zero());
                t
            } else {
                T::mul_zero()
            };

            for i in 0..mat.rows() {
                let j = perm.get_unchecked(i);
                tmp.mul_assign(mat.get_unchecked(i, j));
            }

            result.add_assign(&tmp);

            if let None = perm.next() {
                break;
            }
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_perm() {
        let mut perm = Permutation::new(3);
        assert_eq!(perm.perm, vec![0, 1, 2]);
        assert_eq!(perm.neg, false);

        perm.next().unwrap();
        assert_eq!(perm.perm, vec![0, 2, 1]);
        assert_eq!(perm.neg, true);

        perm.next().unwrap();
        assert_eq!(perm.perm, vec![1, 0, 2]);
        assert_eq!(perm.neg, true);

        perm.next().unwrap();
        assert_eq!(perm.perm, vec![1, 2, 0]);
        assert_eq!(perm.neg, false);

        perm.next().unwrap();
        assert_eq!(perm.perm, vec![2, 0, 1]);
        assert_eq!(perm.neg, false);

        perm.next().unwrap();
        assert_eq!(perm.perm, vec![2, 1, 0]);
        assert_eq!(perm.neg, true);

        assert!(perm.next().is_none());
    }

    use crate::DataMatrix;
    use mat_macro::mat_;

    #[test]
    fn test_det() {
        let m: DataMatrix<i32> = mat_![
            0 1 2;
            0 3 4;
            2 8 9;
        ];

        assert_eq!(det(&m).unwrap(), -4);
    }
}
