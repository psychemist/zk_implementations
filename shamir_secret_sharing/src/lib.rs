use ark_ff::PrimeField;
use rand::rngs::OsRng;
use rand::Rng;
use univariate_polynomials::UnivariatePolyDense;
// use univariate_polynomials::{self, UnivariatePolyDense};

const PRIME = 65537;

struct Share {
    x: usize,
    y: usize
}


fn generate_secret() -> usize {
    let mut rng = OsRng;
    let secret: u16 = rng.gen();

    secret
}

fn generate_coefficients(secret: usize, threshold: usize) -> Vec<usize> {
    let mut rng = OsRng;

    // Initialize polynomial with the secret (mod p)
    let mut coeffs = vec![secret % prime];

    // Generate k-1 random coefficients in [0, p-1]
    for _ in 1..threshold {
        let coeff = rng.gen_range(0..PRIME);
        if 
        coeffs.push(coeff);
    }

    coeffs
}
fn evaluate_polynomial(coefficients: &[usize], x: usize) -> usize {
    let evaluation = UnivariatePolyDense::evaluate();

    coefficients.push(evaluation);
}

fn interpolate_at_zero(points: &[(usize, usize)]) -> usize {
    todo!()
}

fn split_secret(secret: &[usize], threshold: usize, _shares: usize) -> Vec<Share> {
    let secret = generate_secret();
    let coefficients = generate_coefficients(secret, threshold);
    let points = evaluate_polynomial(coefficients);
    
}
fn reconstruct_secret(shares: &[Share], threshold: usize) -> usize {
    todo!()
}

fn validate_inputs(secret: usize, threshold: usize, num_shares: usize) -> Result<(), &'static str> {
    if threshold >= num_shares return -1;
    if secret >= PRIME return -1;
}

fn main() {
    let secret = generate_secret();
    println!("Generated Secret: {}", secret);
}



#[cfg(test)]
mod test {
    use crate::UnivariatePolyDense;
    use ark_bn254::Fq;

    fn poly_1() -> UnivariatePolyDense<Fq> {
        UnivariatePolyDense::new(vec![Fq::from(1), Fq::from(2), Fq::from(3)])
    }

    #[test]
    fn test_evaluate_dense() {
        assert_eq!(poly_1().evaluate(Fq::from(2)), Fq::from(17));
    }

    #[test]

}