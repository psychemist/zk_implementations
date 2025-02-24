
#[cfg(test)]
mod test {
    use crate::multilinear_polynomial::multilinear::MultilinearPoly;
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
    fn test_invalid_representation() {
        MultilinearPoly::new(2, vec![Fq::from(0), Fq::from(1), Fq::from(2)]);
    }

    #[test]
    fn test_partial_evaluate_a_2v() {
        let poly = MultilinearPoly::<Fq> {
            num_vars: 2,
            evaluations: vec![Fq::from(0), Fq::from(2), Fq::from(0), Fq::from(5)],
        };
        let partial_evaluated_poly = poly.partial_evaluate((1, Fq::from(5)));

        assert_eq!(
            partial_evaluated_poly.evaluations,
            vec![Fq::from(0), Fq::from(17)]
        );
    }

    #[test]
    fn test_partial_evaluate_b_2v() {
        let poly = MultilinearPoly::<Fq> {
            num_vars: 2,
            evaluations: vec![Fq::from(0), Fq::from(2), Fq::from(0), Fq::from(5)],
        };
        let partial_evaluated_poly = poly.partial_evaluate((0, Fq::from(3)));

        assert_eq!(
            partial_evaluated_poly.evaluations,
            vec![Fq::from(6), Fq::from(15)]
        );
    }

    #[test]
    fn test_partial_evaluate_a_3v() {
        let poly_2 = MultilinearPoly::new(
            3,
            vec![
                Fq::from(0),
                Fq::from(0),
                Fq::from(0),
                Fq::from(3),
                Fq::from(0),
                Fq::from(0),
                Fq::from(2),
                Fq::from(5),
            ],
        );
        let result = poly_2.partial_evaluate((2, Fq::from(1)));

        assert_eq!(
            result.evaluations,
            vec![Fq::from(0), Fq::from(0), Fq::from(2), Fq::from(5)]
        );
    }

    #[test]
    fn test_partial_evaluate_b_3v() {
        let poly_2 = MultilinearPoly::new(
            3,
            vec![
                Fq::from(0),
                Fq::from(0),
                Fq::from(0),
                Fq::from(3),
                Fq::from(0),
                Fq::from(0),
                Fq::from(2),
                Fq::from(5),
            ],
        );
        let result = poly_2.partial_evaluate((1, Fq::from(5)));

        assert_eq!(
            result.evaluations,
            vec![Fq::from(0), Fq::from(15), Fq::from(10), Fq::from(25)]
        );
    }

    #[test]
    fn test_partial_evaluate_c_3v() {
        let poly_2 = MultilinearPoly::new(
            3,
            vec![
                Fq::from(0),
                Fq::from(0),
                Fq::from(0),
                Fq::from(3),
                Fq::from(0),
                Fq::from(0),
                Fq::from(2),
                Fq::from(5),
            ],
        );
        let result = poly_2.partial_evaluate((0, Fq::from(3)));

        assert_eq!(
            result.evaluations,
            vec![Fq::from(0), Fq::from(9), Fq::from(0), Fq::from(11)]
        );
    }

    #[test]
    fn test_evaluate_abc() {
        let poly_2 = MultilinearPoly::new(
            3,
            vec![
                Fq::from(0),
                Fq::from(0),
                Fq::from(0),
                Fq::from(3),
                Fq::from(0),
                Fq::from(0),
                Fq::from(2),
                Fq::from(5),
            ],
        );
        let result = poly_2.evaluate(vec![Fq::from(3), Fq::from(5), Fq::from(1)]);

        assert_eq!(result, Fq::from(55));
    }
}