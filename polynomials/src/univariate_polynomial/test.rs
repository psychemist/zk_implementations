
#[cfg(test)]
mod test {
    use crate::univariate_polynomial::univariate::UnivariatePolyDense;
    use crate::univariate_polynomial::univariate::UnivariatePolySparse;
    use ark_bn254::Fq;
    use ark_bn254::Fr;

    fn poly_1() -> UnivariatePolyDense<Fq> {
        UnivariatePolyDense::new(vec![Fq::from(1), Fq::from(2), Fq::from(3)])
    }

    fn poly_2() -> UnivariatePolyDense<Fq> {
        UnivariatePolyDense::new(
            [
                vec![Fq::from(3), Fq::from(4)],
                vec![Fq::from(0); 9],
                vec![Fq::from(5)],
            ]
            .concat(),
        )
    }

    fn poly_3() -> UnivariatePolySparse<Fr> {
        UnivariatePolySparse::new(vec![(Fr::from(3), 2), (Fr::from(2), 1), (Fr::from(1), 0)])
    }

    fn poly_4() -> UnivariatePolySparse<Fr> {
        UnivariatePolySparse::new(vec![(Fr::from(5), 11), (Fr::from(4), 1), (Fr::from(3), 0)])
    }

    #[test]
    fn test_degree_dense() {
        assert_eq!(poly_1().degree(), 2);
    }

    #[test]
    fn test_evaluate_dense() {
        assert_eq!(poly_1().evaluate(Fq::from(2)), Fq::from(17));
    }

    #[test]
    fn test_add_dense() {
        assert_eq!(
            (&poly_1() + &poly_2()).coefficient,
            [
                vec![Fq::from(4), Fq::from(6), Fq::from(3)],
                vec![Fq::from(0); 8],
                vec![Fq::from(5)]
            ]
            .concat()
        )
    }

    #[test]
    fn test_multiply_dense() {
        let poly_1 = UnivariatePolyDense::new(vec![Fq::from(5), Fq::from(0), Fq::from(2)]);
        let poly_2 = UnivariatePolyDense::new(vec![Fq::from(6), Fq::from(2)]);

        assert_eq!(
            (&poly_1 * &poly_2).coefficient,
            vec![Fq::from(30), Fq::from(10), Fq::from(12), Fq::from(4)]
        );
    }

    #[test]
    fn test_interpolate_dense() {
        let maybe_2x = UnivariatePolyDense::interpolate(
            vec![Fq::from(2), Fq::from(4)],
            vec![Fq::from(4), Fq::from(8)],
        );
        assert_eq!(maybe_2x.coefficient, vec![Fq::from(0), Fq::from(2)]);
    }

    #[test]
    fn test_degree_sparse() {
        assert_eq!(poly_4().degree(), 11);
    }

    #[test]
    fn test_evaluate_sparse() {
        assert_eq!(poly_4().evaluate(Fr::from(2)), Fr::from(10251));
    }

    
    #[test]
    fn test_add_sparse() {
        assert_eq!(
            (&poly_3() + &poly_4()).coefficient,
            vec![
                (Fr::from(5), 11),
                (Fr::from(3), 2),
                (Fr::from(6), 1),
                (Fr::from(4), 0)
            ]
        );
    }

    #[test]
    fn test_multiply_sparse() {
        let poly_1 = UnivariatePolySparse::new(vec![(Fr::from(2), 2), (Fr::from(5), 0)]);
        let poly_2 = UnivariatePolySparse::new(vec![(Fr::from(2), 1), (Fr::from(6), 0)]);

        assert_eq!(
            (&poly_1 * &poly_2).coefficient,
            vec![
                (Fr::from(4), 3),
                (Fr::from(12), 2),
                (Fr::from(10), 1),
                (Fr::from(30), 0)
            ]
        );
    }

    #[test]
    fn test_interpolate_sparse() {
        let double = UnivariatePolySparse::interpolate(
            vec![Fr::from(2), Fr::from(4)],
            vec![Fr::from(4), Fr::from(8)],
        );
        assert_eq!(double.coefficient, vec![(Fr::from(2), 1)]);
    }
}