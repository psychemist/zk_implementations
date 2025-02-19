
use ark_ff::PrimeField;

pub struct MultilinearPoly<F: PrimeField> {
    pub evaluations: Vec<F>,
}

impl<F: PrimeField> MultilinearPoly<F> {
    /// Constructs a new multilinear polynomial.
    /// `num_vars` is the number of variables; `evaluations` must have length 2^num_vars.
    pub fn new(num_vars: u32, evaluations: Vec<F>) -> Self {
        if 1 << num_vars != evaluations.len() {
            panic!("Not a valid Boolean hypercube evaluation!");
        }
        MultilinearPoly { evaluations }
    }

    /// Partially evaluates the multilinear polynomial.
    ///
    /// The `assignments` slice has one entry per variable. Each entry is:
    /// - `Some(v)`: fix that variable to the field element `v`.
    /// - `None`: leave that variable free.
    ///
    /// The function returns a new evaluation vector for the polynomial in the remaining free variables.
    /// In the special case where all variables are fixed, the result is a vector of length 1 (the full evaluation).
    pub fn partial_evaluate(&self, assignments: &[Option<F>]) -> Vec<F> {
        let num_vars = assignments.len();
        if self.evaluations.len() != (1 << num_vars) {
            panic!("Assignment length does not match number of variables");
        }

        // Start with a copy of the full evaluation vector.
        let mut current = self.evaluations.clone();

        // Process variables in reverse order (from most significant to least significant)
        for i in (0..num_vars).rev() {
            if let Some(v) = assignments[i] {
                // For a fixed variable, collapse that dimension.
                // For variable i, assuming variable 0 is LSB,
                // the block size is 2^(i+1) and the stride is 2^i.
                let stride = 1 << i;         // number of entries in one half-block
                let block_size = stride * 2;   // total block size for variable i
                let mut new_current = Vec::with_capacity(current.len() / 2);
                // Process the evaluation vector in chunks of block_size.
                for chunk in current.chunks_exact(block_size) {
                    let (first, second) = chunk.split_at(stride);
                    for j in 0..stride {
                        // new_value = (1 - v)*first[j] + v*second[j]
                        let new_val = (F::one() - v) * first[j] + v * second[j];
                        new_current.push(new_val);
                    }
                }
                current = new_current;
            }
            // If assignments[i] is None, leave that dimension free.
        }
        current
    }

