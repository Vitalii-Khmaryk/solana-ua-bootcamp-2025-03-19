use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    commitment_config::CommitmentConfig,
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;
use dotenv::dotenv;
use std::{env, str::FromStr};
use spl_token::ID as TOKEN_PROGRAM_ID;

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

    println!("🔑 Our public key is: {}", sender.pubkey());

    let token_mint = Pubkey::from_str("GAekSueWeNCWHSDiKpR5mYUp4S3Ej32uoXVjcVUH4Na6")?;
    let recipient = Pubkey::from_str("4XZ1pXz4kDdjg6MJCGqvjfyscY45DKQnVp9iQWzVKjoA")?;

    let ata = get_associated_token_address(&recipient, &token_mint);
    println!("🔍 Associated Token Account: {}", ata);

    if connection.get_account(&ata).is_err() {
        println!("📦 Creating associated token account...");

        let ix = create_associated_token_account(
            &sender.pubkey(),
            &recipient,
            &token_mint,
            &TOKEN_PROGRAM_ID,
        );
        

        let recent_blockhash = connection.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&sender.pubkey()),
            &[&sender],
            recent_blockhash,
        );

        connection.send_and_confirm_transaction(&tx)?;
        println!("✅ ATA created successfully!");
    } else {
        println!("✅ ATA already exists.");
    }

    println!(
        "🔗 https://explorer.solana.com/address/{}?cluster=devnet",
        ata
    );

    Ok(())
}
