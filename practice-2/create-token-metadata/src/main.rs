use std::{env, str::FromStr};
use dotenv::dotenv;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use mpl_token_metadata::{
    accounts::Metadata,
    instructions::CreateMetadataAccountV3Builder,
    types::{DataV2},
};

fn main() {
    dotenv().ok();
    let private_key_str = env::var("SECRET_KEY").expect("SECRET_KEY not set");
    let private_key: Vec<u8> = serde_json::from_str(&private_key_str).expect("Invalid secret key");
    let user = Keypair::from_bytes(&private_key).expect("Invalid private key");

    let connection = RpcClient::new("https://api.devnet.solana.com");

    let mint_pubkey = Pubkey::from_str("GAekSueWeNCWHSDiKpR5mYUp4S3Ej32uoXVjcVUH4Na6")
        .expect("Invalid mint address");

    let (metadata_pda, _) = Metadata::find_pda(&mint_pubkey);

    let metadata_data = DataV2 {
        name: "Solana Vitalii".to_string(),
        symbol: "UAB-3".to_string(),
        uri: "https://arweave.net/1234".to_string(),
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let mut builder = CreateMetadataAccountV3Builder::new();

    let ix = builder
    .metadata(metadata_pda)
    .mint(mint_pubkey)
    .mint_authority(user.pubkey())
    .update_authority(user.pubkey(), true)
    .payer(user.pubkey())
    .data(metadata_data)
    .is_mutable(true)
    .instruction(); 


    let latest_blockhash = connection.get_latest_blockhash().expect("Failed to get blockhash");

    let mut tx = Transaction::new_with_payer(&[ix], Some(&user.pubkey()));
    tx.sign(&[&user], latest_blockhash);

    let sig = connection
        .send_and_confirm_transaction(&tx)
        .expect("Transaction failed");

    println!("✅ Metadata created: https://explorer.solana.com/tx/{}?cluster=devnet", sig);
}
