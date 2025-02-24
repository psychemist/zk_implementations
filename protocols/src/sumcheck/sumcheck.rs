// use crate::fiat_shamir::transcript::Transcript;
// use ark_ff::{BigInteger, PrimeField};
// use polynomials::multilinear_polynomial::multilinear::MultilinearPoly;
// use sha3::Keccak256;

// /// A proof generated by the sum-check protocol with two evaluation points per round.
// #[derive(Clone, Debug)]
// pub struct Proof<F: PrimeField> {
//     pub claimed_sum: F,
//     pub round_polys: Vec<[F; 2]>,
// }

// /// A proof generated by a modified sum-check protocol (e.g. for a GKR‐style protocol)
// /// that uses three evaluation points per round.
// #[derive(Clone, Debug)]
// pub struct PartialProof<F: PrimeField> {
//     pub claimed_sum: F,
//     pub round_polys: Vec<[F; 3]>,
// }

// /// Helper: convert a slice of field elements into a vector of bytes (big-endian).
// fn absorb_bytes<F: PrimeField, I: IntoIterator<Item = F>>(elements: I) -> Vec<u8> {
//     elements
//         .into_iter()
//         .flat_map(|f| f.into_bigint().to_bytes_be())
//         .collect()
// }

// /// Runs the sum-check protocol prover. It takes as input a multilinear polynomial `poly` and
// /// a claimed sum (the “public” sum) and returns a proof that consists of a sequence of round
// /// polynomials. (Each round polynomial has two evaluations.)
// pub fn prove<F: PrimeField>(poly: &MultilinearPoly<F>, claimed_sum: F) -> Proof<F> {
//     let mut transcript = Transcript::<Keccak256, F>::new();

//     // Absorb the public inputs: the polynomial’s evaluation table and the claimed sum.
//     let poly_bytes = absorb_bytes(poly.evals.iter().copied());
//     transcript.absorb(&poly_bytes);
//     transcript.absorb(&claimed_sum.into_bigint().to_bytes_be());

//     let mut current_poly = poly.clone();
//     let mut rounds = Vec::with_capacity(current_poly.n_vars);

//     // For each variable in the polynomial, produce a round polynomial.
//     for _ in 0..current_poly.n_vars {
//         // Compute the sum of evaluations after partially evaluating at 0 and 1.
//         let sum0: F = current_poly
//             .partial_evaluate((current_poly.n_vars - 1, F::zero()))
//             .evals
//             .iter()
//             .copied()
//             .sum();
//         let sum1: F = current_poly
//             .partial_evaluate((current_poly.n_vars - 1, F::one()))
//             .evals
//             .iter()
//             .copied()
//             .sum();

//         let round_poly = [sum0, sum1];
//         let round_bytes = absorb_bytes(round_poly.iter().copied());
//         transcript.absorb(&round_bytes);
//         rounds.push(round_poly);

//         // Squeeze a new challenge and update the polynomial.
//         let challenge = transcript.squeeze();
//         current_poly = current_poly.partial_evaluate((current_poly.n_vars - 1, challenge));
//     }

//     Proof { claimed_sum, round_polys: rounds }
// }

// /// Runs a variant of the sum-check prover (e.g. for a GKR protocol) where each round polynomial
// /// is given at three points: 0, 1, and 2.
// pub fn partial_prove<F: PrimeField>(
//     poly: &MultilinearPoly<F>,
//     claimed_sum: F,
//     transcript: &mut Transcript<Keccak256, F>,
// ) -> PartialProof<F> {
//     // Absorb the public inputs.
//     let poly_bytes = absorb_bytes(poly.evals.iter().copied());
//     transcript.absorb(&poly_bytes);
//     transcript.absorb(&claimed_sum.into_bigint().to_bytes_be());

//     let mut current_poly = poly.clone();
//     let mut rounds = Vec::with_capacity(current_poly.n_vars);

//     for _ in 0..current_poly.n_vars {
//         let sum0: F = current_poly
//             .partial_evaluate((current_poly.n_vars - 1, F::zero()))
//             .evals
//             .iter()
//             .copied()
//             .sum();
//         let sum1: F = current_poly
//             .partial_evaluate((current_poly.n_vars - 1, F::one()))
//             .evals
//             .iter()
//             .copied()
//             .sum();
//         let sum2: F = current_poly
//             .partial_evaluate((current_poly.n_vars - 1, F::from(2)))
//             .evals
//             .iter()
//             .copied()
//             .sum();

//         let round_poly = [sum0, sum1, sum2];
//         let round_bytes = absorb_bytes(round_poly.iter().copied());
//         transcript.absorb(&round_bytes);
//         rounds.push(round_poly);

//         let challenge = transcript.squeeze();
//         current_poly = current_poly.partial_evaluate((current_poly.n_vars - 1, challenge));
//     }

//     PartialProof { claimed_sum, round_polys: rounds }
// }

// /// Verifies a sum-check proof. It returns `true` if the proof is valid.
// /// The verifier recomputes challenges and uses a final check that the final value equals the
// /// evaluation of the polynomial at those challenges.
// pub fn verify<F: PrimeField>(proof: &Proof<F>, poly: &mut MultilinearPoly<F>) -> bool {
//     if proof.round_polys.len() != poly.n_vars {
//         return false;
//     }

//     let mut transcript = Transcript::<Keccak256, F>::new();
//     transcript.absorb(&absorb_bytes(poly.evals.iter().copied()));
//     transcript.absorb(&proof.claimed_sum.into_bigint().to_bytes_be());

