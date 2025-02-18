use ark_ff::PrimeField;

#[derive(Debug, Clone)]
struct MultilinearPoly<F: PrimeField> {
    num_vars: usize,
    evaluations: Vec<F>,
}

impl<F: PrimeField> MultilinearPoly<F> {
    // fn new(num_vars: usize, evaluations: Vec<F>) -> Result<Self, &'static str> {
        fn new(num_vars: usize, evaluations: Vec<F>) -> Self {
        // if 2_usize.pow(num_vars as u32) != evaluations.len() {
        if evaluations.len() != 1 << num_vars {
            // return Err("Not a valid Boolean hypercube evaluation!");
            panic!("Not a valid Boolean hypercube evaluation!");
        };

        // Ok(Self {
        //     num_vars,
        //     evaluations,
        // })

        Self { num_vars, evaluations }
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

        // Process each variable one by one.
        // We assume that the ordering of `current` is such that variable i (with 0-index)
        // appears in contiguous blocks of size 2^(i+1).
        for i in 0..num_vars {
            if let Some(v) = assignments[i] {
                // For a fixed variable, we "collapse" that dimension.
                // The block size for variable i is 2^(i+1).
                // In each block, the first half corresponds to the variable being 0,
                // and the second half corresponds to it being 1.
                let stride = 1 << i;         // number of entries in one half-block
                let block_size = stride * 2;   // total block size
                let mut new_current = Vec::with_capacity(current.len() / 2);
                // Process the evaluation vector in chunks of block_size.
                for chunk in current.chunks_exact(block_size) {
                    // Split the chunk into two halves:
                    let first = &chunk[..stride];     // corresponding to bit = 0
                    let second = &chunk[stride..];      // corresponding to bit = 1
                    // For each paired entry, compute the weighted combination.
                    for j in 0..stride {
                        // new_value = (1 - v)*first[j] + v*second[j]
                        let new_val = (F::one() - v) * first[j] + v * second[j];
                        new_current.push(new_val);
                    }
                }
                // Replace current with the collapsed vector.
                current = new_current;
            }
            // If assignments[i] is None, we leave that dimension free (do nothing).
        }
        current
    }

    /// Full evaluation: all variables are fixed.
    /// This is just a wrapper around `partial_evaluate` that expects every assignment to be Some(_).
    pub fn evaluate(&self, assignments: &[F]) -> F {
        // Convert a slice of F into a slice of Some(F)
        let assignments_opt: Vec<Option<F>> = assignments.iter().cloned().map(Some).collect();
        let result = self.partial_evaluate(&assignments_opt);
        if result.len() != 1 {
            panic!("Full evaluation did not collapse to a single value");
        }
        result[0]
    }

    // fn partial_evaluate() {
    //     todo!()
    // }

    // fn evaluate() {
    //     todo!()
    // }

    fn evaluate_and_interpolate() {
        todo!()
    }
}


#[cfg(test)]
mod test {
    use crate::MultilinearPoly;
    use ark_bn254::Fq;
    use ark_ff::{One, Zero};

    fn poly_1() -> MultilinearPoly<Fq> {
        MultilinearPoly::new(2, vec![Fq::from(0), Fq::from(2), Fq::from(3), Fq::from(5)])
        // MultilinearPoly::new(2, vec![Fq::from(0), Fq::from(2), Fq::from(3), Fq::from(5)]).expect("Not a valid Boolean hypercube evaluation!")
    }

    #[test]
    fn test_representation () {
        assert_eq!(poly_1().evaluations,
            vec![Fq::from(0), Fq::from(2), Fq::from(3), Fq::from(5)]);
    }

    #[test]
    #[should_panic(expected = "Not a valid Boolean hypercube evaluation!")]
    fn test_panic_invalid_representation() {
        MultilinearPoly::new(2, vec![Fq::from(0), Fq::from(1), Fq::from(2), Fq::from(3), Fq::from(5)]);
    }

    /// Test full evaluation on a 2-variable polynomial.
    /// The evaluation vector is assumed to be in the order:
    /// index 0: (0,0), index 1: (1,0), index 2: (0,1), index 3: (1,1)
    #[test]
    fn test_full_evaluation_2var() {
        // Let our Boolean function be defined by:
        // f(0,0) = 1, f(1,0) = 2, f(0,1) = 3, f(1,1) = 4.
        let evaluations = vec![
            Fq::from(1u64),
            Fq::from(2u64),
            Fq::from(3u64),
            Fq::from(4u64),
        ];
        let poly = MultilinearPoly::new(2, evaluations);

        // For full evaluation, passing Boolean values should pick the corresponding entry.
        // Our ordering is that variable 0 is the least significant bit:
        // (x0, x1): (0,0) -> index 0, (1,0) -> index 1, (0,1) -> index 2, (1,1) -> index 3.
        assert_eq!(poly.evaluate(&[Fq::zero(), Fq::zero()]), Fq::from(1u64));
        assert_eq!(poly.evaluate(&[Fq::one(),  Fq::zero()]), Fq::from(2u64));
        assert_eq!(poly.evaluate(&[Fq::zero(), Fq::one()]),  Fq::from(3u64));
        assert_eq!(poly.evaluate(&[Fq::one(),  Fq::one()]),  Fq::from(4u64));
    }

