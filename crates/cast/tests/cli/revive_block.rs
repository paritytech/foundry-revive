use foundry_test_utils::{casttest, revive::PolkadotNode, util::OutputExt};
use serial_test::serial;


casttest!(#[serial] test_cast_block_number, async |_prj, cmd| {
    let _node = PolkadotNode::start().await.expect("failed to start node");
    let url = PolkadotNode::http_endpoint();
    let bn = cmd
        .cast_fuse()
        .args(["block-number", "--rpc-url", url])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    assert!(bn.parse::<u64>().is_ok(), "block-number output not a valid integer: `{}`", bn);
});

casttest!(#[serial] test_cast_gas_price, async |_prj, cmd| {
    let _node = PolkadotNode::start().await.expect("failed to start node");
    let url = PolkadotNode::http_endpoint();

    let gp = cmd
        .cast_fuse()
        .args(["gas-price", "--rpc-url", url])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    assert!(gp.parse::<u128>().is_ok(), "gas-price output not a valid integer: `{}`", gp);
});

casttest!(#[serial] test_cast_basefee, async |_prj, cmd| {
    let _node = PolkadotNode::start().await.expect("failed to start node");
    let url = PolkadotNode::http_endpoint();

    let bf = cmd
        .cast_fuse()
        .args(["basefee", "--rpc-url", url])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    assert!(bf.parse::<u128>().is_ok(), "basefee output not a valid integer: `{}`", bf);
});

casttest!(#[serial] test_cast_block, async |_prj, cmd| {
    let _node = PolkadotNode::start().await.expect("failed to start node");
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
        "block info missing fields: `{}`",
        info
    );
});

casttest!(#[serial] test_cast_age, async |_prj, cmd| {
    let _node = PolkadotNode::start().await.expect("failed to start node");
    let url = PolkadotNode::http_endpoint();

    let age = cmd
        .cast_fuse()
        .args(["age", "latest", "--rpc-url", url])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    assert!(age.ends_with("UTC"), "age output not a human timestamp ending in UTC: `{}`", age);
});

casttest!(#[serial] test_cast_find_block, async |_prj, cmd| {
    let _node = PolkadotNode::start().await.expect("failed to start node");
    let url = PolkadotNode::http_endpoint();

    let bn = cmd
        .cast_fuse()
        .args(["block-number", "--rpc-url", url])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .parse::<u64>()
        .unwrap();

    let ts_hex = cmd
        .cast_fuse()
        .args(["block", "latest", "-f", "timestamp", "--rpc-url", url])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    let ts = u64::from_str_radix(ts_hex.trim_start_matches("0x"), 16).unwrap();

    let fb = cmd
        .cast_fuse()
        .args(["find-block", &ts.to_string(), "--rpc-url", url])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .parse::<u64>()
        .unwrap();

    assert!(
        fb <= bn,
        "find-block({}) returned {}, which is > latest block-number ({})",
        ts,
        fb,
        bn
    );
});
