//! In-memory trees for proof fixtures with real Merkle paths.
use num_bigint::BigUint;
use solana_signature::Signature;
use spr_client::{
    MerkleContext, MerkleProof, NonInclusionProof, SpendProof, NULLIFIER_TREE_HEIGHT,
    STATE_TREE_HEIGHT,
};
use spr_hasher::Poseidon;
use spr_interface::pda;
use spr_keypair::ShieldedKeypair;
use spr_merkle_tree::{indexed::IndexedMerkleTree, MerkleTree};
use spr_transaction::{instructions::transact::BN254_MODULUS_DEC, Data, Mint, Utxo, WalletUtxo};

/// As Photon names the trees in a Merkle context.
const STATE_TREE_TYPE: u16 = 1;
const NULLIFIER_TREE_TYPE: u16 = 2;
/// The tree every note of a fixture is on.
pub const TREE_ID: u16 = 0;

/// The state tree a fixture's notes are on and an empty nullifier tree.
pub struct Trees {
    state: MerkleTree<Poseidon>,
    nullifiers: IndexedMerkleTree<Poseidon, usize>,
}

impl Trees {
    pub fn new() -> Self {
        let upper_bound = BigUint::parse_bytes(BN254_MODULUS_DEC.as_bytes(), 10)
            .expect("the BN254 modulus")
            - 1u32;
        Self {
            state: MerkleTree::<Poseidon>::new(STATE_TREE_HEIGHT, 0),
            nullifiers: IndexedMerkleTree::<Poseidon, usize>::new_with_next_value(
                NULLIFIER_TREE_HEIGHT,
                0,
                upper_bound,
            )
            .expect("the nullifier tree"),
        }
    }

    /// `count` SOL notes of `owner`, appended to the state tree.
    pub fn notes(&mut self, owner: &ShieldedKeypair, count: usize) -> Vec<WalletUtxo> {
        let nullifier_pubkey = owner.nullifier_key.pubkey().expect("nullifier pubkey");
        (0..count)
            .map(|index| {
                // 32 bytes of index + 1, below the field from the 49th note
                // on (0x31... is not): the smaller fixtures keep theirs.
                let mut blinding = [index as u8 + 1; 32];
                if blinding[0] >= 0x30 {
                    blinding[0] = 0;
                }
                let utxo = Utxo {
                    owner: owner.signing_pubkey(),
                    asset: Mint::SOL,
                    amount: 1_000 + index as u64,
                    blinding,
                    ring_program_id: None,
                    data: Data::default(),
                };
                let utxo_hash = utxo
                    .hash(&nullifier_pubkey, &[0; 32], &[0; 32], TREE_ID)
                    .expect("utxo hash");
                self.state.append(&utxo_hash).expect("append the note");
                WalletUtxo {
                    nullifier: owner
                        .nullifier_key
                        .nullifier(&utxo_hash, &utxo.blinding)
                        .expect("nullifier"),
                    utxo,
                    nullifier_pubkey,
                    utxo_hash,
                    tx_viewing_key: None,
                    data_hash: None,
                    ring_data_hash: None,
                    tree_id: TREE_ID,
                    leaf_index: index as u64,
                    latest_tree_id: None,
                    slot: 0,
                    tx_signature: Signature::default(),
                    slot_index: 0,
                }
            })
            .collect()
    }

    pub fn spend_proofs(&self, notes: &[WalletUtxo]) -> Vec<SpendProof> {
        let tree = pda::tree(TREE_ID);
        notes
            .iter()
            .map(|note| SpendProof {
                state: MerkleProof {
                    leaf: note.utxo_hash,
                    merkle_context: MerkleContext {
                        tree_type: STATE_TREE_TYPE,
                        tree,
                    },
                    path: self
                        .state
                        .get_proof_of_leaf(note.leaf_index as usize, true)
                        .expect("merkle proof")
                        .to_vec(),
                    leaf_index: note.leaf_index,
                    root: self.state.root(),
                    root_seq: 0,
                    root_index: 0,
                },
                nullifier: self.non_inclusion(note.nullifier),
            })
            .collect()
    }

    pub fn non_inclusion(&self, nullifier: [u8; 32]) -> NonInclusionProof {
        let proof = self
            .nullifiers
            .get_non_inclusion_proof(&BigUint::from_bytes_be(&nullifier))
            .expect("non-inclusion proof");
        NonInclusionProof {
            leaf: nullifier,
            merkle_context: MerkleContext {
                tree_type: NULLIFIER_TREE_TYPE,
                tree: pda::tree(TREE_ID),
            },
            path: proof.merkle_proof,
            low_element: proof.leaf_lower_range_value,
            low_element_index: proof.leaf_index as u64,
            high_element: proof.leaf_higher_range_value,
            high_element_index: proof.next_index as u64,
            root: proof.root,
            root_seq: 0,
            root_index: 0,
        }
    }
}
