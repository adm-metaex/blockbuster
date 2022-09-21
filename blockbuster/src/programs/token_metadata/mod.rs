use crate::program_handler::ParseResult;
use crate::{
    error::BlockbusterError, instruction::InstructionBundle, program_handler::ProgramParser,
};
use crate::{program_handler::NotUsed, programs::ProgramParseResult};
use borsh::BorshDeserialize;
use solana_sdk::borsh::try_from_slice_unchecked;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::pubkeys;

use plerkle_serialization::AccountInfo;

pub use mpl_bubblegum::state::leaf_schema::{LeafSchema, LeafSchemaEvent};
pub use mpl_bubblegum::InstructionName;
use mpl_token_metadata::state::{
    CollectionAuthorityRecord, Edition, EditionMarker, Key, MasterEditionV1, MasterEditionV2,
    Metadata, ReservationListV1, ReservationListV2, UseAuthorityRecord,
};

pubkeys!(
    token_metadata_id,
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

pub enum TokenMetadataAccountData {
    EditionV1(Edition),
    MasterEditionV1(MasterEditionV1),
    MetadataV1(Metadata),
    MasterEditionV2(MasterEditionV2),
    EditionMarker(EditionMarker),
    UseAuthorityRecord(UseAuthorityRecord),
    CollectionAuthorityRecord(CollectionAuthorityRecord),
    ReservationListV2(ReservationListV2),
    ReservationListV1(ReservationListV1),
}

pub struct TokenMetadataAccountState {
    key: Key,
    data: TokenMetadataAccountData,
}

impl ParseResult for TokenMetadataAccountState {
    fn result(&self) -> &Self
    where
        Self: Sized,
    {
        self
    }
    fn result_type(&self) -> ProgramParseResult {
        ProgramParseResult::TokenMetadata(self)
    }
}

pub struct TokenMetadataParser;

impl ProgramParser for TokenMetadataParser {
    fn key(&self) -> Pubkey {
        token_metadata_id()
    }
    fn key_match(&self, key: &Pubkey) -> bool {
        key == &token_metadata_id()
    }

    fn handle_account(
        &self,
        account_info: &AccountInfo,
    ) -> Result<Box<(dyn ParseResult + 'static)>, BlockbusterError> {
        let account_data = if let Some(account_info) = account_info.data() {
            account_info
        } else {
            return Err(BlockbusterError::DeserializationError);
        };

        let key = Key::try_from_slice(&account_data[0..1])?;

        let token_metadata_account_state = match key {
            Key::EditionV1 => {
                let account: Edition = try_from_slice_unchecked(account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::EditionV1(account),
                }
            }
            Key::MasterEditionV1 => {
                let account: MasterEditionV2 = try_from_slice_unchecked(account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::MasterEditionV2(account),
                }
            }
            Key::MasterEditionV2 => {
                let account: MasterEditionV1 = try_from_slice_unchecked(account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::MasterEditionV1(account),
                }
            }
            Key::UseAuthorityRecord => {
                let account: UseAuthorityRecord = try_from_slice_unchecked(account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::UseAuthorityRecord(account),
                }
            }
            Key::EditionMarker => {
                let account: EditionMarker = try_from_slice_unchecked(account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::EditionMarker(account),
                }
            }
            Key::CollectionAuthorityRecord => {
                let account: CollectionAuthorityRecord = try_from_slice_unchecked(account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::CollectionAuthorityRecord(account),
                }
            }
            Key::MetadataV1 => {
                let account: Metadata = try_from_slice_unchecked(account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::MetadataV1(account),
                }
            }
            Key::ReservationListV1 => {
                let account: ReservationListV1 = try_from_slice_unchecked(account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::ReservationListV1(account),
                }
            }
            Key::ReservationListV2 => {
                let account: ReservationListV2 = try_from_slice_unchecked(account_data)?;

                TokenMetadataAccountState {
                    key: account.key,
                    data: TokenMetadataAccountData::ReservationListV2(account),
                }
            }
            Key::Uninitialized => {
                return Err(BlockbusterError::UninitializedAccount);
            }
        };

        Ok(Box::new(token_metadata_account_state))
    }

    fn handle_instruction(
        &self,
        _bundle: &InstructionBundle,
    ) -> Result<Box<(dyn ParseResult + 'static)>, BlockbusterError> {
        Ok(Box::new(NotUsed::new()))
    }
}
