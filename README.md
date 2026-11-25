# ZK Research Cohort Implementations

This repository contains Rust implementations of various Zero-Knowledge Proof (ZKP) protocols and polynomial primitives, developed as part of a ZK Research Cohort. The implementations are built using the [Arkworks](https://arkworks.rs/) ecosystem, specifically targeting the BN254 elliptic curve.

## Overview

The project is organized as a workspace with two primary crates:

- **`polynomials`**: Provides the mathematical foundations, including custom implementations of polynomial structures.
- **`protocols`**: Contains the core ZK protocols and cryptographic schemes built on top of the polynomial primitives.

## Implementations

### 🧮 Polynomials (`/polynomials`)
Foundational structures required for the protocols:
- **Multilinear Polynomials**: Implementation of polynomials in multiple variables where the degree in each variable is at most 1. Crucial for the Sumcheck protocol.
- **Univariate Polynomials**: Basic operations for single-variable polynomials.

### Protocols (`/protocols`)
Interactive and non-interactive protocols:
- **Sumcheck Protocol**: A fundamental interactive proof system that allows a prover to convince a verifier of the sum of a multivariate polynomial over a Boolean hypercube.
- **GKR (Goldwasser-Kalai-Rothblum)**: A protocol for verifying computations expressed as layered arithmetic circuits (IP for log-depth circuits).
- **Shamir's Secret Sharing**: A threshold scheme to split a secret into parts, where the secret can only be reconstructed when a sufficient number of parts are combined.
- **Fiat-Shamir Transform**: A technique used to convert public-coin interactive protocols (like Sumcheck and GKR) into non-interactive Zero-Knowledge arguments.

## Stack

- **Language**: [Rust](https://www.rust-lang.org/)
- **Crypto Library**: [Arkworks](https://arkworks.rs/)
  - `ark-bn254`: For the BN254 elliptic curve (commonly used in Ethereum ZK applications).
  - `ark-ff`: For finite field arithmetic.
- **Utilities**: `rand` for randomness, `sha3` for hashing (likely used in the Fiat-Shamir implementation).

## Getting Started

### Prerequisites
Ensure you have Rust and Cargo installed. You can install them via [rustup.rs](https://rustup.rs/).

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/psychemist/zk_implementations.git
   cd zk_implementations
   ```

2. Build the project:
   ```bash
   cargo build
   ```

### Running Tests
The repository includes unit tests for both the polynomial arithmetic and the cryptographic protocols.

Run all tests:
```bash
cargo test
```

Run tests for a specific crate:
```bash
# Test only the protocols
cargo test -p protocols

# Test only the polynomial implementations
cargo test -p polynomials
```

## Disclaimer

These implementations are intended for **educational and research purposes**. They have not been audited and are **not intended for production use**.