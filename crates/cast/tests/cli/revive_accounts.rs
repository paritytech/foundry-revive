use foundry_test_utils::{casttest_serial, deploy_contract, revive::PolkadotNode, util::OutputExt};

casttest_serial!(test_cast_balance, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();
        let (account, _) = PolkadotNode::dev_accounts().next().expect("no dev accounts available");
        let account = account.to_string();

        let bal = cmd
            .cast_fuse()
            .args(["balance", "--rpc-url", url, &account])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(bal.parse::<u128>().is_ok(), "balance wasn't a valid integer: `{bal}`");
    }
});

casttest_serial!(test_cast_nonce, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();
        let (account, _) = PolkadotNode::dev_accounts().next().unwrap();
        let account = account.to_string();

        let nonce = cmd
            .cast_fuse()
            .args(["nonce", "--rpc-url", url, &account])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(nonce.parse::<u64>().is_ok(), "nonce wasn't a valid integer: `{nonce}`");
    }
});

casttest_serial!(test_cast_code, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let (url, _deployer_pk, contract_address, _tx_hash) = deploy_contract!(cmd);

        cmd.cast_fuse()
            .args(["code", "--rpc-url", url, &contract_address])
            .assert_success()
            .stdout_eq(str![[r#"
0x5[..]

"#]]);
    }
});

casttest_serial!(test_cast_codesize, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let (url, _deployer_pk, contract_address, _tx_hash) = deploy_contract!(cmd);

        let size = cmd
            .cast_fuse()
            .args(["codesize", "--rpc-url", url, &contract_address])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(size.parse::<u64>().is_ok(), "codesize wasn't a valid integer: `{size}`");
    }
});

casttest_serial!(test_cast_storage, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let (url, _deployer_pk, contract_address, _tx_hash) = deploy_contract!(cmd);

        let val = cmd
            .cast_fuse()
            .args(["storage", "--rpc-url", url, &contract_address, "0x0"])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(val.starts_with("0x"), "storage didn't return hex: `{val}`");
    }
});
