mod process_proxy_create_world;
mod process_register;

use process_proxy_create_world::*;
use process_register::*;

use crate::instruction::{accounts::*, RushProxyInstruction};
use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub struct Processor {}

impl<'a> Processor {
    /// Process the transaction
    ///
    /// - Deserializes the instruction data
    /// - Routes transaction data to the proper handler
    pub fn process(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'a>],
        instruction_data: &[u8],
    ) -> ProgramResult {
        // get instruction
        let instruction = RushProxyInstruction::try_from_slice(instruction_data)?;
        match instruction {
            RushProxyInstruction::Register {
                user_agent_salt,
                bump,
            } => process_register(
                program_id,
                RegisterAccounts::context(accounts)?,
                user_agent_salt,
                bump,
            )?,

            RushProxyInstruction::Deregister => {}

            RushProxyInstruction::ProxyCreateWorld {
                user_agent_salt,
                user_bump,
                name,
                description,
                regions,
                entities,
                world_bump,
            } => process_proxy_create_world(
                program_id,
                ProxyCreateWorldAccounts::context(accounts)?,
                user_agent_salt,
                user_bump,
                name,
                description,
                regions,
                entities,
                world_bump,
            )?,
        }

        Ok(())
    }
}
