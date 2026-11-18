//! Public deposits and SPL interface setup.
use super::{prepared::PreparedTransaction, types::Operation, MobileWallet};
use crate::{asset::mint_name, error::WalletError};
use solana_pubkey::Pubkey;
use spr_client::{ClientError, Rpc};
use spr_interface::{pda, state::ProtocolConfig, PROGRAM_ID_PUBKEY};
use spr_program::instruction::{
    AssetDeposit, CreateSplInterface, Deposit, DepositAsset, DepositSplAccounts,
};

impl MobileWallet {
    /// Deposit public SOL (`mint` `None`) or tokens from this account into
    /// its own private balance. Deposits carry no proof. A deposit of zero
    /// fails with [`WalletError::AmountZero`] before the network.
    ///
    /// The first deposit of a mint into the pool creates the mint's registry
    /// entry and vault in the same transaction, signed and paid for by this
    /// account; [`Operation::Deposit`] says so. A pool whose protocol config
    /// lets only its authority create them refuses with
    /// [`WalletError::AssetNotSupported`] before the blockhash is read. Two
    /// first deposits of one mint prepared at the same time both create
    /// them, so the second to land fails.
    pub fn prepare_deposit(
        &mut self,
        mint: Option<String>,
        amount: u64,
        priority_fee_lamports: Option<u64>,
    ) -> Result<PreparedTransaction, WalletError> {
        if amount == 0 {
            return Err(WalletError::AmountZero);
        }
        let (asset, registered) = self
            .assets()
            .resolve_for_deposit(&self.client, mint.as_deref())?;
        let mut instructions = Vec::with_capacity(2);
        if let (false, Some(token_program)) = (registered, asset.token_program) {
            self.check_spl_interface_creation(asset.mint)?;
            instructions.push(
                CreateSplInterface {
                    authority: self.owner,
                    mint: asset.mint,
                    token_program,
                }
                .instruction(),
            );
        }
        let deposit_asset = match asset.token_program {
            None => DepositAsset::Sol,
            Some(token_program) => DepositAsset::Spl(DepositSplAccounts {
                mint: asset.mint,
                user_token: pda::associated_token_address_with_program(
                    &self.owner,
                    &asset.mint,
                    &token_program,
                ),
                token_program,
            }),
        };
        // The signing key's tag marks the deposit, as it marks every note
        // this wallet receives: the wallet's scan reads it back by that tag.
        let deposit = Deposit {
            tree: pda::tree(0),
            depositor: self.owner,
            deposits: vec![AssetDeposit {
                asset: deposit_asset,
                view_tag: self.address.confidential_view_tag()?,
                owner: self.address.owner_hash()?,
                amount,
                memo: None,
            }],
        }
        .instruction()?;
        instructions.push(deposit);
        let message =
            self.with_priority_fee(self.message(&instructions)?, priority_fee_lamports)?;
        self.pending(
            Operation::Deposit {
                mint: mint_name(&asset.mint),
                amount,
                creates_spl_interface: !registered,
            },
            message,
        )
    }

    /// Whether this account may create `mint`'s registry entry and vault, as
    /// the pool's `create_spl_interface` decides: anyone when the protocol
    /// config allows it, else only the protocol authority.
    fn check_spl_interface_creation(&self, mint: Pubkey) -> Result<(), WalletError> {
        let address = pda::protocol_config();
        let account = self
            .client
            .get_account(address)?
            .filter(|account| account.owner == PROGRAM_ID_PUBKEY)
            .ok_or(ClientError::ProtocolConfigNotFound(address))?;
        let config = ProtocolConfig::from_account_bytes(&account.data)
            .map_err(|failure| ClientError::InvalidProtocolConfig(format!("{failure:?}")))?;
        if config.allows_permissionless_spl_interface_creation()
            || config.check_protocol_authority(&self.owner).is_ok()
        {
            return Ok(());
        }
        Err(WalletError::AssetNotSupported {
            mint: mint.to_string(),
        })
    }
}
