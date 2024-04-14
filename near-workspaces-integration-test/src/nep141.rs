use near_gas::*;
use near_workspaces::{
    types::NearToken, Account, Contract, DevNetwork, Worker
};

use crate::TEST_TOKEN_FILEPATH;

pub async fn create_nep141_token(
    owner: &Account,
    worker: &Worker<impl DevNetwork>,
) -> anyhow::Result<Contract> {
    let token_contract_wasm = std::fs::read(TEST_TOKEN_FILEPATH)?;
    let token_contract = worker.dev_deploy(&token_contract_wasm).await?;
    println!(
        "init token contract {} {}",
        owner.id(),
        format!("{}", "200000000000000")
    );
    let res = token_contract
        .call("new_default_meta")
        .args_json(serde_json::json!({
            "owner_id": owner.id(),
            "total_supply": "200000000000000"
        }))
        .transact()
        .await?;
    if res.failures().len() > 0 {
        println!("token contract new_default_meta result: {:#?}", res);
        panic!("err on init")
    }

    Ok(token_contract)
}

pub async fn ft_transfer(
    nep_141_contract: &Contract,
    source: &Account,
    receiver: &Account,
    amount_string: &String,
) -> anyhow::Result<()> {
    let res = source
        .call(nep_141_contract.id(), "ft_transfer")
        .args_json(serde_json::json!({
           "receiver_id": receiver.id(),
            "amount": amount_string
        }))
        .gas(NearGas::from_tgas(200))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await?;
    if res.failures().len() > 0 {
        panic!(
            "Transfer {} to {} {} err: {:?}\n",
            source.id(),
            receiver.id(),
            amount_string,
            res
        );
    }
    Ok(())
}

pub async fn ft_balance(nep_141_contract: &Contract, account: &Account) -> anyhow::Result<String> {
    let res = nep_141_contract
        .view("ft_balance_of")
        .args_json(serde_json::json!({
            "account_id": account.id()
        }))
        .await?;
    let res: serde_json::Value = serde_json::from_slice(&res.result)?;
    Ok(String::from(res.as_str().unwrap()))
}

pub async fn storage_deposit(account: &Account, token_contract: &Contract) -> anyhow::Result<()> {
    println!("storage_deposit {}", account.id());
    let res = account
        .call(token_contract.id(), "storage_deposit")
        .args_json(serde_json::json!({
            "account_id": account.id(),
        }))
        .gas(NearGas::from_tgas(50))
        .deposit(NearToken::from_millinear(250))
        .transact()
        .await?;
    if res.failures().len() > 0 {
        println!("storage_deposit {}: {:?}\n", account.id(), res);
        Err(anyhow::Error::msg("storage_deposit failure"))
    } else {
        Ok(())
    }
}

