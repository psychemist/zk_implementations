use ark_ff::PrimeField;
use std::iter::{Product, Sum};
use std::ops::{Add, Mul};

// ============= STRUCTS =============
#[derive(Debug, PartialEq, Clone)]
pub struct UnivariatePolyDense<F: PrimeField> {
    pub coefficient: Vec<F>,
}

#[derive(Debug, PartialEq, Clone)]
struct UnivariatePolySparse<F: PrimeField> {
    coefficient: Vec<(F, usize)>,
}

// ============= DENSE IMPLEMENTATIONS =============

impl<F: PrimeField> UnivariatePolyDense<F> {
    pub fn new(coefficient: Vec<F>) -> Self {
        UnivariatePolyDense { coefficient }
    }

    fn degree(&self) -> usize {
        if self.coefficient.is_empty() {
            return 0;
        } else {
            return self.coefficient.len() - 1;
        }
    }

    pub fn evaluate(&self, x: F) -> F {
        self.coefficient
            .iter()
            .rev()
            .cloned()
            .reduce(|acc, curr| acc * x + curr)
            .unwrap()
    }

    pub fn scalar_mul(&self, scalar: &F) -> Self {
        UnivariatePolyDense::new(
            self.coefficient
                .iter()
                .map(|coeff| *coeff * *scalar)
                .collect()
        )
    }

    pub fn basis(x: &F, interpolating_set: &[F]) -> Self {
        // numerator
        let numerator: UnivariatePolyDense<F> = interpolating_set
            .iter()
            .filter(|val| *val != x)
            .map(|x_n| UnivariatePolyDense::new(vec![x_n.neg(), F::one()]))
            .product();

        // denominator
        let denominator = F::one() / numerator.evaluate(*x);

        numerator.scalar_mul(&denominator)
    }

    pub fn interpolate(xs: Vec<F>, ys: Vec<F>) -> Self {
        xs.iter()
            .zip(ys.iter())
            .map(|(x, y)| Self::basis(x, &xs).scalar_mul(y))
            .sum()
    }
}

impl<F: PrimeField> Add for &UnivariatePolyDense<F> {
    type Output = UnivariatePolyDense<F>;

    fn add(self, rhs: Self) -> Self::Output {
        let (mut bigger, smaller) = if self.degree() < rhs.degree() {
            (rhs.clone(), self)
        } else {
            (self.clone(), rhs)
        };

        let _ = bigger
            .coefficient
            .iter_mut()
            .zip(smaller.coefficient.iter())
            .map(|(b_coeff, s_coeff)| *b_coeff += s_coeff)
            .collect::<()>();

        UnivariatePolyDense::new(bigger.coefficient)
    }
}

impl<F: PrimeField> Mul for &UnivariatePolyDense<F> {
    type Output = UnivariatePolyDense<F>;

    fn mul(self, rhs: Self) -> Self::Output {
        let new_degree = self.degree() + rhs.degree();
        let mut result = vec![F::zero(); new_degree + 1];
        for i in 0..self.coefficient.len() {
            for j in 0..rhs.coefficient.len() {
                result[i + j] += self.coefficient[i] * rhs.coefficient[j]
            }
        }
        UnivariatePolyDense::new(result)
    }
}

impl<F: PrimeField> Sum for UnivariatePolyDense<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = UnivariatePolyDense::new(vec![F::zero()]);
        for poly in iter {
            result = &result + &poly;
        }
        result
    }
}

impl<F: PrimeField> Product for UnivariatePolyDense<F> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = UnivariatePolyDense::new(vec![F::one()]);
        for poly in iter {
            result = &result * &poly;
        }
        result
    }
}

// ============= SPARSE IMPLEMENTATIONS =============

impl<F: PrimeField> UnivariatePolySparse<F> {
    fn new(coefficient: Vec<(F, usize)>) -> Self {
        UnivariatePolySparse { coefficient }
    }

