use crate::{
    error::BlockbusterError,
    instruction::InstructionBundle,
    program_handler::{NotUsed, ParseResult, ProgramParser},
    programs::{
        candy_machine::state::{CandyMachine, CollectionPDA, FreezePDA},
        ProgramParseResult,
    },
};
use borsh::BorshDeserialize;
use plerkle_serialization::account_info_generated::account_info::AccountInfo;
use solana_sdk::{pubkey::Pubkey, pubkeys};
use std::convert::TryInto;

mod state;

pubkeys!(
    candy_machine_id,
    "cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ"
);

// Anchor account discriminators.
const CANDY_MACHINE_DISCRIMINATOR: [u8; 8] = [51, 173, 177, 113, 25, 241, 109, 189];
const COLLECTION_PDA_DISCRIMINATOR: [u8; 8] = [203, 128, 119, 125, 234, 89, 232, 157];
const FREEZE_PDA_DISCRIMINATOR: [u8; 8] = [154, 58, 148, 24, 101, 200, 243, 127];

pub enum CandyMachineAccountData {
    CandyMachine(CandyMachine),
    CollectionPDA(CollectionPDA),
    FreezePDA(FreezePDA),
}

impl ParseResult for CandyMachineAccountData {
    fn result_type(&self) -> ProgramParseResult {
        ProgramParseResult::CandyMachine(self)
    }
}

pub struct CandyMachineParser;

impl ProgramParser for CandyMachineParser {
    fn key(&self) -> Pubkey {
        candy_machine_id()
    }

    fn key_match(&self, key: &Pubkey) -> bool {
        key == &candy_machine_id()
    }

    fn handle_account(
        &self,
        account_info: &AccountInfo,
    ) -> Result<Box<dyn ParseResult>, BlockbusterError> {
        let account_data = if let Some(account_info) = account_info.data() {
            account_info
        } else {
            return Err(BlockbusterError::DeserializationError);
        };

        let discriminator: [u8; 8] = account_data[0..8].try_into().unwrap();

        let account_type = match discriminator {
            CANDY_MACHINE_DISCRIMINATOR => {
                let candy_machine = CandyMachine::try_from_slice(&account_data[8..])?;
                CandyMachineAccountData::CandyMachine(candy_machine)
            }
            COLLECTION_PDA_DISCRIMINATOR => {
                let collection_pda = CollectionPDA::try_from_slice(&account_data[8..])?;
                CandyMachineAccountData::CollectionPDA(collection_pda)
            }
            FREEZE_PDA_DISCRIMINATOR => {
                let freeze_pda = FreezePDA::try_from_slice(&account_data[8..])?;
                CandyMachineAccountData::FreezePDA(freeze_pda)
            }
            _ => return Err(BlockbusterError::UnknownAccountDiscriminator),
        };

        Ok(Box::new(account_type))
    }

    fn handle_instruction(
        &self,
        _bundle: &InstructionBundle,
    ) -> Result<Box<dyn ParseResult>, BlockbusterError> {
        Ok(Box::new(NotUsed::new()))
    }
}
