use ark_ff::PrimeField;
use std::iter::{Product, Sum};
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct UnivariatePolyDense<F: PrimeField> {
    pub coefficient: Vec<F>,
}

impl<F: PrimeField> UnivariatePolyDense<F> {
    pub fn new(coefficient: Vec<F>) -> Self {
        UnivariatePolyDense { coefficient }
    }

    pub fn degree(&self) -> usize {
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

    fn scalar_mul(&self, scalar: &F) -> Self {
        UnivariatePolyDense::new(
            self.coefficient
                .iter()
                .map(|coeff| *coeff * *scalar)
                .collect()
        )
    }

    fn basis(x: &F, interpolating_set: &[F]) -> Self {
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
