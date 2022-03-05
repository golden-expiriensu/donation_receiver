use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
use solana_program::system_instruction;
use solana_program_test::{processor, tokio, ProgramTest, ProgramTestContext};
use solana_sdk::signature::{Keypair, Signer};
use solana_sdk::transaction::Transaction;

use charity::{entrypoint::process_instruction, id, instruction::ProgramSelector};
use charity::{
    state::{DonationPDA, Bank},
    DONATION_PDA_SEED,
};

struct RuntimeEnviroment {
    ctx: ProgramTestContext,
    admin: Keypair,
    user: Keypair
}

impl RuntimeEnviroment {
    async fn init() -> Self {
        let program = ProgramTest::new("charity", id(), processor!(process_instruction));
        let mut ctx = program.start_with_context().await;

        let admin = Keypair::new();
        let user = Keypair::new();

        ctx.banks_client.process_transaction(
            Transaction::new_signed_with_payer(
                &[
                    system_instruction::transfer(&ctx.payer.pubkey(), &admin.pubkey(), 1_000_000_000),
                    system_instruction::transfer(&ctx.payer.pubkey(), &user.pubkey(), 1_000_000_000)
                ], 
                Some(&ctx.payer.pubkey()),
                &[&ctx.payer],
                ctx.last_blockhash
            )
        ).await.unwrap();

        ctx.banks_client.process_transaction(
            Transaction::new_signed_with_payer(
                &[ProgramSelector::transfer_ownership(&admin.pubkey(), admin.pubkey().to_bytes())],
                Some(&admin.pubkey()),
                &[&admin],
                ctx.last_blockhash
            )
        ).await.unwrap();

        let bank_account = ctx.banks_client.get_account(Bank::get_bank_pubkey()).await.unwrap().unwrap();
        let bank = Bank::try_from_slice(bank_account.data.as_slice()).unwrap();

        assert_eq!(bank.admin, admin.pubkey().to_bytes());

        // init user's donation pda
        let alloc_space = DonationPDA { total_donated: 0 }.try_to_vec().unwrap().len();
        let rent = ctx.banks_client.get_rent().await.unwrap();
        let lamports = rent.minimum_balance(alloc_space);
        let instruction = system_instruction::create_account_with_seed(
            &user.pubkey(),
            &DonationPDA::get_donation_pda_pubkey(&user.pubkey()),
            &user.pubkey(),
            DONATION_PDA_SEED,
            lamports,
            alloc_space as u64,
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
    let mut env = RuntimeEnviroment::init().await;
    const DONATION_AMOUNT: u64 = 10_000_000;

    env.ctx.banks_client.process_transaction(
        Transaction::new_signed_with_payer(
            &[ProgramSelector::donate(&env.user.pubkey(), DONATION_AMOUNT)],
            Some(&env.user.pubkey()),
            &[&env.user],
            env.ctx.last_blockhash
        )
    ).await.unwrap();

    let donation_pda_account = env.ctx.banks_client.get_account(
        DonationPDA::get_donation_pda_pubkey(&env.user.pubkey())
    ).await.unwrap().unwrap();

    let donation_pda = DonationPDA::try_from_slice(donation_pda_account.data.as_slice()).unwrap();

    assert_eq!(donation_pda.total_donated, DONATION_AMOUNT);
}
