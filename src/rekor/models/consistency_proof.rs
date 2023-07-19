/*
 * Rekor
 *
 * Rekor is a cryptographically secure, immutable transparency log for signed software releases.
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use crate::errors::SigstoreError;
use crate::errors::SigstoreError::{ConsistencyProofError, UnexpectedError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ConsistencyProof {
    /// The hash value stored at the root of the merkle tree at the time the proof was generated
    #[serde(rename = "rootHash")]
    pub root_hash: String,
    #[serde(rename = "hashes")]
    pub hashes: Vec<String>,
}

impl ConsistencyProof {
    pub fn new(root_hash: String, hashes: Vec<String>) -> ConsistencyProof {
        ConsistencyProof { root_hash, hashes }
    }

    pub fn verify(
        &self,
        old_size: usize,
        old_root: &str,
        new_size: usize,
    ) -> Result<(), SigstoreError> {
        use crate::crypto::merkle::{MerkleProofVerifier, Rfc6269Default};

        // decode hashes from hex and convert them to the required data structure
        // immediately return an error when conversion fails
        let proof_hashes = self
            .hashes
            .iter()
            .map(|h| {
                hex::decode(h)
                    .map_err(Into::into) // failed to decode from hex
                    .and_then(|h| {
                        <[u8; 32]>::try_from(h).map_err(|err| UnexpectedError(format!("{err:?}")))
                    })
                    .map(Into::into)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let old_root = hex::decode(old_root)
            .map_err(Into::into)
            .and_then(|h| {
                <[u8; 32]>::try_from(h).map_err(|err| UnexpectedError(format!("{err:?}")))
            })
            .map(Into::into)?;

        let new_root = hex::decode(&self.root_hash)
            .map_err(Into::into)
            .and_then(|h| {
                <[u8; 32]>::try_from(h).map_err(|err| UnexpectedError(format!("{err:?}")))
            })
            .map(Into::into)?;

        Rfc6269Default::verify_consistency(old_size, new_size, &proof_hashes, &old_root, &new_root)
            .map_err(ConsistencyProofError)
    }
}
