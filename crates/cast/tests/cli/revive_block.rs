use foundry_test_utils::{casttest_serial, revive::PolkadotNode, util::OutputExt};

casttest_serial!(test_cast_block_number, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();
        let block_number = cmd
            .cast_fuse()
            .args(["block-number", "--rpc-url", url])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(
            block_number.parse::<u64>().is_ok(),
            "block-number output not a valid integer: `{block_number}`"
        );
    }
});

casttest_serial!(test_cast_gas_price, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();

        cmd.cast_fuse().args(["gas-price", "--rpc-url", url]).assert_success().stdout_eq(str![[
            r#"
1000

"#
        ]]);
    }
});

casttest_serial!(test_cast_basefee, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();

        let base_fee = cmd
            .cast_fuse()
            .args(["basefee", "--rpc-url", url])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(
            base_fee.parse::<u128>().is_ok(),
            "basefee output not a valid integer: `{base_fee}`"
        );
    }
});

casttest_serial!(test_cast_block, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();

        let output = cmd
            .cast_fuse()
            .args(["block", "latest", "--rpc-url", url, "--json"])
            .assert_success()
            .get_output()
            .stdout_lossy();

        let block_data = serde_json::from_str::<serde_json::Value>(&output)
            .expect("Failed to parse JSON output");

        assert!(block_data.get("hash").is_some(), "Missing 'hash' field");

        assert!(block_data.get("number").is_some(), "Missing 'number' field");

        assert!(block_data.get("parentHash").is_some(), "Missing 'parentHash' field");
        assert!(block_data.get("timestamp").is_some(), "Missing 'timestamp' field");
        assert!(block_data.get("transactions").is_some(), "Missing 'transactions' field");
    }
});

casttest_serial!(test_cast_age, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();

        let age = cmd
            .cast_fuse()
            .args(["age", "latest", "--rpc-url", url])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(age.ends_with("UTC"), "age output not a human timestamp ending in UTC: `{age}`");
    }
});

casttest_serial!(test_cast_find_block, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();

        let latest_block_number = cmd
            .cast_fuse()
            .args(["block-number", "--rpc-url", url])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .parse::<u64>()
            .expect("Failed to parse block number");

        let latest_block = cmd
            .cast_fuse()
            .args(["block", "latest", "--rpc-url", url, "--json"])
            .assert_success()
            .get_output()
            .stdout_lossy();

        let ts = serde_json::from_str::<serde_json::Value>(&latest_block)
            .expect("Failed to parse JSON")
            .get("timestamp")
            .and_then(|v| v.as_str())
            .and_then(|hex| u64::from_str_radix(hex.trim_start_matches("0x"), 16).ok())
            .expect("Failed to extract timestamp");

        let found_block = cmd
            .cast_fuse()
            .args(["find-block", &ts.to_string(), "--rpc-url", url])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .parse::<u64>()
            .expect("Failed to parse found block number");

        // The found block should be the same as or very close to the latest block
        assert!(
            found_block <= latest_block_number,
            "find-block({ts}) returned {found_block}, which is > latest block-number ({latest_block_number})"
        );
    }
});
