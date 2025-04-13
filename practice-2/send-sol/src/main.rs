use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    system_instruction,
    pubkey::Pubkey,
    instruction::Instruction,
    message::Message,
};
use solana_client::rpc_client::RpcClient;
use std::str::FromStr;
use anyhow::Result;
use dotenv::dotenv;
use std::env;

fn main() -> Result<()> {
    dotenv().ok();

    let private_key_json = env::var("SECRET_KEY")
        .expect("Add SECRET_KEY to .env as a JSON array of u8s (e.g., from Sollet or Phantom export)");

    let keypair_bytes: Vec<u8> = serde_json::from_str(&private_key_json)?;
    let sender = Keypair::from_bytes(&keypair_bytes)?;

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    println!("🔑 Our public key is: {}", sender.pubkey());

    let recipient = Pubkey::from_str("4XZ1pXz4kDdjg6MJCGqvjfyscY45DKQnVp9iQWzVKjoA")?;
    println!("💸 Attempting to send 0.01 SOL to {}...", recipient);

    let lamports = (0.01 * solana_sdk::native_token::LAMPORTS_PER_SOL as f64) as u64;

    let transfer_instruction = system_instruction::transfer(&sender.pubkey(), &recipient, lamports);

    let memo_program_id = Pubkey::from_str("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr")?;
    let memo_text = "I am a little man who walks alone!";
    let memo_instruction = Instruction::new_with_bytes(
        memo_program_id,
        memo_text.as_bytes(),
        vec![],
    );

    let message = Message::new(&[transfer_instruction, memo_instruction], Some(&sender.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);

    let recent_blockhash = client.get_latest_blockhash()?;
    transaction.sign(&[&sender], recent_blockhash);

    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("✅ Transaction confirmed, signature: {}", signature);

    Ok(())
}
