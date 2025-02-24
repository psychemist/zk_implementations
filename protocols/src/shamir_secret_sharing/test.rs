#[cfg(test)]
mod tests {
    use crate::shamir_secret_sharing::shamir::Share;
    use ark_bn254::Fq;

    #[test]
    fn test_secret_sharing() {
        type F = Fq;

        // Generate a secret
        let secret: F = Share::create_secret();

        // Generate shares and polynomial coefficients
        let threshold = 3;
        let num_shares = 5;
        let shares = Share::split_secret(secret, threshold, num_shares);

        // Reconstruct the secret
        let reconstructed_secret = Share::reconstruct_secret(&shares[..threshold]);

        // Validate that the reconstructed secret matches the original
        assert_eq!(secret, reconstructed_secret);
    }
}