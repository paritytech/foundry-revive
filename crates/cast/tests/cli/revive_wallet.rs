use foundry_test_utils::{casttest, util::OutputExt};
use std::fs;
use tempfile::TempDir;

casttest!(test_cast_wallet_address, |_prj, cmd| {
    let pk = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    cmd.cast_fuse().args(["wallet", "address", "--private-key", pk]).assert_success().stdout_eq(
        str![[r#"
0x[..]

"#]],
    );
});

casttest!(test_cast_wallet_list, |_prj, cmd| {
    let tmp = TempDir::new().expect("tmpdir");
    let home = tmp.path();

    fs::create_dir_all(home.join("keystore")).expect("couldn't create keystore dir");
    cmd.env("FOUNDRY_HOME", home.to_str().unwrap());

    cmd.cast_fuse().args(["wallet", "list"]).assert_success().stdout_eq(str![[r#"
0x[..] (Local)
foo (Local)

"#]]);
});

casttest!(test_cast_wallet_import, |_prj, cmd| {
    let tmp = TempDir::new().unwrap();
    let keystore_dir = tmp.path().join("keystore");
    fs::create_dir_all(&keystore_dir).unwrap();
    let dir_arg = keystore_dir.to_str().unwrap();

    let dummy_pk = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    cmd.cast_fuse()
        .args([
            "wallet",
            "import",
            "--private-key",
            dummy_pk,
            "--unsafe-password",
            "testpass",
            "--keystore-dir",
            dir_arg,
            "foo",
        ])
        .assert_success()
        .stdout_eq(str![[r#"
`foo` keystore was saved successfully. [ADDRESS]

"#]]);
});

casttest!(test_cast_wallet_sign_verify, |_prj, cmd| {
    let pk = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    let address = cmd
        .cast_fuse()
        .args(["wallet", "address", "--private-key", pk])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    let msg = "hello, world";
    let sig = cmd
        .cast_fuse()
        .args(["wallet", "sign", "--private-key", pk, msg])
        .assert_success()
        .get_output()
        .stdout_lossy()
        .trim()
        .to_string();

    cmd.cast_fuse()
        .args(["wallet", "verify", "--address", &address, msg, &sig])
        .assert_success()
        .stdout_eq(str![[r#"
Validation succeeded. Address 0x[..] signed this message.

"#]]);
});
