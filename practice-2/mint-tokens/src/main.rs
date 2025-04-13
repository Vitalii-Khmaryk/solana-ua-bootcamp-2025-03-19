use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use spl_token::instruction::mint_to;
use spl_token::ID as TOKEN_PROGRAM_ID;
use dotenv::dotenv;
use std::{env, str::FromStr};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let secret_key_str = env::var("SECRET_KEY")
        .expect("❌ Add SECRET_KEY to .env as a JSON array string of your private key.");
    let secret_bytes: Vec<u8> = serde_json::from_str(&secret_key_str)?;
    let sender = Keypair::from_bytes(&secret_bytes)?;

    let connection = RpcClient::new_with_commitment(
        "https://api.devnet.solana.com".to_string(),
        CommitmentConfig::confirmed(),
    );

    let token_mint = Pubkey::from_str("GAekSueWeNCWHSDiKpR5mYUp4S3Ej32uoXVjcVUH4Na6")?;
    let recipient_ata = Pubkey::from_str("4MHmdoVJpuuyTYNnEeo4kCZhz6qEahpm8ZZeXDKa5jda")?;

    const DECIMALS: u64 = 10u64.pow(2);
    let amount = 10 * DECIMALS;

    let ix = mint_to(
        &TOKEN_PROGRAM_ID,
        &token_mint,
        &recipient_ata,
        &sender.pubkey(),
        &[],
        amount,
    )?;

    let recent_blockhash = connection.get_latest_blockhash()?;
    let tx = Transaction::new_signed_with_payer(
        &[ix],
        Some(&sender.pubkey()),
        &[&sender],
        recent_blockhash,
    );

    let sig = connection.send_and_confirm_transaction(&tx)?;
    println!("✅ Success!");
    println!(
        "Mint Token Transaction: https://explorer.solana.com/tx/{}?cluster=devnet",
        sig
    );

    Ok(())
}
