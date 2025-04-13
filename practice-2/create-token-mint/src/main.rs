use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,
    program_pack::Pack,
    transaction::Transaction,
    system_instruction,
    message::Message,
};
use spl_token::instruction::initialize_mint;
use spl_token::id as token_program_id;
use anyhow::Result;
use dotenv::dotenv;
use std::env;
use serde_json;

fn main() -> Result<()> {
    dotenv().ok();

    let private_key = env::var("SECRET_KEY")
        .expect("Add SECRET_KEY to .env as a JSON array");
    let bytes: Vec<u8> = serde_json::from_str(&private_key)?;
    let payer = Keypair::from_bytes(&bytes)?;

    let rpc_url = "https://api.devnet.solana.com";
    let client = RpcClient::new(rpc_url.to_string());

    println!("🔑 Our public key is: {}", payer.pubkey());

    let mint = Keypair::new();
    let decimals = 2;
    let mint_rent = client.get_minimum_balance_for_rent_exemption(spl_token::state::Mint::LEN)?;

    let create_account_ix = system_instruction::create_account(
        &payer.pubkey(),
        &mint.pubkey(),
        mint_rent,
        spl_token::state::Mint::LEN as u64,
        &token_program_id(),
    );

    let init_mint_ix = initialize_mint(
        &token_program_id(),
        &mint.pubkey(),
        &payer.pubkey(),
        None,
        decimals,
    )?;

    let message = Message::new(&[create_account_ix, init_mint_ix], Some(&payer.pubkey()));
    let mut transaction = Transaction::new_unsigned(message);

    let recent_blockhash = client.get_latest_blockhash()?;
    transaction.sign(&[&payer, &mint], recent_blockhash);

    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("✅ Token Mint Signature: {}", signature);

    let link = format!(
        "https://explorer.solana.com/address/{}?cluster=devnet",
        mint.pubkey()
    );
    println!("✅ Token Mint: {}", link);

    Ok(())
}
