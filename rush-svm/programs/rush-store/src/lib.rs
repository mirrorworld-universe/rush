use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    program::{invoke, invoke_signed},
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction::create_account,
    system_program::ID as SYSTEM_PROGRAM_ID,
    sysvar::rent::Rent,
    sysvar::Sysvar,
};

solana_program::declare_id!("Aq2EAZ8i8UgKGaGzpSPhfvGxf4hkziymA4WqXrJ4NYu4");
entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize)]
pub enum ProgramInstruction {
    Initialize,
    Add { a: u64, b: u64 },
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct State {
    sum: u64,
}

impl State {
    const LEN: usize = 8;
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // UNPACK DATA USING BORSH
    //
    // :'( Data babies
    let instruction = ProgramInstruction::try_from_slice(instruction_data)?;

    match instruction {
        // Accounts
        //
        // 1. State
        // 2. Payer
        // 3. System Program (A program in Solana, is an account)
        ProgramInstruction::Initialize => {
            let accounts_iter = &mut accounts.iter();
            let state = next_account_info(accounts_iter)?;
            let payer = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;

            // VALIDATION HERE IS NEEDED but not right now

            // PDA == PROGRAM DERIVED ADDRESS
            let (state_pda, state_bump) =
                Pubkey::find_program_address(&[b"state", payer.key.as_ref()], program_id);

            // business logic
            // create State account
            let rent = Rent::get()?.minimum_balance(State::LEN); // 8 bytes * Rent Fee
            let space: u64 = State::LEN as u64; // 8 bytes
            invoke_signed(
                &create_account(payer.key, &state_pda, rent, space, program_id),
                &[payer.clone(), state.clone()],
                &[&[b"state", payer.key.as_ref(), &[state_bump]]],
            )?;

            // fetch the data
            let mut state_raw_bytes = state.try_borrow_mut_data()?;
            // pub struct State {
            //     sum: u64,
            // }
            let mut state_data = State::try_from_slice(&state_raw_bytes)?;
            state_data.sum = 0;

            state_data.serialize(&mut &mut state_raw_bytes[..])?;
        }
        ProgramInstruction::Add { a, b } => {
            let accounts_iter = &mut accounts.iter();
            let payer = next_account_info(accounts_iter)?;
            let state = next_account_info(accounts_iter)?;

            // VALIDATION HERE IS NEEDED but not right now

            // fetch the data
            let mut state_raw_bytes = state.try_borrow_mut_data()?;
            // pub struct State {
            //     sum: u64,
            // }
            let mut state_data = State::try_from_slice(&state_raw_bytes)?;
            state_data.sum = a + b;

            state_data.serialize(&mut &mut state_raw_bytes[..])?;
        }
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program_test::*;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        pubkey::Pubkey,
        signature::Keypair,
        signature::Signer,
        system_instruction,
        transaction::Transaction,
    };

    #[tokio::test]
    async fn test_sanity() {
        assert!(true);
    }

    #[tokio::test]
    async fn test_add() {
        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new(
            // .so fixture is  retrieved from /target/deploy
            "add", program_id,
            // shank is incompatible with instantiating the BuiltInFunction
            None,
        );
        let mut ctx = program_test.start_with_context().await;

        let (state_pda, state_bump) =
            Pubkey::find_program_address(&[b"state", &ctx.payer.pubkey().as_ref()], &program_id);

        // create mint
        let init_ix = ProgramInstruction::Initialize;
        let mut init_ix_data = Vec::new();
        init_ix.serialize(&mut init_ix_data).unwrap();

        let first_num = 1;
        let second_num = 2;
        let add_ix = ProgramInstruction::Add {
            a: first_num.clone(),
            b: second_num.clone(),
        };
        let mut add_ix_data = Vec::new();
        add_ix.serialize(&mut add_ix_data).unwrap();

        let transaction = Transaction::new_signed_with_payer(
            &[
                Instruction {
                    program_id,
                    // Accounts
                    //
                    // 1. State
                    // 2. Payer
                    // 3. System Program (A program in Solana, is an account)
                    accounts: vec![
                        AccountMeta::new(state_pda, false),
                        AccountMeta::new(ctx.payer.pubkey(), true),
                        AccountMeta::new_readonly(SYSTEM_PROGRAM_ID, false),
                    ],
                    data: init_ix_data.clone(),
                },
                Instruction {
                    program_id,
                    // Accounts
                    //
                    // 1. Payer
                    // 2. State
                    accounts: vec![
                        AccountMeta::new(ctx.payer.pubkey(), true),
                        AccountMeta::new(state_pda, false),
                    ],
                    data: add_ix_data.clone(),
                },
            ],
            Some(&ctx.payer.pubkey()),
            &[&ctx.payer.insecure_clone()],
            ctx.last_blockhash,
        );

        // send transaction
        ctx.banks_client
            .process_transaction(transaction)
            .await
            .unwrap();

        // confirm state
        let state = ctx
            .banks_client
            .get_account_data_with_borsh::<State>(state_pda)
            .await
            .unwrap();

        let sum: u64 = first_num + second_num;
        assert_eq!(state.sum, sum);
    }
}
