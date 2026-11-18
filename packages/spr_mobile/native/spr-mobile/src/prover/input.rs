//! Convert the client's assembled fields to canonical zopro field elements.

use num_bigint::BigUint;
use spr_client::prover::{LocalProvingInputs, MergeInputs, TransferInputs, TreeSlotFields};
use zopro_spr as z;

use crate::WalletError;

pub(super) fn convert(input: LocalProvingInputs<'_>) -> Result<z::ProvingInput, WalletError> {
    match input {
        LocalProvingInputs::Transfer(inputs) => transfer(inputs, z::TransferKind::Confidential),
        LocalProvingInputs::TransferRing(inputs) => transfer(inputs, z::TransferKind::Ring),
        LocalProvingInputs::RingAuthority(inputs) => {
            transfer(inputs, z::TransferKind::RingAuthority)
        }
        LocalProvingInputs::Merge(inputs) => merge(inputs, false),
        LocalProvingInputs::MergeRing(inputs) => merge(inputs, true),
    }
}

fn field(value: &BigUint) -> Result<z::Fr, WalletError> {
    if value.bits() > 256 {
        return Err(WalletError::ProofFailed);
    }
    let mut limbs = [0; 4];
    for (limb, digit) in limbs.iter_mut().zip(value.iter_u64_digits()) {
        *limb = digit;
    }
    z::Fr::from_canonical(limbs).ok_or(WalletError::ProofFailed)
}

fn fields(values: &[BigUint]) -> Result<Vec<z::Fr>, WalletError> {
    values.iter().map(field).collect()
}

fn tree_slots(slots: &[TreeSlotFields]) -> Result<Vec<z::TreeSlot>, WalletError> {
    slots
        .iter()
        .map(|slot| {
            Ok(z::TreeSlot {
                id: field(&slot.id)?,
                utxo_root: field(&slot.utxo_root)?,
                nullifier_root: field(&slot.nullifier_root)?,
            })
        })
        .collect()
}

fn element(bytes: &[u8; 32]) -> Result<z::Fr, WalletError> {
    z::Fr::from_be_bytes(bytes).ok_or(WalletError::ProofFailed)
}

fn utxo(input: &spr_client::ProofInputUtxo) -> Result<z::Utxo, WalletError> {
    Ok(z::Utxo {
        domain: element(&input.domain)?,
        owner: element(&input.owner_hash)?,
        asset: element(&input.asset)?,
        amount: element(&input.amount)?,
        blinding: element(&input.blinding)?,
        data_hash: element(&input.data_hash)?,
        ring_data_hash: element(&input.ring_data_hash)?,
        ring_program_id: element(&input.ring_program_id)?,
    })
}

