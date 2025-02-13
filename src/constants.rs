/// First 64 bytes of the BLAKE2s input during group hash.
/// This is chosen to be some random string that we couldn't have anticipated when we designed
/// the algorithm, for rigidity purposes.
/// We deliberately use an ASCII hex string of 32 bytes here.
pub const GH_FIRST_BLOCK: &'static [u8; 64]
          = b"096b36a5804bfacef1691e173c366a47ff5ba84a44f26ddd7e8d9f79d5b42df0";

// BLAKE2s invocation personalizations
/// BLAKE2s Personalization for CRH^ivk = BLAKE2s(ak | nk)
pub const CRH_IVK_PERSONALIZATION: &'static [u8; 8]
          = b"Zcashivk";

/// BLAKE2s Personalization for PRF^nf = BLAKE2s(nk | rho)
pub const PRF_NF_PERSONALIZATION: &'static [u8; 8]
          = b"Zcash_nf";

// Group hash personalizations
/// BLAKE2s Personalization for Pedersen hash generators.
pub const PEDERSEN_HASH_GENERATORS_PERSONALIZATION: &'static [u8; 8]
          = b"Zcash_PH";

/// BLAKE2s Personalization for the group hash for key diversification
pub const KEY_DIVERSIFICATION_PERSONALIZATION: &'static [u8; 8]
          = b"Zcash_gd";

/// BLAKE2s Personalization for the spending key base point
pub const SPENDING_KEY_GENERATOR_PERSONALIZATION: &'static [u8; 8]
          = b"Zcash_G_";

/// BLAKE2s Personalization for the proof generation key base point
pub const PROOF_GENERATION_KEY_BASE_GENERATOR_PERSONALIZATION: &'static [u8; 8]
          = b"Zcash_H_";

/// BLAKE2s Personalization for the value commitment generator for the value
pub const VALUE_COMMITMENT_GENERATOR_PERSONALIZATION: &'static [u8; 8]
          = b"Zcash_cv";

/// BLAKE2s Personalization for the nullifier position generator (for computing rho)
pub const NULLIFIER_POSITION_IN_TREE_GENERATOR_PERSONALIZATION: &'static [u8; 8]
          = b"Zcash_J_";

/// BLAKE2s Personalization hash of (R_x || message) in EdDSA variant with 256 bit hash
pub const MATTER_EDDSA_BLAKE2S_PERSONALIZATION: &'static [u8; 8] 
            = b"Matter_H";

/// Eth block hash for block 10M
pub const ETH_BLOCK_10_000_000_HASH: &'static str
            = "aa20f7bde5be60603f11a45fc4923aab7552be775403fc00c2e6b805e6297dbe";


/// DST constant for multiexp function
pub const MULTIEXP_DST: &'static [u8; 8]  = b"Multiexp";
  
use crate::bellman::pairing::{Engine, GenericCurveAffine, GenericCurveProjective};
use crate::byteorder::{BigEndian, ReadBytesExt};
  
pub fn make_random_points_with_unknown_discrete_log_from_seed<G: GenericCurveProjective + rand::Rand>(
    dst: &[u8],
    seed: &[u8],
    num_points: usize
) -> Vec<G::Affine> {
    let mut result = vec![];

    use rand::{Rng, SeedableRng};
    use rand::chacha::ChaChaRng;
    // Create an RNG based on the outcome of the random beacon
    let mut rng = {
        // if we use Blake hasher
        let input: Vec<u8> = dst.iter().chain(seed.iter()).cloned().collect();
        let h = blake2s_simd::blake2s(&input);
        assert!(h.as_bytes().len() == 32);
        let mut seed = [0u32; 8];
        for (i, chunk) in h.as_bytes().chunks_exact(8).enumerate() {
            seed[i] = (&chunk[..]).read_u32::<BigEndian>().expect("digest is large enough for this to work");
        }

        ChaChaRng::from_seed(&seed)
    };

    for _ in 0..num_points {
        let point: G = rng.gen();

        result.push(point.into_affine());
    }

    result
}
  
pub fn make_random_points_with_unknown_discrete_log<E: Engine>(
    dst: &[u8],
    num_points: usize
) -> Vec<E::G1Affine> {
    make_random_points_with_unknown_discrete_log_from_seed::<E::G1>(
        dst, 
        &hex::decode(crate::constants::ETH_BLOCK_10_000_000_HASH).unwrap(),
        num_points
    )
}

pub fn make_random_points_with_unknown_discrete_log_generic<G: GenericCurveProjective + rand::Rand>(
    dst: &[u8],
    num_points: usize
) -> Vec<G::Affine> {
    make_random_points_with_unknown_discrete_log_from_seed::<G>(
        dst, 
        &hex::decode(crate::constants::ETH_BLOCK_10_000_000_HASH).unwrap(),
        num_points
    )
}
  