use foundry_test_utils::{casttest_serial, revive::PolkadotNode, util::OutputExt};

casttest_serial!(test_cast_chain_id, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let rpc_url = PolkadotNode::http_endpoint();
        let id = cmd
            .cast_fuse()
            .args(["chain-id", "--rpc-url", rpc_url])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(id.parse::<u64>().is_ok(), "chain-id wasn't a number: {id}");
    }
});

casttest_serial!(test_cast_chain, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let rpc_url = PolkadotNode::http_endpoint();
        let name = cmd
            .cast_fuse()
            .args(["chain", "--rpc-url", rpc_url])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert_eq!(
            name, "unknown",
            "cast chain should return \"unknown\" for an unrecognized chain ID"
        );
    }
});

casttest_serial!(test_cast_client, |_prj, cmd| {
    if let Ok(_node) = tokio::runtime::Runtime::new().unwrap().block_on(PolkadotNode::start()) {
        let rpc_url = PolkadotNode::http_endpoint();
        let version = cmd
            .cast_fuse()
            .args(["client", "--rpc-url", rpc_url])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(!version.is_empty(), "Expected non-empty client version, got `{version}`");
    }
});
