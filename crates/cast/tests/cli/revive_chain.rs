use foundry_test_utils::{
    casttest,
    revive::PolkadotNode,
    util::OutputExt,
};

casttest!(test_cast_chain_id, async |_prj, cmd| {
    let _node   = PolkadotNode::start().await.expect("failed to start node");
    let rpc_url = PolkadotNode::http_endpoint();
    let id = cmd
        .cast_fuse()
        .args(["chain-id", "--rpc-url", &rpc_url])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    assert!(
        id.parse::<u64>().is_ok(),
        "chain-id wasn’t a number: {id}"
    );
});

casttest!(test_cast_chain, async |_prj, cmd| {
    let _node   = PolkadotNode::start().await.expect("failed to start node");
    let rpc_url = PolkadotNode::http_endpoint();
    let name = cmd
        .cast_fuse()
        .args(["chain", "--rpc-url", &rpc_url])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    assert_eq!(
        name,
        "unknown",
        "cast chain should return \"unknown\" for an unrecognized chain ID"
    );
});

casttest!(test_cast_client, async |_prj, cmd| {
    let _node   = PolkadotNode::start().await.expect("failed to start node");
    let rpc_url = PolkadotNode::http_endpoint();
    let version = cmd
        .cast_fuse()
        .args(["client", "--rpc-url", &rpc_url])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    assert!(
        !version.is_empty(),
        "Expected non-empty client version, got `{version}`"
    );
});
