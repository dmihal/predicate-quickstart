use fuels::{accounts::predicate::Predicate, types::Bits256, prelude::*};

// Load abi from json
abigen!(Predicate(
    name = "SimplePredicate",
    abi = "out/debug/simple_predicate-abi.json"
));

async fn get_wallets() -> (WalletUnlocked, WalletUnlocked, WalletUnlocked) {
    // Launch a local network and deploy the contract
    let mut wallets = launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(3),             /* Single wallet */
            Some(2),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        None,
        None,
    )
    .await;

    let wallet_1 = wallets.pop().unwrap();
    let wallet_2 = wallets.pop().unwrap();
    let wallet_3 = wallets.pop().unwrap();
    (wallet_1, wallet_2, wallet_3)
}

fn get_predicate(sender: &Bech32Address, recipient: &Bech32Address, provider: &Provider) -> Predicate {
    let configurables = SimplePredicateConfigurables::new()
        .with_SENDER(Bits256(sender.hash().into()))
        .with_RECIPIENT(Bits256(recipient.hash().into()));

    let mut predicate: Predicate = Predicate::load_from("out/debug/simple_predicate.bin")
        .unwrap()
        .with_configurables(configurables);
    predicate.set_provider(provider.clone());

    predicate
}

#[tokio::test]
async fn can_send() {
    let (wallet_1, wallet_2, _wallet_3) = get_wallets().await;
    let provider = wallet_1.provider().unwrap();
    let predicate = get_predicate(&wallet_1.address(), &wallet_2.address(), &provider);

    let amount = 1_000;

    // Fund the predicate
    wallet_1
        .transfer(
            predicate.address(),
            amount,
            BASE_ASSET_ID,
            TxParameters::default(),
        )
        .await
        .unwrap();

    // Send a transfer
    predicate
        .transfer(
            wallet_2.address(),
            amount,
            BASE_ASSET_ID,
            TxParameters::default(),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn can_send_back() {
    let (wallet_1, wallet_2, _wallet_3) = get_wallets().await;
    let provider = wallet_1.provider().unwrap();
    let predicate = get_predicate(&wallet_1.address(), &wallet_2.address(), &provider);

    let amount = 1_000;

    // Fund the predicate
    wallet_1
        .transfer(
            predicate.address(),
            amount,
            BASE_ASSET_ID,
            TxParameters::default(),
        )
        .await
        .unwrap();

    // Send a transfer
    predicate
        .transfer(
            wallet_1.address(),
            amount,
            BASE_ASSET_ID,
            TxParameters::default(),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn shouldnt_send_to_random_person() {
    let (wallet_1, wallet_2, wallet_3) = get_wallets().await;
    let provider = wallet_1.provider().unwrap();
    let predicate = get_predicate(&wallet_1.address(), &wallet_2.address(), &provider);

    let amount = 1_000;

    // Fund the predicate
    wallet_1
        .transfer(
            predicate.address(),
            amount,
            BASE_ASSET_ID,
            TxParameters::default(),
        )
        .await
        .unwrap();

    // Send a transfer
    let tx = predicate
        .transfer(
            wallet_3.address(),
            amount,
            BASE_ASSET_ID,
            TxParameters::default(),
        )
        .await;

    let is_err = tx.is_err();
    assert!(is_err);
}
