use std::time::{SystemTime, UNIX_EPOCH};
use std::{io::Read, time::Duration};

use drillx::Solution;
use ore_api::{
    consts::{BUS_ADDRESSES, CONFIG_ADDRESS, EPOCH_DURATION, MINT_ADDRESS, PROOF,
    TOKEN_DECIMALS, TREASURY_ADDRESS }, instruction, state::{Config, Proof, Treasury}, ID as ORE_ID
};
pub use ore_utils::AccountDeserialize;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_client::client_error::{ClientError, ClientErrorKind};
use solana_sdk::{
    account::ReadableAccount, clock::Clock, instruction::Instruction, pubkey::Pubkey, sysvar,
};
use spl_associated_token_account::get_associated_token_address;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use futures::StreamExt;
use tokio::sync::RwLock;
use std::sync::Arc;
use serde::Deserialize;
use tokio::time::sleep;
use solana_sdk::{hash::Hash};

#[derive(Debug, Deserialize)]
pub struct Tip {
    pub time: String,
    pub landed_tips_25th_percentile: f64,
    pub landed_tips_50th_percentile: f64,
    pub landed_tips_75th_percentile: f64,
    pub landed_tips_95th_percentile: f64,
    pub landed_tips_99th_percentile: f64,
    pub ema_landed_tips_50th_percentile: f64,
}

pub const ORE_TOKEN_DECIMALS: u8 = TOKEN_DECIMALS;

pub const BLOCKHASH_QUERY_RETRIES: usize = 5;
pub const BLOCKHASH_QUERY_DELAY: u64 = 500;

pub fn get_auth_ix(signer: Pubkey, ) -> Instruction {
    let proof = proof_pubkey(signer);

    instruction::auth(proof)
}

pub fn get_mine_ix(signer: Pubkey, solution: Solution, bus: usize) -> Instruction {
    instruction::mine(signer, signer, BUS_ADDRESSES[bus], solution)
}

pub fn get_register_ix(signer: Pubkey) -> Instruction {
    instruction::open(signer, signer, signer)
}

pub fn get_reset_ix(signer: Pubkey) -> Instruction {
    instruction::reset(signer)
}

pub fn get_claim_ix(signer: Pubkey, beneficiary: Pubkey, claim_amount: u64) -> Instruction {
    instruction::claim(signer, beneficiary, claim_amount)
}

pub fn get_stake_ix(signer: Pubkey, sender: Pubkey, stake_amount: u64) -> Instruction {
    instruction::stake(signer, sender, stake_amount)
}

pub fn get_ore_mint() -> Pubkey {
    MINT_ADDRESS
}

pub fn get_ore_epoch_duration() -> i64 {
    EPOCH_DURATION
}

pub fn get_ore_decimals() -> u8 {
    TOKEN_DECIMALS
}

pub async fn get_config(
    client: &RpcClient,
) -> Result<ore_api::state::Config, String> {
    let data = client.get_account_data(&CONFIG_ADDRESS).await;
    match data {
        Ok(data) => {
            let config = Config::try_from_bytes(&data);
            if let Ok(config) = config {
                return Ok(*config)
            } else {
                return Err("Failed to parse config account".to_string())
            }
        }
        Err(_) => return Err("Failed to get config account".to_string()),
    }
}

