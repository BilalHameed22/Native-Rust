// Goal: understand the absolute minimum shape of a native solana program
//the entrypoint macro, the 'process_instruction" signature, logging, and 
//how to iterate over the accounts passed into an instruction

//This program nothing "useful" - it just logs a message and echoes
//back info about whatever accounts were passed to it. That's deliberate.
//before adding state or CPI, get comfortable with plumbing.

use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

// 'entrypont!' is a macro that generates the actual FFI-safe entrypoint the
//Solana runtime calls into. Under the hood it wraps our funtion so the
//runtime can hand it raw pointers/lengths and have them safely turned into
//the '&Pubkey", "&[AccountInfo]", '&[u8] types below.

entrypoint!(process_instruction);

/// Every ntive Solana program has exactly one entrypoint funtion with this
/// signature. Think of it as 'fn main()' for on-chain programs.
///
/// - 'program_id' : this program's own on-chain address (useful for PDA checks)
/// -'accounts' : every account the "caller" included in this instruction, in the exact order they sent them.
/// -'instruction_data' : an opaque byte blob _ You decide how to interpret it 

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult{
    // 'msg!' writes to the progam log, visible via 'solana logs' or in
    //transaction simulation results. This is your 'println!' on-chain but it consts compute units, so keep it lean in production.
    msg!("Hello, Solana! This program is: {}", program_id);
    msg!("Received {} account(s)", accounts.len());
    msg!("Instruction data was {} byte(s)", instruction_data.len());

    // Iterate every account passed in and log a few useful fields.
    // This is the same pattern you'll use in every future program to pull accounts out one at a time via an iterator (See project 2 onward), where we use 'next_account_info' instead of a raw loop
    
    for (i, account) in accounts.iter().enumerate() {
        msg!(
            "Account #{i}:  Key = {} owner = {} lamports = {} is_singer = {} is_writeable = {}",
            account.key,
            account.owner,
            account.lamports(),
            account.is_signer,
            account.is_writeable

        );

    }
    Ok(())
   
}
 //----------------------------------------------------------------------
    // Unit tests using 'solana-program-test', which spins up an in-process "bank client" simulating the runtime - no local validator process needed.
    // Run with : 'cargo test'
    //----------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use solana_program_test::*;
    use solana_sdk::{
        instruction::{AccountMeta, Instruction},
        signature::Signer,
        transaction::Transaction,
    };

    #[tokio::test]
    async fn test_hello_world_executes_without_error(){
        //Register our pogram under a test-only program ID.
        let program_id = Pubkey::new_unique();
        let program_test = ProgramTest::new("hello_world", program_id, processor!(process_instruction));

        let (banks_client, payer, recent_blockhash) = program_test.start().await;

        // Build a transaction that calls our program with zero extra accounts and empty instruction data - that's all "hello world" needs.
        let instruction = Instruction::new_with_bytes(
            program_id,
            &[], // instruction data
            vec![AccountMeta::new(payer.pubkey(), true)],
        );

        let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
        transaction.sign(&[&payer], recent_blockhash);

        // If 'process_instruction' returned an Err, this would fail the test.
        banks_client
            .process_instruction(transaction)
            .await
            .unwrap();


    }
}