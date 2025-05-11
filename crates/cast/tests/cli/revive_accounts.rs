use foundry_test_utils::{
    casttest_serial,
    revive::PolkadotNode,
    util::OutputExt,
};

casttest_serial!(test_cast_balance, async |_prj, cmd| {
    let _node = PolkadotNode::start().await.unwrap();
    let url   = PolkadotNode::http_endpoint();

    let (account, _) = PolkadotNode::dev_accounts()
        .next()
        .expect("no dev accounts available");
    let account = account.to_string();

    let bal = cmd
        .cast_fuse()
        .args(["balance", "--rpc-url", url, &account])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    // wei equivalent    
    assert!(
        bal.parse::<u128>().is_ok(),
        "balance wasn’t a valid integer: `{}`",
        bal
    );
});

casttest_serial!(test_cast_nonce, async |_prj, cmd| {
    let _node = PolkadotNode::start().await.unwrap();
    let url   = PolkadotNode::http_endpoint();
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

    assert!(
        nonce.parse::<u64>().is_ok(),
        "nonce wasn’t a valid integer: `{}`",
        nonce
    );
});

casttest_serial!(test_cast_code, async |_prj, cmd| {
    let _node = PolkadotNode::start().await.unwrap();
    let url   = PolkadotNode::http_endpoint();
    let (account, _) = PolkadotNode::dev_accounts().next().unwrap();
    let account = account.to_string();

    let code = cmd
        .cast_fuse()
        .args(["code", "--rpc-url", url, &account])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    // Non‐contract accounts return "0x"
    assert!(
        code == "0x" || code.starts_with("0x"),
        "code should be hex, got `{}`",
        code
    );
});

casttest_serial!(test_cast_codesize, async |_prj, cmd| {
    let _node = PolkadotNode::start().await.unwrap();
    let url   = PolkadotNode::http_endpoint();
    let (account, _) = PolkadotNode::dev_accounts().next().unwrap();
    let account = account.to_string();

    let size = cmd
        .cast_fuse()
        .args(["codesize", "--rpc-url", url, &account])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    assert!(
        size.parse::<u64>().is_ok(),
        "codesize wasn’t a valid integer: `{}`",
        size
    );
});

casttest_serial!(test_cast_storage, async |_prj, cmd| {
    let _node = PolkadotNode::start().await.unwrap();
    let url   = PolkadotNode::http_endpoint();
    let (account, _) = PolkadotNode::dev_accounts().next().unwrap();
    let account = account.to_string();

    let val = cmd
        .cast_fuse()
        .args(["storage", "--rpc-url", url, &account, "0x0"])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    assert!(
        val.starts_with("0x"),
        "storage didn’t return hex: `{}`",
        val
    );
});
