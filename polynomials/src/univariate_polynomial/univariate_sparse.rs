
use ark_ff::PrimeField;
use std::iter::{Product, Sum};
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq, Clone)]
pub struct UnivariatePolySparse<F: PrimeField> {
    pub coefficient: Vec<(F, usize)>,
}


impl<F: PrimeField> UnivariatePolySparse<F> {
    pub fn new(coefficient: Vec<(F, usize)>) -> Self {
        UnivariatePolySparse { coefficient }
    }

    pub fn degree(&self) -> usize {
        self.coefficient.first().map(|(_, d)| *d).unwrap_or(0)
    }

    pub fn evaluate(&self, x: F) -> F {
        if self.coefficient.is_empty() {
            return F::zero();
        }

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

    pub fn interpolate(xs: Vec<F>, ys: Vec<F>) -> Self {
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
        let mut result = self.coefficient.clone();
        for (coeff, degree) in &rhs.coefficient {
            match result.iter_mut().find(|(_, d)| *d == *degree) {
                Some((existing_coeff, _)) => *existing_coeff += coeff,
                None => result.push((*coeff, *degree)),
            }
        }

        result.sort_by(|a, b| b.1.cmp(&a.1));
        UnivariatePolySparse::new(result)
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
