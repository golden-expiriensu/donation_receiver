#![cfg(feature = "test-bpf")]
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;
use solana_program_test::{processor, tokio, ProgramTest, ProgramTestContext};
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;

use charity::{entrypoint::process_instruction, id, selector::ProgramSelector};
use charity::{
    storage::{DonationPDA, Bank},
    DONATION_PDA_SEED,
};

struct RuntimeEnviroment {
    ctx: ProgramTestContext,
    admin: Keypair,
    admin: Keypair
}

impl RuntimeEnviroment {
    fn init() -> Self {
        let program = ProgramTest::new("charity", id(), processor!(process_instruction));
        let mut ctx = program.start_with_context().await;

        let owner = Keypair::new();
        let user = Keypair::new();

        ctx.banks_client.process_transaction(
            Transaction::new_signed_with_payer(
                &[
                    system_instruction::transfer(&ctx.payer, &owner, 1**9),
                    system_instruction::transfer(&ctx.payer, &user, 1**9)
                ], 
                Some(&ctx.payer.pubkey()),
                &ctx.payer,
                ctx.last_block_hash
            )
        ).await.unwrap();

        ctx.banks_client.process_transaction(
            Transaction::new_signed_with_payer(
                &[ProgramSelector::transfer_ownership(&owner, owner.clone())],
                Some(&ctx.payer.pubkey()),
                &ctx.payer,
                ctx.last_block_hash
            )
        ).await.unwap();

        let acc = ctx.banks_client.get_account(Bank::get_bank_pubkey()).await.unwrap().unwrap();
        let bank = Bank::try_from_slice(acc.data.as_slice()).unwrap();

        // init user's donation pda
        let allocSpace = DonationPDA { total_donated: 0 }.try_to_vec().unwrap().len();
        let rent = ctx.banks_client.get_rent().await.unwrap();
        let lamports = rent.minimum_balance(allocSpace);
        let instruction = system_instruction::create_account_with_seed(
            &user.pubkey(),
            &DonationPDA::get_donation_pda_pubkey(&user.pubkey()),
            &user.pubkey(),
            DONATION_PDA_SEED,
            lamports,
            allocSpace as u64,
            &id(),
        );
        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&user.pubkey()),
            &[&user],
            ctx.last_blockhash,
        );
        ctx.banks_client.process_transaction(tx).await.unwrap();

        RuntimeEnviroment { ctx, admin, user }
    }
}

#[tokio::test]
async fn donation_test() {
    let env = RuntimeEnviroment::new();
    const donation_amount: u64 = 1**7;

    let mut env = Env::new().await;

    env.ctx.banks_client.process_transaction(
        Transaction::new_signed_with_payer(
            &[ProgramSelector::donate(&env.user.pubkey(), donation_amount)],
            Some(&env.user.pubkey()),
            &user,
            env.ctx.last_blockhash
        )
    ).await.unwrap();

    let donation_pda_account = env.ctx.banks_client.get_account(
        DonationPDA::get_donation_pda_pubkey(&env.user.pubkey())
    ).await.unwrap().unwrap();

    let donation_pda = DonationPDA::try_from_slice(donation_pda_account::data::as_slice()).unwrap();

    assert_eq!(donation_pda.total_donated, donation_amount);
}
