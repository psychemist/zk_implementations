use ark_ff::PrimeField;
use rand::rngs::OsRng;
use polynomials::univariate_polynomial::univariate::UnivariatePolyDense;
// use univariate_polynomials::{self, UnivariatePolyDense};
 
// const PRIME: F = 65537;

// Define a struct for shares
#[derive(Debug, Clone)]
struct Share<F: PrimeField> {
    x: F,
    y: F,
}

// Generate a random secret
fn create_secret<F: PrimeField>() -> F {
    let mut rng = OsRng;
    // Use F::rand to generate a random field element
    let secret = F::rand(&mut rng);

    secret
}

// Generate polynomial coefficients
fn generate_coefficients<F: PrimeField>(secret: F, threshold: usize) -> UnivariatePolyDense<F> {
    let mut rng = OsRng;

    // Initialize polynomial with the secret
    let mut poly = UnivariatePolyDense::new(vec![secret]);

    // Generate k-1 random coefficients
    for _ in 1..threshold {
        // Generate a random field element
        let coeff = F::rand(&mut rng);
        // Add coeff to polynomial
        poly.coefficient.push(coeff);
    }

    poly
}

// Generate shares by evaluating the polynomial at distinct points
fn split_secret<F: PrimeField>(secret: F, threshold: usize, num_shares: usize) -> Vec<Share<F>> {
    assert!(
        threshold <= num_shares,
        "Threshold must be less than or equal to total shares"
    );

    let polynomial = generate_coefficients(secret, threshold);
    // let mut shares = vec![secret];
    let mut shares = Vec::new();
    // shares.push(Share{ x: F::zero(), y: secret });

    // Create shares of x = degree and y = coefficient @ x
    for i in 1..=num_shares {
        let x = F::from(i as u64);
        let y = polynomial.evaluate(x);
        shares.push(Share { x, y });
    }

    shares
}

// Reconstruct the secret using Lagrange interpolation
fn reconstruct_secret<F: PrimeField>(shares: &[Share<F>]) -> F {
    // assert!(shares.len >= threshold, "Shares and threshold are of unequal length!");

    let password = F::zero();

    // Collect points for interpolation
    let xs: Vec<F> = shares.iter().map(|p| p.x).collect();
    let ys: Vec<F> = shares.iter().map(|p| p.y).collect();

    // Interpolate the points to get the polynomial
    let poly = UnivariatePolyDense::interpolate(xs, ys);

    poly.evaluate(F::from(password))
}


fn main() {
    // Define the field type (e.g., ark_bn254::Fq)
    type F = ark_bn254::Fq;

    // Generate a secret
    let secret: F = create_secret();
    println!("\nGenerated Secret: {}", secret);

    let threshold: usize = 3;
    let num_shares: usize = 5;
    
    // Generate shares
    let shares: Vec<Share<F>> = split_secret(secret, threshold, num_shares);

    // Print shares
    for (i, share) in shares.iter().enumerate() {
        println!("\nShare {}: ({}, {})", i + 1, share.x, share.y);
    }

    // Reconstruct the secret
    let reconstructed_secret = reconstruct_secret(&shares[..threshold]);
    println!("\nReconstructed Secret: {}", reconstructed_secret);

    // Validate that the reconstructed secret matches the original
    assert_eq!(secret, reconstructed_secret);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fq;

    #[test]
    fn test_secret_sharing() {
        type F = Fq;

        // Generate a secret
        let secret = create_secret::<F>();

        // Generate shares and polynomial coefficients
        let threshold = 3;
        let num_shares = 5;
        let shares = split_secret(secret, threshold, num_shares);

        // Reconstruct the secret
        let reconstructed_secret = reconstruct_secret(&shares[..threshold]);

        // Validate that the reconstructed secret matches the original
        assert_eq!(secret, reconstructed_secret);
    }
}