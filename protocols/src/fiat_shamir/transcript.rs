use ark_ff::PrimeField;
use sha3::{Digest, Keccak256};
use std::marker::PhantomData;

#[derive(Default)]
pub struct Transcript <K: HashFunctionTrait, F: PrimeField> {
    hash_function: K,
    _field: PhantomData<F>,
}

impl <K: HashFunctionTrait, F: PrimeField> Transcript<K, F> {
    pub fn new(hash_function: K) -> Self {
        Self {
            hash_function,
            _field: PhantomData,
        }
    }

    pub fn absorb(&mut self, data: &[u8]) {
        self.hash_function.append(data);
    }

    pub fn squeeze(&mut self) -> F {
        let hash_output = self.hash_function.generate_hash();
        self.absorb(&hash_output);
        F::from_be_bytes_mod_order(&hash_output)
    }

    pub fn squeeze_iterator(&mut self, n: usize) -> Vec<F> {
        (0..n).map(|_| self.squeeze()).collect()
    }
}

pub trait HashFunctionTrait {
    fn append(&mut self, data: &[u8]);
    fn generate_hash(&self) -> Vec<u8>;
}

impl HashFunctionTrait for Keccak256 {
    fn append(&mut self, data: &[u8]) {
        self.update(data)
        // Digest::update(self, data);
    }

    fn generate_hash(&self) -> Vec<u8> {
        self.clone().finalize().to_vec()
    }
}