//     let mut computed_sum = proof.claimed_sum;
//     let mut challenges = Vec::with_capacity(proof.round_polys.len());

//     for round_poly in &proof.round_polys {
//         let round_total: F = round_poly.iter().copied().sum();
//         if computed_sum != round_total {
//             return false;
//         }

//         transcript.absorb(&absorb_bytes(round_poly.iter().copied()));
//         let challenge = transcript.squeeze();
//         challenges.push(challenge);

//         // Update the claimed sum using the linear combination.
//         computed_sum = round_poly[0] + challenge * (round_poly[1] - round_poly[0]);
//     }

//     // Final check: the polynomial evaluated at the challenge points must equal computed_sum.
//     poly.evaluate(&challenges) == computed_sum
// }

// /// Verifies a partial sum-check proof. Instead of returning a boolean, it returns the list of
// /// challenges and the final computed sum. (The caller can then compare the final computed sum
// /// to poly.evaluate(challenges).)
// pub fn partial_verify<F: PrimeField>(
//     proof: &PartialProof<F>,
//     poly: &mut MultilinearPoly<F>,
//     transcript: &mut Transcript<Keccak256, F>,
// ) -> (Vec<F>, F) {
//     if proof.round_polys.len() != poly.n_vars {
//         return (vec![], F::zero());
//     }

//     transcript.absorb(&absorb_bytes(poly.evals.iter().copied()));
//     transcript.absorb(&proof.claimed_sum.into_bigint().to_bytes_be());

//     let mut computed_sum = proof.claimed_sum;
//     let mut challenges = Vec::with_capacity(proof.round_polys.len());

//     for round_poly in &proof.round_polys {
//         let round_total: F = round_poly.iter().copied().sum();
//         if computed_sum != round_total {
//             return (vec![], F::zero());
//         }

//         transcript.absorb(&absorb_bytes(round_poly.iter().copied()));
//         let challenge = transcript.squeeze();
//         challenges.push(challenge);

//         computed_sum = round_poly[0] + challenge * (round_poly[1] - round_poly[0]);
//     }

//     (challenges, computed_sum)
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::multilinear_poly::MultilinearPoly;
//     use crate::multilinear_poly::tests::to_field;
//     use ark_bn254::Fr;

//     /// Tests the standard sum-check protocol on a small multilinear polynomial.
//     #[test]
//     fn test_sumcheck_valid() {
//         // Create a polynomial with 3 variables. (The evaluations vector has 2^3 = 8 entries.)
//         let poly = MultilinearPoly::new(to_field(vec![0, 0, 0, 3, 0, 0, 2, 5]), 3);
//         let claimed_sum = Fr::from(10);
//         let proof = prove(&poly, claimed_sum);
//         let mut poly_for_verification = poly.clone();
//         assert!(verify(&proof, &mut poly_for_verification));
//     }

//     /// Tampering with a round polynomial should make verification fail.
//     #[test]
//     fn test_sumcheck_invalid() {
//         let poly = MultilinearPoly::new(to_field(vec![0, 0, 0, 3, 0, 0, 2, 5]), 3);
//         let claimed_sum = Fr::from(10);
//         let mut proof = prove(&poly, claimed_sum);
//         // Tamper with the first round: change the first value.
//         if let Some(first_round) = proof.round_polys.get_mut(0) {
//             first_round[0] = Fr::from(999);
//         }
//         let mut poly_for_verification = poly.clone();
//         assert!(!verify(&proof, &mut poly_for_verification));
//     }

//     /// Test the partial sum-check protocol (with three evaluation points per round)
//     /// and verify that the final computed sum matches the polynomial evaluation.
//     #[test]
//     fn test_partial_sumcheck_valid() {
//         let poly = MultilinearPoly::new(to_field(vec![0, 0, 0, 3, 0, 0, 2, 5]), 3);
//         let claimed_sum = Fr::from(10);
//         let mut transcript = Transcript::<Keccak256, Fr>::new();
//         let partial_proof = partial_prove(&poly, claimed_sum, &mut transcript);

//         // For verification, we create a fresh transcript.
//         let mut verify_transcript = Transcript::<Keccak256, Fr>::new();
//         let mut poly_for_verification = poly.clone();
//         let (challenges, final_sum) = partial_verify(&partial_proof, &mut poly_for_verification, &mut verify_transcript);

//         assert_eq!(poly.evaluate(&challenges), final_sum);
//     }

//     /// Tampering with a partial proof should cause the verification to return an empty challenge list
//     /// or a zero final sum.
//     #[test]
//     fn test_partial_sumcheck_invalid() {
//         let poly = MultilinearPoly::new(to_field(vec![0, 0, 0, 3, 0, 0, 2, 5]), 3);
//         let claimed_sum = Fr::from(10);
//         let mut transcript = Transcript::<Keccak256, Fr>::new();
//         let mut partial_proof = partial_prove(&poly, claimed_sum, &mut transcript);

//         // Tamper with the second round polynomial.
//         if let Some(round_poly) = partial_proof.round_polys.get_mut(1) {
//             round_poly[1] = Fr::from(999);
//         }
//         let mut verify_transcript = Transcript::<Keccak256, Fr>::new();
//         let mut poly_for_verification = poly.clone();
//         let (challenges, final_sum) =
//             partial_verify(&partial_proof, &mut poly_for_verification, &mut verify_transcript);

//         // Since the proof is invalid, we expect an empty challenge list or a zero final sum.
//         assert!(challenges.is_empty() || final_sum.is_zero());
//     }
// }
