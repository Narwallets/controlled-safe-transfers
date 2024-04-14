use anyhow::anyhow;
use near_gas::*;
use near_workspaces::{
    network::Sandbox,
    result::ExecutionFinalResult,
    types::{KeyType, NearToken, SecretKey},
    Account, AccountId, Contract, Worker,
};
use std::{
    env,
    str::{self, FromStr},
};
use tokio::join;

// use meta_test_utils::now::Now;
// use meta_test_utils::now;

const CONTROLLED_TRANSFER_CONTRACT_FILEPATH: &str =
    "target/wasm32-unknown-unknown/release/controlled_transfer_contract.wasm";
const TEST_TOKEN_FILEPATH: &str = "target/wasm32-unknown-unknown/release/test_nep141_token.wasm";

pub const E24: u128 = 1_000_000_000_000_000_000_000_000;

fn test_token_as_u128_string(amount: u64) -> String {
    format!("{}000000", amount)
}

mod nep141;
use nep141::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("init sandbox");
    let worker = near_workspaces::sandbox().await?;

    // Creating Accounts.
    println!("Creating Accounts");
    let (owner, operator, destination, other_receiver) = join!(
        create_account(&worker, "owner"),
        create_account(&worker, "operator"),
        create_account(&worker, "destination"),
        create_account(&worker, "other-receiver")
    );
    let (owner, operator, destination, other_receiver) =
        (owner?, operator?, destination?, other_receiver?);

    ///////////////////////////////////////
    // Stage 1: Deploy relevant contracts
    ///////////////////////////////////////

    let contract_account = create_account(&worker, "controlled-transfer").await?;
    let controlled_transfer_contract =
        deploy_controlled_transfer_contract(&contract_account, &owner, &operator).await?;

    let test_token_contract = create_nep141_token(&owner, &worker).await?;

    println!(
        "controlled transfer Contract: {}",
        controlled_transfer_contract.id()
    );
    println!("test-token token Contract: {}", test_token_contract.id());

    println!("Owner: {}", owner.id());
    println!("destination: {}", destination.id());
    println!("other_receiver: {}", other_receiver.id());

    (_, _, _) = join!(
        storage_deposit(
            &controlled_transfer_contract.as_account(),
            &test_token_contract,
        ),
        storage_deposit(&destination, &test_token_contract),
        storage_deposit(&other_receiver, &test_token_contract)
    );

    // send some tokens to the controlled-contract
    {
        let amount = test_token_as_u128_string(40);
        ft_transfer(
            &test_token_contract,
            &owner,
            &controlled_transfer_contract.as_account(),
            &amount,
        )
        .await?;
        // check balance
        let balance = ft_balance(
            &test_token_contract,
            &controlled_transfer_contract.as_account(),
        )
        .await?;
        println!(
            "initial balance of {}: {:?}\n",
            controlled_transfer_contract.id(),
            &balance
        );
        assert_eq!(balance, amount);
    }

    // test controlled transfer
    {
        let amount = test_token_as_u128_string(15);
        controlled_add_destination(&controlled_transfer_contract, &owner, &destination).await?;
        controlled_ft_transfer(
            &controlled_transfer_contract,
            &test_token_contract,
            &operator,
            &destination,
            &amount,
        )
        .await?;
        // check balance
        let balance = ft_balance(&test_token_contract, &destination).await?;
        println!("mpDAO balance of {}: {:?}\n", destination.id(), &balance);
        assert_eq!(balance, amount);
    }

    {
        // must fail, destination not in list
        let res: ExecutionFinalResult = operator
            .call(controlled_transfer_contract.id(), "ft_transfer")
            .args_json(serde_json::json!({
                "token_contract_id": test_token_contract.id(),
                "receiver_id": other_receiver.id(),
                "amount": test_token_as_u128_string(1)
            }))
            .gas(NearGas::from_tgas(200))
            .transact()
            .await?;
        assert!(res.is_failure());
        let err_text = res.into_result().unwrap_err().to_string();
        assert!(
            err_text.contains("destination other-receiver.test.near not allowed"),
            "err_text:{}",
            err_text
        );
        println!("ok, blocks transfer if destination is not valid")
    }

    {
        // must fail, sender is no operator
        let res: ExecutionFinalResult = owner
            .call(controlled_transfer_contract.id(), "ft_transfer")
            .args_json(serde_json::json!({
                "token_contract_id": test_token_contract.id(),
                "receiver_id": destination.id(),
                "amount": test_token_as_u128_string(1)
            }))
            .gas(NearGas::from_tgas(200))
            .transact()
            .await?;
        assert!(res.is_failure());
        let err_text = res.into_result().unwrap_err().to_string();
        assert!(
            err_text.contains("Only the operator can call this function"),
            "err_text:{}",
            err_text
        );
        println!("ok, blocks transfer if sender is not operator")
    }

    {
        let balance = ft_balance(&test_token_contract, &controlled_transfer_contract.as_account()).await?;
        println!("mpDAO balance of {}: {:?}\n", controlled_transfer_contract.id(), &balance);
        assert_eq!(balance, test_token_as_u128_string(25));
    }

    Ok(())
}

