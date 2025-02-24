#[cfg(test)]
mod test {
    use crate::fiat_shamir::transcript::Transcript;
    use ark_bn254::Fq;
    use ark_ff::{BigInteger, PrimeField};
    use sha3::digest::core_api::CoreWrapper;
    use sha3::{Digest, Keccak256, Keccak256Core};

    #[test]
    fn test_hash() {
        let mut transcript: Transcript<CoreWrapper<Keccak256Core>, Fq> =
            Transcript::new(Keccak256::new());

        transcript.absorb(Fq::from(11).into_bigint().to_bytes_be().as_slice());
        transcript.absorb("zero knowledge".as_bytes());

        let challenge1 = transcript.squeeze();
        let challenge2 = transcript.squeeze();

        dbg!(challenge1);
        dbg!(challenge2);
    }

    #[test]
    fn test_absorb_and_squeeze() {
        let mut transcript: Transcript<CoreWrapper<Keccak256Core>, Fq> =
            Transcript::new(Keccak256::new());

        let element = 42;

        transcript.absorb(&[element]);
        let random_element = transcript.squeeze();

        // verify randomness (hashing)
        assert_ne!(random_element, Fq::from(element));
    }

    #[test]
    fn test_transcript_determinism() {
        let mut first_transcript: Transcript<CoreWrapper<Keccak256Core>, Fq> =
            Transcript::new(Keccak256::new());

        let mut second_transcript: Transcript<CoreWrapper<Keccak256Core>, Fq> =
            Transcript::new(Keccak256::new());

        first_transcript.absorb(b"hello");
        first_transcript.absorb(b"world");
        first_transcript.squeeze();
        first_transcript.absorb(b"psychemist");

        second_transcript.absorb(b"hello");
        second_transcript.absorb(b"world");
        second_transcript.squeeze();
        second_transcript.absorb(b"psychemist");

        assert_eq!(first_transcript.squeeze(), second_transcript.squeeze());
    }

    #[test]
    fn test_sample_challenge_should_absorb_after_sampling() {
        let mut transcript: Transcript<CoreWrapper<Keccak256Core>, Fq> =
            Transcript::new(Keccak256::new());

        let element = 69;

        transcript.absorb(&[element]);
        let random_element = transcript.squeeze();
        let random_element_a = transcript.squeeze();
        let random_element_b = transcript.squeeze();
        let random_element_c = transcript.squeeze();
        let random_element_d = transcript.squeeze();
        let random_element_e = transcript.squeeze();

        dbg!(&random_element);
        dbg!(&random_element_a);
        dbg!(&random_element_b);
        dbg!(&random_element_c);
        dbg!(&random_element_d);
        dbg!(&random_element_e);

        // Verify challenges are different after sampling challenges
        assert_ne!(random_element, random_element_a);
        assert_ne!(random_element_b, random_element_e);
    }

    #[test]
    fn test_transcript_should_iterate_over_sample_challenge() {
        let mut transcript: Transcript<CoreWrapper<Keccak256Core>, Fq> =
            Transcript::new(Keccak256::new());

        let element = 127;

        transcript.absorb(&[element]);
        let random_element = transcript.squeeze();
        let random_element_x = transcript.squeeze_iterator(5);

        dbg!(&random_element);
        dbg!(&random_element_x);

        // Verify challenges are different after sampling challenges iteratively
        assert_ne!(vec![random_element], random_element_x);
    }
}
