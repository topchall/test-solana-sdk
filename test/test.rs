use solana_program::pubkey::Pubkey;
use solana_program_test::{tokio, ProgramTest}

#[tokio::test]
async fn test_initiallize_mint() {
    let program_test = ProgramTest::default();
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;
}