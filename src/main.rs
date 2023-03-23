use dotenv;
use ethers::prelude::*;
use eyre::Result;
use std::env;

abigen!(ArbClaim, "./abi/ArbClaim.json");
abigen!(Arb, "./abi/Arb.json");

use arbclaim::account::Account;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_filename(".env").ok();

    let eth_rpc = env::var("ETH_RPC")?;
    let rpc = env::var("ARB_RPC")?;
    let private_key = env::var("PRIVATE_KEY")?;
    let binance_deposit_addr = env::var("MY_BINANCE_DEPOSIT_ADDRESS")?;
    let binance_deposit_addr = binance_deposit_addr.parse::<Address>()?;
    let enable_transfer_to_binance = env::var("ENABLE_TRANSFER_TO_BINANCE")?;
    let enable_transfer_to_binance = enable_transfer_to_binance == "true";

    let arb_addr = "0x912CE59144191C1204E64559FE8253a0e49E6548".parse::<Address>()?;
    let arb_claim_addr = "0x67a24ce4321ab3af51c2d0a4801c3e111d88c9d9".parse::<Address>()?;
    let account = Account::new(&private_key, &rpc).await;
    let eth_account = Account::new(&private_key, &eth_rpc).await;
    let arb_claim = ArbClaim::new(arb_claim_addr, account.client.clone());
    let arb = Arb::new(arb_addr, account.client.clone());

    let start_block = 16890400u64;
    let end_block = 18208000u64;
    let mut finish = false;

    while let Ok(now) = eth_account.client.get_block_number().await {
        if now.as_u64() >= start_block - 25 && now.as_u64() <= end_block {
            loop {
                let cc = arb_claim.claim().from(account.sender);
                if let Err(err) = cc.estimate_gas().await {
                    println!("err: {:?}", err);
                } else {
                    cc.send().await?.await?;
                    println!("claim done");
                    if !enable_transfer_to_binance {
                        finish = true;
                        break;
                    }
                    if let Ok(arb_balance) = arb.balance_of(account.sender).call().await {
                        if arb_balance > U256::zero() {
                            let cc = arb
                                .transfer(binance_deposit_addr, arb_balance)
                                .from(account.sender);
                            if let Err(err) = cc.estimate_gas().await {
                                println!("err: {:?}\n", err);
                            } else {
                                cc.send().await.unwrap();
                                finish = true;
                                println!("transfer to binance done");
                                break;
                            }
                        }
                    }
                }
            }
        } else {
            println!("not in claim time");
        }
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        if finish {
            break;
        }
    }

    Ok(())
}