    /// Test partial evaluation on a 2-variable polynomial by fixing the second variable.
    /// Fixing variable 1 to 0 should leave a new evaluation table of length 2,
    /// corresponding to f(x0,0) = [f(0,0), f(1,0)].
    #[test]
    fn test_partial_evaluation_2var_fix_second() {
        let evaluations = vec![
            Fq::from(1u64),
            Fq::from(2u64),
            Fq::from(3u64),
            Fq::from(4u64),
        ];
        let poly = MultilinearPoly::new(2, evaluations);

        // Fix variable 1 (the second variable) to 0 and leave variable 0 free.
        let partial = poly.partial_evaluate(&[None, Some(Fq::zero())]);

        // Expected new evaluation vector: [f(0,0), f(1,0)] = [1, 2]
        assert_eq!(partial.len(), 2);
        assert_eq!(partial[0], Fq::from(1u64));
        assert_eq!(partial[1], Fq::from(2u64));
    }

    /// Test partial evaluation on a 2-variable polynomial by fixing the first variable to a non-Boolean value.
    /// Fix variable 0 to 1/2 and leave variable 1 free.
    /// For each block corresponding to variable 0:
    ///   new_value = (1 - 1/2)*value_when_0 + (1/2)*value_when_1.
    #[test]
    fn test_partial_evaluation_2var_fix_first() {
        let evaluations = vec![
            Fq::from(1u64), // f(0,0)
            Fq::from(2u64), // f(1,0)
            Fq::from(3u64), // f(0,1)
            Fq::from(4u64), // f(1,1)
        ];
        let poly = MultilinearPoly::new(2, evaluations);

        // Fix variable 0 (first variable) to 1/2, leave variable 1 free.
        // For the first block [1,2]: new_value = (1 - 1/2)*1 + (1/2)*2 = 0.5*1 + 0.5*2 = 1.5.
        // For the second block [3,4]: new_value = (1 - 1/2)*3 + (1/2)*4 = 0.5*3 + 0.5*4 = 3.5.
        let half = Fq::from(1u64) / Fq::from(2u64);
        let partial = poly.partial_evaluate(&[Some(half), None]);

        let exp0 = Fq::from(3u64) / Fq::from(2u64); // 3/2 = 1.5
        let exp1 = Fq::from(7u64) / Fq::from(2u64); // 7/2 = 3.5

        assert_eq!(partial.len(), 2);
        assert_eq!(partial[0], exp0);
        assert_eq!(partial[1], exp1);
    }

    /// Test full partial evaluation on a 3-variable polynomial.
    /// Fixing all variables should yield a vector of length 1.
    #[test]
    fn test_partial_evaluation_full_3var() {
        // Define a 3-variable polynomial with evaluations for the 8 hypercube points:
        // Ordering: (0,0,0), (1,0,0), (0,1,0), (1,1,0), (0,0,1), (1,0,1), (0,1,1), (1,1,1)
        let evaluations = vec![
            Fq::from(1u64), Fq::from(2u64), Fq::from(3u64), Fq::from(4u64),
            Fq::from(5u64), Fq::from(6u64), Fq::from(7u64), Fq::from(8u64),
        ];
        let poly = MultilinearPoly::new(3, evaluations);

        // Full evaluation: fix all variables, e.g., evaluate at (1,0,1).
        let partial = poly.partial_evaluate(&[Some(Fq::one()), Some(Fq::zero()), Some(Fq::one())]);
        assert_eq!(partial.len(), 1);

        // Verify that full evaluation via evaluate() returns the same result.
        let full_eval = poly.evaluate(&[Fq::one(), Fq::zero(), Fq::one()]);
        assert_eq!(full_eval, partial[0]);
    }

    /// Test that the constructor panics when given an evaluation vector
    /// whose length does not equal 2^(number of variables).
    #[test]
    #[should_panic(expected = "Not a valid Boolean hypercube evaluation!")]
    fn test_invalid_new() {
        // For 2 variables, we expect 2^2 = 4 evaluations.
        let evaluations = vec![Fq::from(1u64), Fq::from(2u64)]; // only 2 entries
        let _poly = MultilinearPoly::new(2, evaluations);
    }
}