//! Tests for the `forge compiler` command.

use foundry_test_utils::snapbox::IntoData;
pub const OTHER_RESOLC_VERSION: &str = "0.1.0-dev.13";

const CONTRACT_A: &str = r#"
// SPDX-license-identifier: MIT
pragma solidity 0.8.4;

contract ContractA {}
"#;

const CONTRACT_B: &str = r#"
// SPDX-license-identifier: MIT
pragma solidity 0.8.11;

contract ContractB {}
"#;

const CONTRACT_C: &str = r#"
// SPDX-license-identifier: MIT
pragma solidity 0.8.27;

contract ContractC {}
"#;

const CONTRACT_D: &str = r#"
// SPDX-license-identifier: MIT
pragma solidity 0.8.27;

contract ContractD {}
"#;

forgetest!(can_resolve_path, |prj, cmd| {
    prj.update_config(|config| {
        config.resolc.resolc = Some(foundry_config::SolcReq::Version(
            semver::Version::parse(OTHER_RESOLC_VERSION).unwrap(),
        ));
        config.resolc.resolc_compile = true;
    });

    prj.add_source("ContractA", CONTRACT_A).unwrap();

    cmd.args(["compiler", "resolve", "--root", prj.root().to_str().unwrap()])
        .assert_success()
        .stdout_eq(str![[r#"
Solidity:
- Resolc v0.1.0-dev.13 and Solc v0.8.4


"#]]);
});

forgetest!(can_list_resolved_compiler_versions, |prj, cmd| {
    prj.update_config(|config| {
        config.resolc.resolc = Some(foundry_config::SolcReq::Version(
            semver::Version::parse(OTHER_RESOLC_VERSION).unwrap(),
        ));
        config.resolc.resolc_compile = true;
    });
    prj.add_source("ContractA", CONTRACT_A).unwrap();

    cmd.args(["compiler", "resolve"]).assert_success().stdout_eq(str![[r#"
Solidity:
- Resolc v0.1.0-dev.13 and Solc v0.8.4


"#]]);
});

forgetest!(can_list_resolved_compiler_versions_json, |prj, cmd| {
    prj.update_config(|config| {
        config.resolc.resolc = Some(foundry_config::SolcReq::Version(
            semver::Version::parse(OTHER_RESOLC_VERSION).unwrap(),
        ));
        config.resolc.resolc_compile = true;
    });
    prj.add_source("ContractA", CONTRACT_A).unwrap();

    cmd.args(["compiler", "resolve", "--json"]).assert_success().stdout_eq(
        str![[r#"
{
  "Solidity": [
    {
      "name": "Resolc",
      "version": "0.1.0-dev.13",
      "dep": [
        "Solc",
        "0.8.4"
      ]
    }
  ]
}
"#]]
        .is_json(),
    );
});

forgetest!(can_list_resolved_compiler_versions_verbose, |prj, cmd| {
    prj.update_config(|config| {
        config.resolc.resolc = Some(foundry_config::SolcReq::Version(
            semver::Version::parse(OTHER_RESOLC_VERSION).unwrap(),
        ));
        config.resolc.resolc_compile = true;
    });
    prj.add_source("ContractC", CONTRACT_C).unwrap();
    prj.add_source("ContractD", CONTRACT_D).unwrap();

    cmd.args(["compiler", "resolve", "-v"]).assert_success().stdout_eq(str![[r#"
Solidity:

Resolc v0.1.0-dev.13 and Solc v0.8.27:
├── src/ContractC.sol
└── src/ContractD.sol


"#]]);
});

forgetest!(can_list_resolved_compiler_versions_verbose_json, |prj, cmd| {
    prj.update_config(|config| {
        config.resolc.resolc = Some(foundry_config::SolcReq::Version(
            semver::Version::parse(OTHER_RESOLC_VERSION).unwrap(),
        ));
        config.resolc.resolc_compile = true;
    });
    prj.add_source("ContractC", CONTRACT_C).unwrap();
    prj.add_source("ContractD", CONTRACT_D).unwrap();

    cmd.args(["compiler", "resolve", "--json", "-v"]).assert_success().stdout_eq(
        str![[r#"
{
  "Solidity": [
    {
      "name": "Resolc",
      "version": "0.1.0-dev.13",
      "paths": [
        "src/ContractC.sol",
        "src/ContractD.sol"
      ],
      "dep": [
        "Solc",
        "0.8.27"
      ]
    }
  ]
}
"#]]
        .is_json(),
    );
});

forgetest!(can_list_resolved_multiple_compiler_versions, |prj, cmd| {
    prj.update_config(|config| {
        config.resolc.resolc = Some(foundry_config::SolcReq::Version(
            semver::Version::parse(OTHER_RESOLC_VERSION).unwrap(),
        ));
        config.resolc.resolc_compile = true;
    });
    prj.add_source("ContractA", CONTRACT_A).unwrap();
    prj.add_source("ContractB", CONTRACT_B).unwrap();
    prj.add_source("ContractC", CONTRACT_C).unwrap();
    prj.add_source("ContractD", CONTRACT_D).unwrap();

    cmd.args(["compiler", "resolve"]).assert_success().stdout_eq(str![[r#"
Solidity:
- Resolc v0.1.0-dev.13 and Solc v0.8.4
- Resolc v0.1.0-dev.13 and Solc v0.8.11
- Resolc v0.1.0-dev.13 and Solc v0.8.27


"#]]);
});

forgetest!(can_list_resolved_multiple_compiler_versions_skipped_json, |prj, cmd| {
    prj.update_config(|config| {
        config.resolc.resolc = Some(foundry_config::SolcReq::Version(
            semver::Version::parse(OTHER_RESOLC_VERSION).unwrap(),
        ));
        config.resolc.resolc_compile = true;
    });
    prj.add_source("ContractA", CONTRACT_A).unwrap();
    prj.add_source("ContractB", CONTRACT_B).unwrap();
    prj.add_source("ContractC", CONTRACT_C).unwrap();
    prj.add_source("ContractD", CONTRACT_D).unwrap();

    cmd.args(["compiler", "resolve", "--skip", "Contract(A|B|C)", "--json", "-v"])
        .assert_success()
        .stdout_eq(
            str![[r#"
{
  "Solidity": [
    {
      "name": "Resolc",
      "version": "0.1.0-dev.13",
      "paths": [
        "src/ContractD.sol"
      ],
      "dep": [
        "Solc",
        "0.8.27"
      ]
    }
  ]
}
"#]]
            .is_json(),
        );
});

forgetest!(can_list_resolved_multiple_compiler_versions_verbose, |prj, cmd| {
    prj.update_config(|config| {
        config.resolc.resolc = Some(foundry_config::SolcReq::Version(
            semver::Version::parse(OTHER_RESOLC_VERSION).unwrap(),
        ));
        config.resolc.resolc_compile = true;
    });
    prj.add_source("ContractA", CONTRACT_A).unwrap();
    prj.add_source("ContractB", CONTRACT_B).unwrap();
    prj.add_source("ContractC", CONTRACT_C).unwrap();
    prj.add_source("ContractD", CONTRACT_D).unwrap();

    cmd.args(["compiler", "resolve", "-vv"]).assert_success().stdout_eq(str![[r#"
Solidity:

Resolc v0.1.0-dev.13 and Solc v0.8.4 (<= istanbul):
└── src/ContractA.sol

Resolc v0.1.0-dev.13 and Solc v0.8.11 (<= london):
└── src/ContractB.sol

Resolc v0.1.0-dev.13 and Solc v0.8.27 (<= cancun):
├── src/ContractC.sol
└── src/ContractD.sol


"#]]);
});

forgetest!(can_list_resolved_multiple_compiler_versions_verbose_json, |prj, cmd| {
    prj.update_config(|config| {
        config.resolc.resolc = Some(foundry_config::SolcReq::Version(
            semver::Version::parse(OTHER_RESOLC_VERSION).unwrap(),
        ));
        config.resolc.resolc_compile = true;
    });
    prj.add_source("ContractA", CONTRACT_A).unwrap();
    prj.add_source("ContractB", CONTRACT_B).unwrap();
    prj.add_source("ContractC", CONTRACT_C).unwrap();
    prj.add_source("ContractD", CONTRACT_D).unwrap();

    cmd.args(["compiler", "resolve", "--json", "-vv"]).assert_success().stdout_eq(
        str![[r#"
{
  "Solidity": [
    {
      "name": "Resolc",
      "version": "0.1.0-dev.13",
      "evm_version": "Istanbul",
      "paths": [
        "src/ContractA.sol"
      ],
      "dep": [
        "Solc",
        "0.8.4"
      ]
    },
    {
      "name": "Resolc",
      "version": "0.1.0-dev.13",
      "evm_version": "London",
      "paths": [
        "src/ContractB.sol"
      ],
      "dep": [
        "Solc",
        "0.8.11"
      ]
    },
    {
      "name": "Resolc",
      "version": "0.1.0-dev.13",
      "evm_version": "Cancun",
      "paths": [
        "src/ContractC.sol",
        "src/ContractD.sol"
      ],
      "dep": [
        "Solc",
        "0.8.27"
      ]
    }
  ]
}
"#]]
        .is_json(),
    );
});