async fn deploy_controlled_transfer_contract(
    contract_account: &Account,
    owner: &Account,
    operator: &Account,
) -> anyhow::Result<Contract> {
    println!(
        "deploy controlled_transfer_contract, cwd:{}",
        env::current_dir()?.display()
    );
    let contract_wasm = std::fs::read(CONTROLLED_TRANSFER_CONTRACT_FILEPATH)?;
    let controlled_transfer_contract = contract_account
        .deploy(&contract_wasm)
        .await
        .unwrap()
        .result;

    let res = controlled_transfer_contract
        .call("new")
        .args_json(serde_json::json!({
            "owner_id": owner.id(),
            "operator_id": operator.id(),
        }))
        .transact()
        .await?;
    if res.failures().len() > 0 {
        panic!("controlled_transfer_contract init error: {:#?}", res)
    }

    Ok(controlled_transfer_contract)
}

pub(crate) const DEV_ACCOUNT_SEED: &str = "testificate";

pub async fn create_account(worker: &Worker<Sandbox>, name: &str) -> anyhow::Result<Account> {
    let sk = SecretKey::from_seed(KeyType::ED25519, DEV_ACCOUNT_SEED);
    //    let (id, sk) = self.dev_generate().await;
    let account_id = AccountId::from_str(&format!("{}.test.near", name)).unwrap();
    let account = worker.create_tla(account_id, sk).await?;
    Ok(account.into_result()?)
}

pub fn verify(res: ExecutionFinalResult, desc: &String) -> anyhow::Result<()> {
    println!("call: {}", desc);
    if res.failures().len() > 0 {
        //anyhow::Error::new(res)
        Err(anyhow!("{:?}", res))
    } else {
        Ok(())
    }
}

pub async fn controlled_ft_transfer(
    controlled_contract: &Contract,
    token: &Contract,
    source: &Account,
    receiver: &Account,
    amount_string: &String,
) -> anyhow::Result<()> {
    let desc = format!(
        "Controlled Transfer {} to {} {}",
        source.id(),
        receiver.id(),
        amount_string
    );
    let res = source
        .call(controlled_contract.id(), "ft_transfer")
        .args_json(serde_json::json!({
            "token_contract_id": token.id(),
            "receiver_id": receiver.id(),
            "amount": amount_string
        }))
        .gas(NearGas::from_tgas(200))
        .transact()
        .await?;

    verify(res, &desc)
}

pub async fn controlled_add_destination(
    controlled_contract: &Contract,
    owner: &Account,
    destination: &Account,
) -> anyhow::Result<()> {
    let desc = format!("add_valid_destination {}", destination.id(),);
    let res = owner
        .call(controlled_contract.id(), "add_valid_destination")
        .args_json(serde_json::json!({
            "account_id": destination.id(),
        }))
        .gas(NearGas::from_tgas(25))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await?;
    verify(res, &desc)
}
