use std::{fs, path::Path};

use foundry_compilers::artifacts::{ConfigurableContractArtifact, Metadata, Remapping};
use foundry_config::{parse_with_profile, BasicConfig, Config, SolidityErrorCode};
use foundry_test_utils::{snapbox::IntoData, util::{pretty_err, read_string}};
use crate::constants::*;
use std::str::FromStr;

// checks that `clean` also works with the "out" value set in Config
forgetest_init!(can_clean_revive_config, |prj, cmd| {
    prj.update_config(|config| config.out = "custom-out".into());
    cmd.args(["build", "--revive"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    // default test contract is written in custom out directory
    let artifact = prj.root().join(format!("custom-out/{TEMPLATE_TEST_CONTRACT_ARTIFACT_JSON}"));
    assert!(artifact.exists());

    cmd.forge_fuse().arg("clean").assert_empty_stdout();
    assert!(!artifact.exists());
});

// checks that extra output works
forgetest_init!(can_emit_extra_output, |prj, cmd| {
    prj.clear();

    cmd.args(["build", "--revive", "--extra-output", "metadata"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    let artifact_path = prj.paths().artifacts.join(TEMPLATE_CONTRACT_ARTIFACT_JSON);
    let artifact: ConfigurableContractArtifact =
        foundry_compilers::utils::read_json_file(&artifact_path).unwrap();
    assert!(artifact.metadata.is_some());

    cmd.forge_fuse()
        .args(["build", "--revive", "--extra-output-files", "metadata", "--force"])
        .root_arg()
        .assert_success()
        .stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    let metadata_path =
        prj.paths().artifacts.join(format!("{TEMPLATE_CONTRACT_ARTIFACT_BASE}.metadata.json"));
    let _artifact: Metadata = foundry_compilers::utils::read_json_file(&metadata_path).unwrap();
});

// checks that extra output works
forgetest_init!(can_emit_multiple_extra_output, |prj, cmd| {
    cmd.args([
        "build",
        "--revive",
        "--extra-output",
        "metadata",
        "legacyAssembly",
        "ir-optimized",
        "--extra-output",
        "ir",
    ])
    .assert_success()
    .stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    let artifact_path = prj.paths().artifacts.join(TEMPLATE_CONTRACT_ARTIFACT_JSON);
    let artifact: ConfigurableContractArtifact =
        foundry_compilers::utils::read_json_file(&artifact_path).unwrap();
    assert!(artifact.metadata.is_some());
    assert!(artifact.legacy_assembly.is_some());
    assert!(artifact.ir.is_some());
    assert!(artifact.ir_optimized.is_some());

    cmd.forge_fuse()
        .args([
            "build",
            "--revive",
            "--extra-output-files",
            "metadata",
            "ir-optimized",
            "evm.bytecode.sourceMap",
            "evm.legacyAssembly",
            "--force",
        ])
        .root_arg()
        .assert_success()
        .stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    let metadata_path =
        prj.paths().artifacts.join(format!("{TEMPLATE_CONTRACT_ARTIFACT_BASE}.metadata.json"));
    let _artifact: Metadata = foundry_compilers::utils::read_json_file(&metadata_path).unwrap();

    let iropt = prj.paths().artifacts.join(format!("{TEMPLATE_CONTRACT_ARTIFACT_BASE}.iropt"));
    std::fs::read_to_string(iropt).unwrap();

    let sourcemap =
        prj.paths().artifacts.join(format!("{TEMPLATE_CONTRACT_ARTIFACT_BASE}.sourcemap"));
    std::fs::read_to_string(sourcemap).unwrap();

    let legacy_assembly = prj
        .paths()
        .artifacts
        .join(format!("{TEMPLATE_CONTRACT_ARTIFACT_BASE}.legacyAssembly.json"));
    std::fs::read_to_string(legacy_assembly).unwrap();
});

forgetest!(can_print_warnings, |prj, cmd| {
    prj.add_source(
        "Foo",
        r"
contract Greeter {
    function foo(uint256 a) public {
        uint256 x = 1;
    }
}
   ",
    )
    .unwrap();

    cmd.arg("build").assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful with warnings:
Warning (5667): Unused function parameter. Remove or comment out the variable name to silence this warning.
 [FILE]:5:18:
  |
5 |     function foo(uint256 a) public {
  |                  ^^^^^^^^^

Warning (2072): Unused local variable.
 [FILE]:6:9:
  |
6 |         uint256 x = 1;
  |         ^^^^^^^^^

Warning (2018): Function state mutability can be restricted to pure
 [FILE]:5:5:
  |
5 |     function foo(uint256 a) public {
  |     ^ (Relevant source part starts here and spans across multiple lines).


"#]]);
});

// Tests that direct import paths are handled correctly
forgetest!(can_handle_direct_imports_into_src, |prj, cmd| {
    prj.add_source(
        "Foo",
        r#"
import {FooLib} from "src/FooLib.sol";
struct Bar {
    uint8 x;
}
contract Foo {
    mapping(uint256 => Bar) bars;
    function checker(uint256 id) external {
        Bar memory b = bars[id];
        FooLib.check(b);
    }
    function checker2() external {
        FooLib.check2(this);
    }
}
   "#,
    )
    .unwrap();

    prj.add_source(
        "FooLib",
        r#"
import {Foo, Bar} from "src/Foo.sol";
library FooLib {
    function check(Bar memory b) public {}
    function check2(Foo f) public {}
}
   "#,
    )
    .unwrap();

    cmd.arg("build").assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);
});

// test that `forge build` does not print `(with warnings)` if file path is ignored
forgetest!(can_compile_without_warnings_ignored_file_paths, |prj, cmd| {
    // Ignoring path and setting empty error_codes as default would set would set some error codes
    prj.update_config(|config| {
        config.ignored_file_paths = vec![Path::new("src").to_path_buf()];
        config.ignored_error_codes = vec![];
    });

    prj.add_raw_source(
        "src/example.sol",
        r"
pragma solidity *;
contract A {
    function testExample() public {}
}
",
    )
    .unwrap();

    cmd.args(["build", "--revive", "--force"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    // Reconfigure without ignored paths or error codes and check for warnings
    prj.update_config(|config| config.ignored_file_paths = vec![]);

    cmd.forge_fuse().args(["build", "--revive", "--force"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful with warnings:
Warning (1878): SPDX license identifier not provided in source file. Before publishing, consider adding a comment containing "SPDX-License-Identifier: <SPDX-License>" to each source file. Use "SPDX-License-Identifier: UNLICENSED" for non-open-source code. Please see https://spdx.org for more information.
Warning: SPDX license identifier not provided in source file. Before publishing, consider adding a comment containing "SPDX-License-Identifier: <SPDX-License>" to each source file. Use "SPDX-License-Identifier: UNLICENSED" for non-open-source code. Please see https://spdx.org for more information.
[FILE]


"#]]);
});

// test that `forge build` does not print `(with warnings)` if there aren't any
forgetest!(can_compile_without_warnings, |prj, cmd| {
    prj.update_config(|config| {
        config.ignored_error_codes = vec![SolidityErrorCode::SpdxLicenseNotProvided];
    });
    prj.add_raw_source(
        "A",
        r"
pragma solidity *;
contract A {
    function testExample() public {}
}
   ",
    )
    .unwrap();

    cmd.args(["build", "--revive", "--force"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    // don't ignore errors
    prj.update_config(|config| {
        config.ignored_error_codes = vec![];
    });

    cmd.forge_fuse().args(["build", "--revive", "--force"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful with warnings:
Warning (1878): SPDX license identifier not provided in source file. Before publishing, consider adding a comment containing "SPDX-License-Identifier: <SPDX-License>" to each source file. Use "SPDX-License-Identifier: UNLICENSED" for non-open-source code. Please see https://spdx.org for more information.
Warning: SPDX license identifier not provided in source file. Before publishing, consider adding a comment containing "SPDX-License-Identifier: <SPDX-License>" to each source file. Use "SPDX-License-Identifier: UNLICENSED" for non-open-source code. Please see https://spdx.org for more information.
[FILE]


"#]]);
});

// test that `forge build` compiles when severity set to error, fails when set to warning, and
// handles ignored error codes as an exception
forgetest!(can_fail_compile_with_warnings, |prj, cmd| {
    prj.update_config(|config| {
        config.ignored_error_codes = vec![];
        config.deny_warnings = false;
    });
    prj.add_raw_source(
        "A",
        r"
pragma solidity *;
contract A {
    function testExample() public {}
}
   ",
    )
    .unwrap();

    // there are no errors
    cmd.args(["build", "--revive", "--force"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful with warnings:
Warning (1878): SPDX license identifier not provided in source file. Before publishing, consider adding a comment containing "SPDX-License-Identifier: <SPDX-License>" to each source file. Use "SPDX-License-Identifier: UNLICENSED" for non-open-source code. Please see https://spdx.org for more information.
Warning: SPDX license identifier not provided in source file. Before publishing, consider adding a comment containing "SPDX-License-Identifier: <SPDX-License>" to each source file. Use "SPDX-License-Identifier: UNLICENSED" for non-open-source code. Please see https://spdx.org for more information.
[FILE]


"#]]);

    // warning fails to compile
    prj.update_config(|config| {
        config.ignored_error_codes = vec![];
        config.deny_warnings = true;
    });

    cmd.forge_fuse().args(["build", "--revive", "--force"]).assert_failure().stderr_eq(str![[r#"
Error: Compiler run failed:
Warning (1878): SPDX license identifier not provided in source file. Before publishing, consider adding a comment containing "SPDX-License-Identifier: <SPDX-License>" to each source file. Use "SPDX-License-Identifier: UNLICENSED" for non-open-source code. Please see https://spdx.org for more information.
Warning: SPDX license identifier not provided in source file. Before publishing, consider adding a comment containing "SPDX-License-Identifier: <SPDX-License>" to each source file. Use "SPDX-License-Identifier: UNLICENSED" for non-open-source code. Please see https://spdx.org for more information.
[FILE]

"#]]);

    // ignores error code and compiles
    prj.update_config(|config| {
        config.ignored_error_codes = vec![SolidityErrorCode::SpdxLicenseNotProvided];
        config.deny_warnings = true;
    });

    cmd.forge_fuse().args(["build", "--revive", "--force"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);
});

// test that a failing `forge build` does not impact followup builds
forgetest!(can_build_after_failure, |prj, cmd| {
    prj.insert_ds_test();

    prj.add_source(
        "ATest.t.sol",
        r#"
import "./test.sol";
contract ATest is DSTest {
    function testExample() public {
        assertTrue(true);
    }
}
   "#,
    )
    .unwrap();
    prj.add_source(
        "BTest.t.sol",
        r#"
import "./test.sol";
contract BTest is DSTest {
    function testExample() public {
        assertTrue(true);
    }
}
   "#,
    )
    .unwrap();

    cmd.args(["build", "--revive"] ).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
...
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    prj.assert_cache_exists();
    prj.assert_artifacts_dir_exists();

    let syntax_err = r#"
import "./test.sol";
contract CTest is DSTest {
    function testExample() public {
        THIS WILL CAUSE AN ERROR
    }
}
   "#;

    // introduce contract with syntax error
    prj.add_source("CTest.t.sol", syntax_err).unwrap();

    // `forge build --force` which should fail
    cmd.forge_fuse().args(["build", "--revive", "--force"]).assert_failure().stderr_eq(str![[r#"
Error: Compiler run failed:
Error (2314): Expected ';' but got identifier
 [FILE]:7:19:
  |
7 |         THIS WILL CAUSE AN ERROR
  |                   ^^^^^

"#]]);

    // but ensure this cleaned cache and artifacts
    assert!(!prj.paths().artifacts.exists());
    assert!(!prj.cache().exists());

    // still errors
    cmd.forge_fuse().args(["build", "--revive", "--force"]).assert_failure().stderr_eq(str![[r#"
Error: Compiler run failed:
Error (2314): Expected ';' but got identifier
 [FILE]:7:19:
  |
7 |         THIS WILL CAUSE AN ERROR
  |                   ^^^^^

"#]]);

    // resolve the error by replacing the file
    prj.add_source(
        "CTest.t.sol",
        r#"
import "./test.sol";
contract CTest is DSTest {
    function testExample() public {
         assertTrue(true);
    }
}
   "#,
    )
    .unwrap();

    cmd.forge_fuse().args(["build", "--revive", "--force"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    prj.assert_cache_exists();
    prj.assert_artifacts_dir_exists();

    // ensure cache is unchanged after error
    let cache = fs::read_to_string(prj.cache()).unwrap();

    // introduce the error again but building without force
    prj.add_source("CTest.t.sol", syntax_err).unwrap();
    cmd.forge_fuse().args(["build", "--revive"]).assert_failure().stderr_eq(str![[r#"
Error: Compiler run failed:
Error (2314): Expected ';' but got identifier
 [FILE]:7:19:
  |
7 |         THIS WILL CAUSE AN ERROR
  |                   ^^^^^

"#]]);

    // ensure unchanged cache file
    let cache_after = fs::read_to_string(prj.cache()).unwrap();
    assert_eq!(cache, cache_after);
});

forgetest_init!(can_use_absolute_imports, |prj, cmd| {
    prj.update_config(|config| {
        let remapping = prj.paths().libraries[0].join("myDependency");
        config.remappings =
            vec![Remapping::from_str(&format!("myDependency/={}", remapping.display()))
                .unwrap()
                .into()];
    });

    prj.add_lib(
        "myDependency/src/interfaces/IConfig.sol",
        r"

    interface IConfig {}
   ",
    )
    .unwrap();

    prj.add_lib(
        "myDependency/src/Config.sol",
        r#"
        import "src/interfaces/IConfig.sol";

    contract Config {}
   "#,
    )
    .unwrap();

    prj.add_source(
        "Greeter",
        r#"
        import "myDependency/src/Config.sol";

    contract Greeter {}
   "#,
    )
    .unwrap();

    cmd.args(["build", "--revive"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);
});

// checks missing dependencies are auto installed
forgetest_init!(can_install_missing_deps_build, |prj, cmd| {
    prj.clear();

    // wipe forge-std
    let forge_std_dir = prj.root().join("lib/forge-std");
    pretty_err(&forge_std_dir, fs::remove_dir_all(&forge_std_dir));

    // Build the project
    cmd.args(["build", "--revive"]).assert_success().stdout_eq(str![[r#"
Missing dependencies found. Installing now...

[UPDATING_DEPENDENCIES]
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    // Expect compilation to be skipped as no files have changed
    cmd.forge_fuse().args(["build", "--revive"]).assert_success().stdout_eq(str![[r#"
No files changed, compilation skipped

"#]]);
});

// checks that extra output works
forgetest_init!(can_build_skip_contracts, |prj, cmd| {
    prj.clear();

    // Only builds the single template contract `src/*`
    cmd.args(["build", "--revive", "--skip", "tests", "--skip", "scripts"]).assert_success().stdout_eq(str![[
        r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#
    ]]);

    // Expect compilation to be skipped as no files have changed
    cmd.args(["build", "--revive"]).assert_success().stdout_eq(str![[r#"
No files changed, compilation skipped

"#]]);
});

forgetest_init!(can_build_skip_glob, |prj, cmd| {
    prj.add_test(
        "Foo",
        r"
contract TestDemo {
function test_run() external {}
}",
    )
    .unwrap();

    // only builds the single template contract `src/*` even if `*.t.sol` or `.s.sol` is absent
    prj.clear();
    cmd.args(["build", "--revive", "--skip", "*/test/**", "--skip", "*/script/**", "--force"])
        .assert_success()
        .stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    cmd.forge_fuse()
        .args(["build", "--revive", "--skip", "./test/**", "--skip", "./script/**", "--force"])
        .assert_success()
        .stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);
});

forgetest_init!(can_build_specific_paths, |prj, cmd| {
    prj.wipe();
    prj.add_source(
        "Counter.sol",
        r"
contract Counter {
function count() external {}
}",
    )
    .unwrap();
    prj.add_test(
        "Foo.sol",
        r"
contract Foo {
function test_foo() external {}
}",
    )
    .unwrap();
    prj.add_test(
        "Bar.sol",
        r"
contract Bar {
function test_bar() external {}
}",
    )
    .unwrap();

    // Build 2 files within test dir
    prj.clear();
    cmd.args(["build", "--revive", "test", "--force"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    // Build one file within src dir
    prj.clear();
    cmd.forge_fuse();
    cmd.args(["build", "--revive", "src", "--force"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    // Build 3 files from test and src dirs
    prj.clear();
    cmd.forge_fuse();
    cmd.args(["build", "--revive", "src", "test", "--force"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    // Build single test file
    prj.clear();
    cmd.forge_fuse();
    cmd.args(["build", "--revive", "test/Bar.sol", "--force"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    // Fail if no source file found.
    prj.clear();
    cmd.forge_fuse();
    cmd.args(["build", "--revive", "test/Dummy.sol", "--force"]).assert_failure().stderr_eq(str![[r#"
Error: No source files found in specified build paths.

"#]]);
});

forgetest_init!(can_build_sizes_repeatedly, |prj, cmd| {
    prj.clear_cache();

    cmd.args(["build", "--revive", "--sizes"]).assert_success().stdout_eq(str![[r#"
...
╭----------+------------------+-------------------+--------------------+---------------------╮
| Contract | Runtime Size (B) | Initcode Size (B) | Runtime Margin (B) | Initcode Margin (B) |
+============================================================================================+
| Counter  | 481              | 509               | 24,095             | 48,643              |
╰----------+------------------+-------------------+--------------------+---------------------╯


"#]]);

    cmd.forge_fuse().args(["build", "--revive", "--sizes", "--json"]).assert_success().stdout_eq(
        str![[r#"
{
  "Counter": {
    "runtime_size": 481,
    "init_size": 509,
    "runtime_margin": 24095,
    "init_margin": 48643
  }
}
"#]]
        .is_json(),
    );
});

// checks that build --names includes all contracts even if unchanged
forgetest_init!(can_build_names_repeatedly, |prj, cmd| {
    prj.clear_cache();

    cmd.args(["build", "--revive", "--names"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!
  compiler version: [..]
    - [..]
...

"#]]);

    cmd.forge_fuse()
        .args(["build", "--names", "--json"])
        .assert_success()
        .stdout_eq(str![[r#""{...}""#]].is_json());
});