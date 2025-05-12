use foundry_test_utils::{casttest, util::OutputExt};
use std::{fs, path::Path};
use tempfile::TempDir;

casttest!(
    test_cast_wallet_new,
    |_prj, cmd| {
        let tmp = TempDir::new().expect("tmpdir");
        let dir = tmp.path().to_str().unwrap();

        let stdout = cmd
            .cast_fuse()
            .args(["wallet", "new", "--unsafe-password", "testpass", dir])
            .assert_success()
            .get_output()
            .stdout_lossy();

        let first_line = stdout.lines().next().expect("no output from wallet new");
        let prefix = "Created new encrypted keystore file: ";
        let path =
            first_line.strip_prefix(prefix).expect("unexpected output format for wallet new");

        assert!(Path::new(path).exists(), "keystore file was not created: {}", path);
    }
);

casttest!(
    test_cast_wallet_address,
    |_prj, cmd| {

        let pk = "0x0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

        let addr = cmd
            .cast_fuse()
            .args(["wallet", "address", "--private-key", pk])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(
            addr.starts_with("0x") && addr.len() == 42,
            "wallet address has wrong format: {}",
            addr
        );

        assert!(
            addr.chars().skip(2).all(|c| c.is_ascii_hexdigit()),
            "address contains non-hex character: {}",
            addr
        );
    }
);

casttest!(
    test_cast_wallet_list,
    |_prj, cmd| {

        let tmp = TempDir::new().expect("tmpdir");
        let home = tmp.path();

        fs::create_dir_all(home.join("keystore")).expect("couldn't create keystore dir");
        cmd.env("FOUNDRY_HOME", home.to_str().unwrap());


        let out = cmd
            .cast_fuse()
            .args(["wallet", "list"])
            .assert_success()
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(
            out.contains("(Local)"),
            "expected to see the built-in Local account, got `{}`",
            out
        );
    }
);

casttest!(
    test_cast_wallet_import,
    |_prj, cmd| {

        let tmp = TempDir::new().unwrap();
        let keystore_dir = tmp.path().join("keystore");
        fs::create_dir_all(&keystore_dir).unwrap();
        let dir_arg = keystore_dir.to_str().unwrap();

        let dummy_pk = "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

        let out = cmd
            .cast_fuse()
            .args(&[
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
            .get_output()
            .stdout_lossy()
            .trim()
            .to_string();

        assert!(
            out.contains("`foo` keystore was saved successfully"),
            "`foo` keystore was not saved successfully"
        );
    }
);

casttest!(
    test_cast_wallet_sign_verify,
    |_prj, cmd| {
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
            .args(&["wallet", "verify", "--address", &address, msg, &sig])
            .assert_success();
    }
);
