use bdk::wallet::bip47;
#[test]
fn test_derive_outbound_wallet() {
    let mut bob_addresses = vec![
        "1KQvRShk6NqPfpr4Ehd53XUhpemBXtJPTL",
        "1CZAmrbKL6fJ7wUxb99aETwXhcGeG3CpeA",
        "1FsBVhT5dQutGwaPePTYMe5qvYqqjxyftc",
        "12u3Uued2fuko2nY4SoSFGCoGLCBUGPkk6",
        "141fi7TY3h936vRUKh1qfUZr8rSBuYbVBK",
    ];
    let bob_paymentcode = PaymentCode::from_str("PM8TJS2JxQ5ztXUpBBRnpTbcUXbUHy2T1abfrb3KkAAtMEGNbey4oumH7Hc578WgQJhPjBxteQ5GHHToTYHE3A1w6p7tU6KSoFmWBVbFGjKPisZDbP97").unwrap();

    let m = crate::keys::bip39::Mnemonic::parse(
        "response seminar brave tip suit recall often sound stick owner lottery motion",
    )
    .unwrap();
    let alice_main_wallet = Wallet::new(
        Bip44(m.clone(), KeychainKind::External),
        None,
        Network::Bitcoin,
        MemoryDatabase::new(),
    )
    .unwrap();
    let mut alice = Bip47Wallet::new(m, &alice_main_wallet).unwrap();
    for i in 0..5 {
        let outbound_wallet = alice.derive_outbound_wallet(&bob_paymentcode, i).unwrap().unwrap();

        let bob_address = outbound_wallet.get_address(AddressIndex::New).unwrap().to_string();
        let test_address = bob_addresses.pop().unwrap().to_owned();
        println!("Testing for address {}", test_address);
        assert_eq!(&bob_address, &test_address, "{} != {}", bob_address, test_address);
    }
}

#[test]
fn test_derive_inbound_wallet() {
    let mut bob_addresses = vec![
        "1KQvRShk6NqPfpr4Ehd53XUhpemBXtJPTL",
        "1CZAmrbKL6fJ7wUxb99aETwXhcGeG3CpeA",
        "1FsBVhT5dQutGwaPePTYMe5qvYqqjxyftc",
        "12u3Uued2fuko2nY4SoSFGCoGLCBUGPkk6",
        "141fi7TY3h936vRUKh1qfUZr8rSBuYbVBK",
    ];
    let alice_paymentcode = PaymentCode::from_str("PM8TJTLJbPRGxSbc8EJi42Wrr6QbNSaSSVJ5Y3E4pbCYiTHUskHg13935Ubb7q8tx9GVbh2UuRnBc3WSyJHhUrw8KhprKnn9eDznYGieTzFcwQRya4GA").unwrap();

    let m = crate::keys::bip39::Mnemonic::parse(
        "reward upper indicate eight swift arch injury crystal super wrestle already dentist",
    )
    .unwrap();
    let bob_main_wallet = Wallet::new(
        Bip44(m.clone(), KeychainKind::External),
        None,
        Network::Bitcoin,
        MemoryDatabase::new(),
    )
    .unwrap();
    let mut bob = Bip47Wallet::new(m, &bob_main_wallet).unwrap();
    for i in 0..5 {
        let inbound_wallet = bob.derive_inbound_wallet(&alice_paymentcode, i).unwrap().unwrap();

        let bob_address = inbound_wallet.get_address(AddressIndex::New).unwrap().to_string();
        let test_address = bob_addresses.pop().unwrap().to_owned();
        println!("Testing for address {}", test_address);
        assert_eq!(&bob_address, &test_address, "{} != {}", bob_address, test_address);
    }
}