fn transfer(input: &TransferInputs, kind: z::TransferKind) -> Result<z::ProvingInput, WalletError> {
    let inputs = input
        .inputs
        .iter()
        .map(|spend| {
            let secret = spend
                .nullifier_secret
                .as_ref()
                .ok_or(WalletError::ProofFailed)?;
            field(&spend.is_dummy)?;
            Ok(z::TransferInput {
                utxo: utxo(&spend.utxo)?,
                state_path: fields(&spend.state_path_elements)?,
                state_path_index: field(&spend.state_path_index)?,
                nullifier_low_value: field(&spend.nullifier_low_value)?,
                nullifier_next_value: field(&spend.nullifier_next_value)?,
                nullifier_low_path: fields(&spend.nullifier_low_path_elements)?,
                nullifier_low_path_index: field(&spend.nullifier_low_path_index)?,
                tree_slot: field(&spend.tree_slot)?,
                nullifier: field(&spend.nullifier)?,
                owner_pk_hash: field(&spend.owner_pk_hash)?,
                nullifier_secret: field(secret)?,
            })
        })
        .collect::<Result<_, WalletError>>()?;
    let outputs = input
        .outputs
        .iter()
        .map(|output| {
            field(&output.is_dummy)?;
            Ok(z::TransferOutput {
                utxo: utxo(&output.utxo)?,
                hash: field(&output.hash)?,
                owner_pk_hash: field(&output.owner_pk_hash)?,
                nullifier_pk: field(&output.nullifier_pk)?,
            })
        })
        .collect::<Result<_, WalletError>>()?;
    let cache = &input.cache;
    Ok(z::ProvingInput::Transfer(z::TransferInputs {
        kind,
        inputs,
        outputs,
        tree_slots: tree_slots(&input.tree_slots)?,
        output_tree_id: field(&input.output_tree_id)?,
        external_data_hash: field(&input.external_data_hash)?,
        private_tx_hash: field(&input.private_tx_hash)?,
        blinding_seed: field(&input.blinding_seed)?,
        public_assets: fields(&input.public_assets)?,
        public_amounts: fields(&input.public_amounts)?,
        ring_program_id: field(&input.ring_program_id)?,
        signer_pk_hashes: fields(&input.signer_pk_hashes)?,
        input_flags: field(&input.input_flags)?,
        published_output_owner_pk_hashes: fields(&input.published_output_owner_pk_hashes)?,
        cache: z::CacheInputs {
            tree_id: field(&cache.tree_id)?,
            read_hash_chain: field(&cache.read_hash_chain)?,
            read_hashes: fields(&cache.read_hashes)?,
            is_cached: cache
                .is_cached
                .iter()
                .map(|cached| z::Fr::from(u64::from(*cached)))
                .collect(),
            read_index: cache
                .read_index
                .iter()
                .map(|index| z::Fr::from(*index as u64))
                .collect(),
        },
        public_input_hash: field(&input.public_input_hash)?,
    }))
}

fn merge(input: &MergeInputs, ring: bool) -> Result<z::ProvingInput, WalletError> {
    let inputs = input
        .inputs
        .iter()
        .map(|spend| {
            Ok(z::MergeInput {
                domain: element(&spend.utxo.domain)?,
                amount: element(&spend.utxo.amount)?,
                blinding: element(&spend.utxo.blinding)?,
                ring_data_hash: element(&spend.utxo.ring_data_hash)?,
                state_path: fields(&spend.state_path_elements)?,
                state_path_index: field(&spend.state_path_index)?,
                nullifier_low_value: field(&spend.nullifier_low_value)?,
                nullifier_next_value: field(&spend.nullifier_next_value)?,
                nullifier_low_path: fields(&spend.nullifier_low_path_elements)?,
                nullifier_low_path_index: field(&spend.nullifier_low_path_index)?,
                tree_slot: field(&spend.tree_slot)?,
                nullifier: field(&spend.nullifier)?,
            })
        })
        .collect::<Result<_, WalletError>>()?;
    Ok(z::ProvingInput::Merge(z::MergeInputs {
        inputs,
        output_ring_data_hash: element(&input.output.utxo.ring_data_hash)?,
        output_hash: field(&input.output.hash)?,
        tree_slots: tree_slots(&input.tree_slots)?,
        output_tree_id: field(&input.output_tree_id)?,
        mint_chunks: fields(&input.mint_chunks)?,
        output_ciphertext_salt: (!ring)
            .then(|| field(&input.output_ciphertext_salt))
            .transpose()?,
        owner_pk_hash: field(&input.owner_pk_hash)?,
        user_nullifier_pk: field(&input.user_nullifier_pk)?,
        user_nullifier_secret: field(&input.user_nullifier_secret)?,
        external_data_hash: field(&input.external_data_hash)?,
        private_tx_hash: field(&input.private_tx_hash)?,
        public_input_hash: field(&input.public_input_hash)?,
        allow_dummy_inputs: field(&input.allow_dummy_inputs)?,
        published_output_ring_data_hash: field(&input.output_ring_data_hash)?,
        ring_program_id: field(&input.ring_program_id)?,
    }))
}