    fn degree(&self) -> usize {
        if self.coefficient.is_empty() {
            return 0;
        } else {
            return self.coefficient[0].1;
        }
    }

    fn evaluate(&self, x: F) -> F {
        self.coefficient
            .iter()
            .fold(F::zero(), |acc, (coeff, power)| {
                acc + *coeff * x.pow(&[*power as u64])
            })
    }

    fn scalar_mul(&self, scalar: &F) -> Self {
            UnivariatePolySparse::new(
                self.coefficient
                    .iter()
                    .map(|(coeff, degree)| (*coeff * *scalar, *degree))
                    .collect::<Vec<(F, usize)>>()
            )
        }

    fn basis(x: &F, interpolating_set: &[F]) -> Self {
        // numerator
        let numerator: UnivariatePolySparse<F> = interpolating_set
            .iter()
            .filter(|val| *val != x)
            .map(|x_n| UnivariatePolySparse::new(vec![(F::one(), 1), (x_n.neg(), 0)]))
            .product();

        // denominator
        let denominator = F::one() / numerator.evaluate(*x);

        numerator.scalar_mul(&denominator)
    }

    fn interpolate(xs: Vec<F>, ys: Vec<F>) -> Self {
        let sum = xs.iter()
            .zip(ys.iter())
            .map(|(x, y)| Self::basis(x, &xs).scalar_mul(y))
            .sum::<Self>();
        
        UnivariatePolySparse::new(
            sum.coefficient
                .into_iter()
                .filter(|(coeff, deg)| *coeff != F::zero() && *deg != 0)
                .collect()
        )
    }
}

impl<F: PrimeField> Add for &UnivariatePolySparse<F> {
    type Output = UnivariatePolySparse<F>;

    fn add(self, rhs: Self) -> Self::Output {
        let (mut bigger, smaller) = if self.degree() < rhs.degree() {
            (rhs.clone(), self)
        } else {
            (self.clone(), rhs)
        };

        let mut new_coefficients: Vec<(F, usize)> = bigger
            .coefficient
            .iter_mut()
            .map(|(b_coeff, b_degree)| {
            if let Some((s_coeff, _s_degree)) = 
                smaller.coefficient.iter().find(|(_, s_degree)| s_degree == b_degree) {
                *b_coeff += s_coeff;
            }
            (*b_coeff, *b_degree)
            })
            .collect();

        new_coefficients.extend(
            smaller
            .coefficient
            .iter()
            .filter(|(_, s_degree)| !bigger.coefficient.iter().any(|(_, b_degree)| b_degree == s_degree))
            .cloned()
        );

        new_coefficients.sort_by(|a, b| b.1.cmp(&a.1));

        UnivariatePolySparse::new(new_coefficients)
    }
}

impl<F: PrimeField> Mul for &UnivariatePolySparse<F> {
    type Output = UnivariatePolySparse<F>;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result = vec![];
        for i in 0..self.coefficient.len() {
            for j in 0..rhs.coefficient.len() {
                let coeff = self.coefficient[i].0 * rhs.coefficient[j].0;
                let degree = self.coefficient[i].1 + rhs.coefficient[j].1;
                if let Some((existing_coeff, _)) = result.iter_mut().find(|(_, d)| *d == degree) {
                    *existing_coeff += coeff;
                } else {
                    result.push((coeff, degree));
                }
            }
        }
        UnivariatePolySparse::new(result)
    }
}

impl<F: PrimeField> Sum for UnivariatePolySparse<F> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = UnivariatePolySparse::new(vec![(F::zero(), 0)]);
        for poly in iter {
            result = &result + &poly;
        }
        result
    }
}

impl<F: PrimeField> Product for UnivariatePolySparse<F> {
    fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut result = UnivariatePolySparse::new(vec![(F::one(), 0)]);
        for poly in iter {
            result = &result * &poly;
        }
        result
    }
}

// ================== TESTS ==================

#[cfg(test)]
mod test {
    use crate::UnivariatePolyDense;
    use crate::UnivariatePolySparse;
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