#[test]
fn t() {
    use crate::testutils::blockchain_tests::TestClient;

    let mut tc = TestClient::default();
    let blockchain = Arc::new(ElectrumBlockchain::from(
        electrum_client::Client::new(&tc.electrsd.electrum_url).unwrap(),
    ));

    // let client = electrum_client::Client::new("ssl://electrum.blockstream.info:60002").unwrap();
    // let blockchain = Arc::new(ElectrumBlockchain::from(client));

    // let key = PrivateKey::from_wif("cU1zSPAAHNGE8quJZkBsFJELTxfJRsS82Z4M4WPb95VcdpBM9gBv").unwrap();
    // let key =
    //     PrivateKey::from_wif("L3esrLZfpd5B2GGSjVSUiHGZvDtDpAAFRLGKP9UiMC46pbfYAJJk").unwrap();
    let m = crate::keys::bip39::Mnemonic::parse(
        "response seminar brave tip suit recall often sound stick owner lottery motion",
    )
    .unwrap();
    let alice_main_wallet = Wallet::new(
        Bip44(m.clone(), KeychainKind::External),
        None,
        Network::Regtest,
        MemoryDatabase::new(),
    )
    .unwrap();
    let tx = crate::testutils! {
        @tx ( (@addr alice_main_wallet.get_address(AddressIndex::Peek(5)).unwrap().address) => 50_000 )
    };
    tc.receive(tx);

    alice_main_wallet
        .sync(&blockchain, SyncOptions::default())
        .unwrap();
    println!("balance: {}", alice_main_wallet.get_balance().unwrap());
    println!(
        "{}",
        alice_main_wallet.get_address(AddressIndex::New).unwrap()
    );

    let mut alice = Bip47Wallet::new(m, &alice_main_wallet).unwrap();
    println!("{} {}", alice.payment_code(), alice.notification_address());
    assert_eq!(alice.payment_code().to_string(), "PM8TJTLJbPRGxSbc8EJi42Wrr6QbNSaSSVJ5Y3E4pbCYiTHUskHg13935Ubb7q8tx9GVbh2UuRnBc3WSyJHhUrw8KhprKnn9eDznYGieTzFcwQRya4GA");
    // assert_eq!(alice.notification_address().to_string(), "1JDdmqFLhpzcUwPeinhJbUPw4Co3aWLyzW");

    // let sharedsecret = Vec::<u8>::from_hex("736a25d9250238ad64ed5da03450c6a3f4f8f4dcdf0b58d1ed69029d76ead48d").unwrap();
    // let sharedsecret: [u8; 32] = sharedsecret.try_into().unwrap();
    // let sharedsecret = bitcoin::secp256k1::ecdh::SharedSecret::from(sharedsecret);
    // dbg!(&sharedsecret);

    // let outpoint = OutPoint::from_str("9c6000d597c5008f7bfc2618aed5e4a6ae57677aab95078aae708e1cab11f486:1").unwrap();
    // dbg!(&outpoint);
    // let bf = BlindingFactor::new(sharedsecret, &outpoint);

    // use bitcoin::hashes::hex::ToHex;
    // println!("blinded code: {}", alice.payment_code().encode_blinded(bf).to_hex());

    let m = crate::keys::bip39::Mnemonic::parse(
        "reward upper indicate eight swift arch injury crystal super wrestle already dentist",
    )
    .unwrap();
    let bob_main_wallet = Wallet::new(
        Bip44(m.clone(), KeychainKind::External),
        None,
        Network::Regtest,
        MemoryDatabase::new(),
    )
    .unwrap();
    let mut bob = Bip47Wallet::new(m, &bob_main_wallet).unwrap();
    println!("{} {}", bob.payment_code(), bob.notification_address());
    assert_eq!(bob.payment_code().to_string(), "PM8TJS2JxQ5ztXUpBBRnpTbcUXbUHy2T1abfrb3KkAAtMEGNbey4oumH7Hc578WgQJhPjBxteQ5GHHToTYHE3A1w6p7tU6KSoFmWBVbFGjKPisZDbP97");
    // assert_eq!(bob.notification_address().unwrap().to_string(), "1ChvUUvht2hUQufHBXF8NgLhW8SwE2ecGV");
    // bob.sync(&blockchain).unwrap();
    // bob.sync(&blockchain).unwrap();
    // bob.sync(&blockchain).unwrap();
    // bob.sync(&blockchain).unwrap();

    let (mut psbt, _) = alice
        .build_notification_tx(&bob.payment_code(), None, None)
        .unwrap()
        .unwrap();
    alice_main_wallet
        .sign(&mut psbt, Default::default())
        .unwrap();
    blockchain.broadcast(&psbt.extract_tx()).unwrap();
    alice.record_notification_tx(&bob.payment_code());

    alice.sync(&blockchain).unwrap();

    let bob_addr = alice.get_payment_address(&bob.payment_code()).unwrap();
    let (mut psbt, _) = {
        let mut builder = alice_main_wallet.build_tx();
        builder.add_recipient(bob_addr.script_pubkey(), 10_000);
        builder.finish().unwrap()
    };
    alice_main_wallet
        .sign(&mut psbt, Default::default())
        .unwrap();
    blockchain.broadcast(&psbt.extract_tx()).unwrap();

    alice.sync(&blockchain).unwrap();

    println!("bob sync");
    bob.sync(&blockchain).unwrap();

    // dbg!(&tx);
}

// TODO: test main wallet with single key and xprv