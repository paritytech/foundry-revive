use foundry_test_utils::{
    casttest_serial,
    revive::PolkadotNode,
    rpc::next_mainnet_etherscan_api_key,
    util::{block_on, OutputExt},
};

casttest_serial!(test_cast_balance, |_prj, cmd| {
    if let Ok(_node) = block_on(PolkadotNode::start()) {
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

        // wei equivalent
        assert!(bal.parse::<u128>().is_ok(), "balance wasn't a valid integer: `{bal}`");
    }
});

casttest_serial!(test_cast_nonce, |_prj, cmd| {
    if let Ok(_node) = block_on(PolkadotNode::start()) {
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
    if let Ok(_node) = block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();
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

        assert!(code == "0x" || code.starts_with("0x"), "code should be hex, got `{code}`");
    }
});

casttest_serial!(test_cast_codesize, |_prj, cmd| {
    if let Ok(_node) = block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();
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

        assert!(size.parse::<u64>().is_ok(), "codesize wasn't a valid integer: `{size}`");
    }
});

casttest_serial!(test_cast_storage, |_prj, cmd| {
    if let Ok(_node) = block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();
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

        assert!(val.starts_with("0x"), "storage didn't return hex: `{val}`");
    }
});

casttest_serial!(storage_layout_simple_json, |_prj, cmd| {
    if let Ok(_node) = block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();
        cmd.cast_fuse().args([
            "storage",
            "--rpc-url",
            url,
            "--block",
            "21034138",
            "--etherscan-api-key",
            next_mainnet_etherscan_api_key().as_str(),
            "0x13b0D85CcB8bf860b6b79AF3029fCA081AE9beF2",
            "--json",
        ])
        .assert_success()
        .stdout_eq(file!["../fixtures/storage_layout_simple.json": Json]);
    }
});

casttest_serial!(cast_storage_layout_complex, |_prj, cmd| {
    if let Ok(_node) = block_on(PolkadotNode::start()) {
        let url = PolkadotNode::http_endpoint();
        cmd.cast_fuse().args([
        "storage",
        "--rpc-url",
        url,
        "--block",
        "21034138",
        "--etherscan-api-key",
        next_mainnet_etherscan_api_key().as_str(),
        "0xBA12222222228d8Ba445958a75a0704d566BF2C8",
    ])
    .assert_success()
    .stdout_eq(str![[r#"

╭-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------╮
| Name                          | Type                                                               | Slot | Offset | Bytes | Value                                            | Hex Value                                                          | Contract                        |
+======================================================================================================================================================================================================================================================================================+
| _status                       | uint256                                                            | 0    | 0      | 32    | 1                                                | 0x0000000000000000000000000000000000000000000000000000000000000001 | contracts/vault/Vault.sol:Vault |
|-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------|
| _generalPoolsBalances         | mapping(bytes32 => struct EnumerableMap.IERC20ToBytes32Map)        | 1    | 0      | 32    | 0                                                | 0x0000000000000000000000000000000000000000000000000000000000000000 | contracts/vault/Vault.sol:Vault |
|-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------|
| _nextNonce                    | mapping(address => uint256)                                        | 2    | 0      | 32    | 0                                                | 0x0000000000000000000000000000000000000000000000000000000000000000 | contracts/vault/Vault.sol:Vault |
|-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------|
| _paused                       | bool                                                               | 3    | 0      | 1     | 0                                                | 0x0000000000000000000000000000000000000000000000000000000000000000 | contracts/vault/Vault.sol:Vault |
|-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------|
| _authorizer                   | contract IAuthorizer                                               | 3    | 1      | 20    | 549683469959765988649777481110995959958745616871 | 0x0000000000000000000000006048a8c631fb7e77eca533cf9c29784e482391e7 | contracts/vault/Vault.sol:Vault |
|-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------|
| _approvedRelayers             | mapping(address => mapping(address => bool))                       | 4    | 0      | 32    | 0                                                | 0x0000000000000000000000000000000000000000000000000000000000000000 | contracts/vault/Vault.sol:Vault |
|-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------|
| _isPoolRegistered             | mapping(bytes32 => bool)                                           | 5    | 0      | 32    | 0                                                | 0x0000000000000000000000000000000000000000000000000000000000000000 | contracts/vault/Vault.sol:Vault |
|-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------|
| _nextPoolNonce                | uint256                                                            | 6    | 0      | 32    | 1760                                             | 0x00000000000000000000000000000000000000000000000000000000000006e0 | contracts/vault/Vault.sol:Vault |
|-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------|
| _minimalSwapInfoPoolsBalances | mapping(bytes32 => mapping(contract IERC20 => bytes32))            | 7    | 0      | 32    | 0                                                | 0x0000000000000000000000000000000000000000000000000000000000000000 | contracts/vault/Vault.sol:Vault |
|-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------|
| _minimalSwapInfoPoolsTokens   | mapping(bytes32 => struct EnumerableSet.AddressSet)                | 8    | 0      | 32    | 0                                                | 0x0000000000000000000000000000000000000000000000000000000000000000 | contracts/vault/Vault.sol:Vault |
|-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------|
| _twoTokenPoolTokens           | mapping(bytes32 => struct TwoTokenPoolsBalance.TwoTokenPoolTokens) | 9    | 0      | 32    | 0                                                | 0x0000000000000000000000000000000000000000000000000000000000000000 | contracts/vault/Vault.sol:Vault |
|-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------|
| _poolAssetManagers            | mapping(bytes32 => mapping(contract IERC20 => address))            | 10   | 0      | 32    | 0                                                | 0x0000000000000000000000000000000000000000000000000000000000000000 | contracts/vault/Vault.sol:Vault |
|-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------|
| _internalTokenBalance         | mapping(address => mapping(contract IERC20 => uint256))            | 11   | 0      | 32    | 0                                                | 0x0000000000000000000000000000000000000000000000000000000000000000 | contracts/vault/Vault.sol:Vault |
╰-------------------------------+--------------------------------------------------------------------+------+--------+-------+--------------------------------------------------+--------------------------------------------------------------------+---------------------------------╯


"#]]);
    }
});
