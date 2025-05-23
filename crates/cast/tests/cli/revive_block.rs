use foundry_test_utils::{casttest_serial, revive::PolkadotNode, util::OutputExt};

casttest_serial!(test_cast_block_number, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();
        let bn = cmd
            .cast_fuse()
            .args(["block-number", "--rpc-url", url])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(bn.parse::<u64>().is_ok(), "block-number output not a valid integer: `{bn}`");
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

        let bf = cmd
            .cast_fuse()
            .args(["basefee", "--rpc-url", url])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(bf.parse::<u128>().is_ok(), "basefee output not a valid integer: `{bf}`");
    }
});

casttest_serial!(test_cast_block, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();

        let info = cmd
            .cast_fuse()
            .args(["block", "latest", "--rpc-url", url])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .to_lowercase();

        assert!(
            info.contains("number") && info.contains("hash"),
            "block info missing fields: `{info}`"
        );
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

        let bn = cmd
            .cast_fuse()
            .args(["block-number", "--rpc-url", url])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .parse::<u64>()
            .expect("Failed to parse block number");

        let timestamp = cmd
            .cast_fuse()
            .args(["block", "latest", "--rpc-url", url, "--json"])
            .assert_success()
            .get_output()
            .stdout_lossy();

        let ts = serde_json::from_str::<serde_json::Value>(&timestamp)
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
            found_block <= bn,
            "find-block({}) returned {}, which is > latest block-number ({})",
            ts,
            found_block,
            bn
        );
    }
});
