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
    
    pub fn partial_evaluate(&self, (position, value): (usize, F)) -> Self {
        if position >= self.num_vars {
            panic!(
                "Position {} is out of range for a polynomial with {} variables",
                position, self.num_vars
            );
        }

        // Use our helper to split the current evaluation vector based on the bit at `position`.
        let (vec0, vec1) = Self::get_paired_evals(&self.evaluations, position);

        // For each pair, compute the combined value.
        let new_evaluations: Vec<F> = vec0
            .into_iter()
            .zip(vec1.into_iter())
            .map(|(a, b)| (F::one() - value) * a + value * b)
            .collect();

        // Return a new MultilinearPoly with one fewer free variable.
        MultilinearPoly {
            evaluations: new_evaluations,
            num_vars: self.num_vars - 1,
        }
    }

    // fn evaluate() {
    //     todo!()
    // }

    fn interpolate() {
        todo!()
    }
    
    fn get_paired_evals(evals: &[F], var_index: usize) -> (Vec<F>, Vec<F>) {
        let total = evals.len();
        let mut vec0 = Vec::with_capacity(total / 2);
        let mut vec1 = Vec::with_capacity(total / 2);
        for (i, &val) in evals.iter().enumerate() {
            if ((i >> var_index) & 1) == 0 {
                vec0.push(val);
            } else {
                vec1.push(val);
            }
        }
        (vec0, vec1)
    }

    fn print_hypercube()  {
        j
    }
}

#[cfg(test)]
mod test {
    use crate::MultilinearPoly;
    use ark_bn254::Fq;

    fn poly_1() -> MultilinearPoly<Fq> {
        MultilinearPoly::new(2, vec![Fq::from(0), Fq::from(2), Fq::from(3), Fq::from(5)])
        // MultilinearPoly::new(2, vec![Fq::from(0), Fq::from(2), Fq::from(3), Fq::from(5)]).expect("Not a valid Boolean hypercube evaluation!")
    }

    #[test]
    fn test_representation () {
        assert_eq!(
            poly_1().evaluations,
            vec![Fq::from(0), Fq::from(2), Fq::from(3), Fq::from(5)]
        );
    }

    #[test]
    #[should_panic(expected = "Not a valid Boolean hypercube evaluation!")]
    fn test_panic_invalid_representation() {
        MultilinearPoly::new(2, vec![Fq::from(0), Fq::from(1), Fq::from(2)]);
    }
}