    /// Full evaluation: all variables are fixed.
    /// This is just a wrapper around `partial_evaluate` that expects every assignment to be Some(_).
    pub fn evaluate(&self, assignments: &[F]) -> F {
        let assignments_opt: Vec<Option<F>> = assignments.iter().cloned().map(Some).collect();
        let result = self.partial_evaluate(&assignments_opt);
        if result.len() != 1 {
            panic!("Full evaluation did not collapse to a single value");
        }
        result[0]
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_ff::{One, Zero};

    /// Test that the representation is valid.
    #[test]
    fn test_representation() {
        // For 2 variables, we need 2^2 = 4 evaluations.
        let evaluations = vec![
            Fr::from(1),
            Fr::from(2),
            Fr::from(3),
            Fr::from(4),
        ];
        let _poly = MultilinearPoly::new(2, evaluations);
    }

    /// Test full evaluation on a 2-variable polynomial.
    /// We assume the ordering (with variable 0 as LSB):
    /// index 0: (0,0), index 1: (1,0), index 2: (0,1), index 3: (1,1)
    #[test]
    fn test_full_evaluation_2var() {
        let evaluations = vec![
            Fr::from(1), // f(0,0)
            Fr::from(2), // f(1,0)
            Fr::from(3), // f(0,1)
            Fr::from(4), // f(1,1)
        ];
        let poly = MultilinearPoly::new(2, evaluations);

        // Evaluate at (0,0)
        assert_eq!(poly.evaluate(&[Fr::zero(), Fr::zero()]), Fr::from(1));
        // Evaluate at (1,0)
        assert_eq!(poly.evaluate(&[Fr::one(),  Fr::zero()]), Fr::from(2));
        // Evaluate at (0,1)
        assert_eq!(poly.evaluate(&[Fr::zero(), Fr::one()]),  Fr::from(3));
        // Evaluate at (1,1)
        assert_eq!(poly.evaluate(&[Fr::one(),  Fr::one()]),  Fr::from(4));
    }

    /// Test partial evaluation on a 2-variable polynomial by fixing the second variable.
    /// Fix variable 1 to 0, so new table should be [f(0,0), f(1,0)] = [1, 2].
    #[test]
    fn test_partial_evaluation_2var_fix_second() {
        let evaluations = vec![
            Fr::from(1),
            Fr::from(2),
            Fr::from(3),
            Fr::from(4),
        ];
        let poly = MultilinearPoly::new(2, evaluations);

        let partial = poly.partial_evaluate(&[None, Some(Fr::zero())]);
        assert_eq!(partial.len(), 2);
        assert_eq!(partial[0], Fr::from(1));
        assert_eq!(partial[1], Fr::from(2));
    }

    /// Test partial evaluation on a 2-variable polynomial by fixing the first variable to 1/2.
    /// Then new evaluations should be computed as weighted averages.
    #[test]
    fn test_partial_evaluation_2var_fix_first() {
        let evaluations = vec![
            Fr::from(1), // f(0,0)
            Fr::from(2), // f(1,0)
            Fr::from(3), // f(0,1)
            Fr::from(4), // f(1,1)
        ];
        let poly = MultilinearPoly::new(2, evaluations);

        let half = Fr::one() / Fr::from(2);
        let partial = poly.partial_evaluate(&[Some(half), None]);

        // For the first block [1,2]: (1-1/2)*1 + 1/2*2 = 0.5*1 + 0.5*2 = 1.5
        // For the second block [3,4]: (1-1/2)*3 + 1/2*4 = 0.5*3 + 0.5*4 = 3.5
        let exp0 = Fr::from(3) / Fr::from(2); // 3/2
        let exp1 = Fr::from(7) / Fr::from(2); // 7/2

        assert_eq!(partial.len(), 2);
        assert_eq!(partial[0], exp0);
        assert_eq!(partial[1], exp1);
    }

    /// Test full partial evaluation on a 3-variable polynomial.
    /// Fixing all variables should yield a vector of length 1.
    #[test]
    fn test_partial_evaluation_full_3var() {
        // For 3 variables, ordering (with variable 0 as LSB) is:
        // (0,0,0)=1, (1,0,0)=2, (0,1,0)=3, (1,1,0)=4,
        // (0,0,1)=5, (1,0,1)=6, (0,1,1)=7, (1,1,1)=8.
        let evaluations = vec![
            Fr::from(1), Fr::from(2), Fr::from(3), Fr::from(4),
            Fr::from(5), Fr::from(6), Fr::from(7), Fr::from(8),
        ];
        let poly = MultilinearPoly::new(3, evaluations);

        // Evaluate at (1,0,1).
        // With our ordering, (1,0,1) corresponds to index 1 + 0*2 + 1*4 = 5, so expected value is 6.
        let partial = poly.partial_evaluate(&[Some(Fr::one()), Some(Fr::zero()), Some(Fr::one())]);
        assert_eq!(partial.len(), 1);

        let full_eval = poly.evaluate(&[Fr::one(), Fr::zero(), Fr::one()]);
        assert_eq!(full_eval, partial[0]);
        // For clarity, we also check that the full evaluation equals 6.
        assert_eq!(full_eval, Fr::from(6));
    }

    /// Test that the constructor panics when given an evaluation vector
    /// whose length does not equal 2^(number of variables).
    #[test]
    #[should_panic(expected = "Not a valid Boolean hypercube evaluation!")]
    fn test_invalid_new() {
        let evaluations = vec![Fr::from(1), Fr::from(2)]; // Only 2 entries for 2 variables.
        let _poly = MultilinearPoly::new(2, evaluations);
    }
}