pub async fn get_proof_and_config_with_busses(
    client: &RpcClient,
    authority: Pubkey,
) -> (
    Result<Proof, ()>,
    Result<ore_api::state::Config, ()>,
    Result<Vec<Result<ore_api::state::Bus, ()>>, ()>,
) {
    let account_pubkeys = vec![
        proof_pubkey(authority),
        CONFIG_ADDRESS,
        BUS_ADDRESSES[0],
        BUS_ADDRESSES[1],
        BUS_ADDRESSES[2],
        BUS_ADDRESSES[3],
        BUS_ADDRESSES[4],
        BUS_ADDRESSES[5],
        BUS_ADDRESSES[6],
        BUS_ADDRESSES[7],
    ];
    let datas = client.get_multiple_accounts(&account_pubkeys).await;
    if let Ok(datas) = datas {
        let proof = if let Some(data) = &datas[0] {
            Ok(*Proof::try_from_bytes(data.data()).expect("Failed to parse treasury account"))
        } else {
            Err(())
        };

        let treasury_config = if let Some(data) = &datas[1] {
            Ok(*ore_api::state::Config::try_from_bytes(data.data())
                .expect("Failed to parse config account"))
        } else {
            Err(())
        };
        let bus_1 = if let Some(data) = &datas[2] {
            Ok(*ore_api::state::Bus::try_from_bytes(data.data())
                .expect("Failed to parse bus1 account"))
        } else {
            Err(())
        };
        let bus_2 = if let Some(data) = &datas[3] {
            Ok(*ore_api::state::Bus::try_from_bytes(data.data())
                .expect("Failed to parse bus2 account"))
        } else {
            Err(())
        };
        let bus_3 = if let Some(data) = &datas[4] {
            Ok(*ore_api::state::Bus::try_from_bytes(data.data())
                .expect("Failed to parse bus3 account"))
        } else {
            Err(())
        };
        let bus_4 = if let Some(data) = &datas[5] {
            Ok(*ore_api::state::Bus::try_from_bytes(data.data())
                .expect("Failed to parse bus4 account"))
        } else {
            Err(())
        };
        let bus_5 = if let Some(data) = &datas[6] {
            Ok(*ore_api::state::Bus::try_from_bytes(data.data())
                .expect("Failed to parse bus5 account"))
        } else {
            Err(())
        };
        let bus_6 = if let Some(data) = &datas[7] {
            Ok(*ore_api::state::Bus::try_from_bytes(data.data())
                .expect("Failed to parse bus6 account"))
        } else {
            Err(())
        };
        let bus_7 = if let Some(data) = &datas[8] {
            Ok(*ore_api::state::Bus::try_from_bytes(data.data())
                .expect("Failed to parse bus7 account"))
        } else {
            Err(())
        };
        let bus_8 = if let Some(data) = &datas[9] {
            Ok(*ore_api::state::Bus::try_from_bytes(data.data())
                .expect("Failed to parse bus1 account"))
        } else {
            Err(())
        };

        (proof, treasury_config, Ok(vec![bus_1, bus_2, bus_3, bus_4, bus_5, bus_6, bus_7, bus_8]))
    } else {
        (Err(()), Err(()), Err(()))
    }
}

pub async fn get_proof(client: &RpcClient, authority: Pubkey) -> Result<Proof, String> {
    let proof_address = proof_pubkey(authority);
    let data = client.get_account_data(&proof_address).await;
    match data {
        Ok(data) => {
            let proof = Proof::try_from_bytes(&data);
            if let Ok(proof) = proof {
                return Ok(*proof)
            } else {
                return Err("Failed to parse proof account".to_string())
            }
        }
        Err(_) => return Err("Failed to get proof account".to_string()),
    }
}

pub fn proof_pubkey(authority: Pubkey) -> Pubkey {
    Pubkey::find_program_address(&[PROOF, authority.as_ref()], &ORE_ID).0
}

pub fn treasury_tokens_pubkey() -> Pubkey {
    get_associated_token_address(&TREASURY_ADDRESS, &MINT_ADDRESS)
}

pub async fn get_clock_account(client: &RpcClient) -> Result<Clock, ()> {
    if let Ok(data) = client.get_account_data(&sysvar::clock::ID).await {
        if let Ok(data) = bincode::deserialize::<Clock>(&data) {
            Ok(data)
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

pub fn get_cutoff(proof: Proof, buffer_time: u64) -> i64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Failed to get time")
        .as_secs() as i64;
    proof
        .last_hash_at
        .saturating_add(60)
        .saturating_sub(buffer_time as i64)
        .saturating_sub(now)
}

pub async fn start_websocket_listener(url: &str, tip_clone: Arc<RwLock<u64>>) {
    let (ws_stream, _) = connect_async(url).await.unwrap();
    let (_, mut read) = ws_stream.split();

    tokio::spawn(async move {
        while let Some(message) = read.next().await {
            if let Ok(Message::Text(text)) = message {
                if let Ok(tips) = serde_json::from_str::<Vec<Tip>>(&text) {
                    for item in tips {
                        let mut tip = tip_clone.write().await;
                        *tip = (item.landed_tips_50th_percentile * (10_f64).powf(9.0)) as u64;
                    }
                }
            }
        }
    });
}

pub async fn get_latest_blockhash_with_retries(
    client: &RpcClient,
) -> Result<(Hash, u64), ClientError> {
    let mut attempts = 0;

    loop {
        if let Ok((hash, slot)) = client
            .get_latest_blockhash_with_commitment(client.commitment())
            .await
        {
            return Ok((hash, slot));
        }

        // Retry
        sleep(Duration::from_millis(BLOCKHASH_QUERY_DELAY)).await;
        attempts += 1;
        if attempts >= BLOCKHASH_QUERY_RETRIES {
            return Err(ClientError {
                request: None,
                kind: ClientErrorKind::Custom(
                    "Max retries reached for latest blockhash query".into(),
                ),
            });
        }
    }
}
