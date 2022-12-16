use crate::utils::{
    abi_calls::{balance, constructor, nonce, transaction_hash, transfer},
    test_helpers::{
        base_asset_contract_id, constructor_users, setup_env, transfer_parameters,
        transfer_signatures, DEFAULT_THRESHOLD, DEFAULT_TRANSFER_AMOUNT,
    },
    VALID_SIGNER_PK,
};
use fuels::{prelude::*, signers::fuel_crypto::Message};

mod success {

    use super::*;

    #[tokio::test]
    async fn transfers() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let (receiver_wallet, receiver, data) = transfer_parameters();

        constructor(&deployer.contract, constructor_users(), DEFAULT_THRESHOLD).await;

        deployer
            .wallet
            .force_transfer_to_contract(
                deployer.contract.get_contract_id(),
                DEFAULT_TRANSFER_AMOUNT,
                BASE_ASSET_ID,
                TxParameters::default(),
            )
            .await
            .unwrap();

        // Check balances pre-transfer
        let initial_contract_balance = balance(&deployer.contract, base_asset_contract_id())
            .await
            .value;

        let initial_receiver_balance = deployer
            .wallet
            .get_provider()
            .unwrap()
            .get_asset_balance(receiver_wallet.address(), BASE_ASSET_ID)
            .await
            .unwrap();

        let nonce = nonce(&deployer.contract).await.value;

        let tx_hash = transaction_hash(
            &deployer.contract,
            receiver.clone(),
            DEFAULT_TRANSFER_AMOUNT,
            data,
            nonce,
        )
        .await
        .value
        .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        let signatures = transfer_signatures(private_key, tx_hash).await;

        transfer(
            &deployer.contract,
            receiver,
            base_asset_contract_id(),
            DEFAULT_TRANSFER_AMOUNT,
            data,
            signatures,
        )
        .await;

        // check balances post-transfer
        let final_contract_balance = balance(&deployer.contract, base_asset_contract_id())
            .await
            .value;

        let final_receiver_balance = deployer
            .wallet
            .get_provider()
            .unwrap()
            .get_asset_balance(receiver_wallet.address(), BASE_ASSET_ID)
            .await
            .unwrap();

        assert_eq!(initial_contract_balance, 200);
        assert_eq!(initial_receiver_balance, 0);

        assert_eq!(final_contract_balance, 0);
        assert_eq!(final_receiver_balance, 200);
    }
}

mod revert {

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "NotInitialized")]
    async fn not_initialized() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let (_receiver_wallet, receiver, data) = transfer_parameters();

        deployer
            .wallet
            .force_transfer_to_contract(
                deployer.contract.get_contract_id(),
                DEFAULT_TRANSFER_AMOUNT,
                BASE_ASSET_ID,
                TxParameters::default(),
            )
            .await
            .unwrap();

        let nonce = nonce(&deployer.contract).await.value;

        let tx_hash = transaction_hash(
            &deployer.contract,
            receiver.clone(),
            DEFAULT_TRANSFER_AMOUNT,
            data,
            nonce,
        )
        .await
        .value
        .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        let signatures = transfer_signatures(private_key, tx_hash).await;

        transfer(
            &deployer.contract,
            receiver,
            base_asset_contract_id(),
            DEFAULT_TRANSFER_AMOUNT,
            data,
            signatures,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientAssetAmount")]
    async fn insufficient_asset_amount() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let (_receiver_wallet, receiver, data) = transfer_parameters();

        constructor(&deployer.contract, constructor_users(), DEFAULT_THRESHOLD).await;

        let nonce = nonce(&deployer.contract).await.value;

        let tx_hash = transaction_hash(
            &deployer.contract,
            receiver.clone(),
            DEFAULT_TRANSFER_AMOUNT,
            data,
            nonce,
        )
        .await
        .value
        .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        let signatures = transfer_signatures(private_key, tx_hash).await;

        transfer(
            &deployer.contract,
            receiver,
            base_asset_contract_id(),
            DEFAULT_TRANSFER_AMOUNT,
            data,
            signatures,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "IncorrectSignerOrdering")]
    async fn incorrect_signer_ordering() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let (_receiver_wallet, receiver, data) = transfer_parameters();

        constructor(&deployer.contract, constructor_users(), DEFAULT_THRESHOLD).await;

        deployer
            .wallet
            .force_transfer_to_contract(
                deployer.contract.get_contract_id(),
                DEFAULT_TRANSFER_AMOUNT,
                BASE_ASSET_ID,
                TxParameters::default(),
            )
            .await
            .unwrap();

        let nonce = nonce(&deployer.contract).await.value;

        let tx_hash = transaction_hash(
            &deployer.contract,
            receiver.clone(),
            DEFAULT_TRANSFER_AMOUNT,
            data,
            nonce,
        )
        .await
        .value
        .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        let signatures = transfer_signatures(private_key, tx_hash).await;
        let incorrectly_ordered_signatures = vec![signatures[1].clone(), signatures[0].clone()];

        transfer(
            &deployer.contract,
            receiver,
            base_asset_contract_id(),
            DEFAULT_TRANSFER_AMOUNT,
            data,
            incorrectly_ordered_signatures,
        )
        .await;
    }

    #[tokio::test]
    #[should_panic(expected = "InsufficientApprovals")]
    async fn insufficient_approvals() {
        let (private_key, deployer, _non_owner) = setup_env(VALID_SIGNER_PK).await.unwrap();

        let (_receiver_wallet, receiver, data) = transfer_parameters();

        constructor(&deployer.contract, constructor_users(), DEFAULT_THRESHOLD).await;

        deployer
            .wallet
            .force_transfer_to_contract(
                deployer.contract.get_contract_id(),
                DEFAULT_TRANSFER_AMOUNT,
                BASE_ASSET_ID,
                TxParameters::default(),
            )
            .await
            .unwrap();

        let nonce = nonce(&deployer.contract).await.value;

        let tx_hash = transaction_hash(
            &deployer.contract,
            receiver.clone(),
            DEFAULT_TRANSFER_AMOUNT,
            data,
            nonce,
        )
        .await
        .value
        .0;
        let tx_hash = unsafe { Message::from_bytes_unchecked(tx_hash) };

        let mut signatures = transfer_signatures(private_key, tx_hash).await;
        signatures.remove(0);

        transfer(
            &deployer.contract,
            receiver,
            base_asset_contract_id(),
            DEFAULT_TRANSFER_AMOUNT,
            data,
            signatures,
        )
        .await;
    }
}
