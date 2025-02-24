use ark_ff::PrimeField;
use std::iter::{Product, Sum};
use std::ops::{Add, Mul};

// ============= STRUCTS =============
#[derive(Debug, PartialEq, Clone)]
pub struct UnivariatePolyDense<F: PrimeField> {
    pub coefficient: Vec<F>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnivariatePolySparse<F: PrimeField> {
    pub coefficient: Vec<(F, usize)>,
}

// ============= DENSE IMPLEMENTATIONS =============

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

// ============= SPARSE IMPLEMENTATIONS =============

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
