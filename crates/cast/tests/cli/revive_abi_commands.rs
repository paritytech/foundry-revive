use foundry_test_utils::{casttest, util::OutputExt};

casttest!(test_abi_encode, async |_prj, cmd| {
    let out = cmd
        .cast_fuse()
        .args(["abi-encode", "foo(uint256)", "1"])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    let expected = "0x".to_owned()
        + "0000000000000000000000000000000000000000000000000000000000000001";
    assert_eq!(out, expected, "abi-encode uint256 failed");
});

casttest!(test_cast_sig, |_prj, cmd| {
    let out = cmd
        .cast_fuse()
        .args(["sig", "transfer(address,uint256)"])
        .assert_success()
        .get_output()
        .stdout_lossy();
    assert!(out.contains("0xa9059cbb"), "unexpected selector: {}", out);
});

casttest!(test_cast_4byte_lookup, |_prj, cmd| {
    let out = cmd
        .cast_fuse()
        .args(["4byte", "0xa9059cbb"])
        .assert_success()
        .get_output()
        .stdout_lossy();
    assert!(out.contains("transfer(address,uint256)"), "unexpected signature: {}", out);
});


casttest!(test_cast_4byte_calldata, |_prj, cmd| {
    let calldata = "0xa9059cbb000000000000000000000000e78388b4ce79068e89bf8aa7f218ef6b9ab0e9d00000000000000000000000000000000000000000000000000174b37380cea000";
    let out = cmd
        .cast_fuse()
        .args(["4byte-calldata", calldata])
        .assert_success()
        .get_output()
        .stdout_lossy();
    assert!(out.contains("transfer(address,uint256)"), "bad signature parse: {}", out);
});

casttest!(test_cast_4byte_event, |_prj, cmd| {
    let topic = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef";
    let out = cmd
        .cast_fuse()
        .args(["4byte-event", topic])
        .assert_success()
        .get_output()
        .stdout_lossy();
    assert!(
        out.contains("Transfer(address,address,uint256)"),
        "unexpected event signature: {}", out
    );
});

casttest!(test_cast_calldata_named, |_prj, cmd| {
    let sig = "balanceOf(address)";
    let data = "0xa16081f360e3847006db660bae1c6d1b2e17ec2a";
    let out = cmd
        .cast_fuse()
        .args(["calldata", sig, data])
        .assert_success()
        .get_output()
        .stdout_lossy();
    assert!(out.contains("0x70a08231000000000000000000000000a16081f360e3847006db660bae1c6d1b2e17ec2a"), "calldata command failed: {}", out);
});

casttest!(test_cast_decode_abi, |_prj, cmd| {
    let types = "transfer(address,uint256)";
    let data = concat!(
        "0xa9059cbb000000000000000000000000e78388b4ce79068e89bf8aa7f218ef6b9ab0e9d0000000000000000000000000000000000000000000000000008a8e4b1a3d8000"
    );
    let out = cmd
        .cast_fuse()
        .args(["decode-abi", "--input", types, data])
        .assert_success()
        .get_output()
        .stdout_lossy();
    assert!(out.contains("0x00000000E78388B4CE79068E89Bf8AA7F218ef6B"), "command decode-abi failed: {}", out);
});

casttest!(test_cast_decode_calldata, |_prj, cmd| {
    let sig = "transfer(address,uint256)";
    let data = concat!(
        "0xa9059cbb000000000000000000000000e78388b4ce79068e89bf8aa7f218ef6b9ab0e9d0000000000000000000000000000000000000000000000000008a8e4b1a3d8000"
    );
    let out = cmd
        .cast_fuse()
        .args(["decode-calldata", sig, data])
        .assert_success()
        .get_output()
        .stdout_lossy();
    assert!(out.contains("0xE78388b4CE79068e89Bf8aA7f218eF6b9AB0e9d0"), "command decode-calldata failed - : {}", out);
});

casttest!(test_cast_upload_signature, |_prj, cmd| {
    let out = cmd.cast_fuse()
        .args(["upload-signature", "spam(uint256,address)"])
        .assert_success()
        .get_output()
        .stdout_lossy();

    assert!(out.contains("spam(uint256,address)"), "missing signature name: {}", out);
    assert!(out.contains("Selectors successfully uploaded") || out.contains("Duplicated"), "unexpected upload result: {}", out);
});
