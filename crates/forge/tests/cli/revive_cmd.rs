use std::str::FromStr;

use foundry_compilers::artifacts::Remapping;

use crate::constants::*;
// checks that `clean` works
forgetest_init!(can_clean_config, |prj, cmd| {
    // Resolc does not respect the `out` settings, example:
    // prj.update_config(|config| config.out = "custom-out".into());
    cmd.args(["build", "--resolc"]).assert_success();

    let artifact = prj.root().join(format!("resolc-out/{TEMPLATE_TEST_CONTRACT_ARTIFACT_JSON}"));
    assert!(artifact.exists());

    cmd.forge_fuse().arg("clean").assert_empty_stdout();
    assert!(!artifact.exists());
});

forgetest!(must_rebuild_when_used_the_same_out, |prj, cmd| {
    prj.add_raw_source(
        "Foo",
        r"
pragma solidity *;
contract Foo {}
   ",
    )
    .unwrap();

    // compile with solc
    cmd.args(["build", "--out=resolc-out"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    let artifact = prj.root().join("resolc-out/");
    assert!(artifact.exists());

    // compile with resolc to the same output dir (resolc has hardcoded output dir)
    cmd.forge_fuse().args(["build", "--resolc"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [RESOLC_VERSION]
[RESOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);

    // compile again with solc to the same output dir
    cmd.forge_fuse().args(["build", "--out=resolc-out"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [SOLC_VERSION]
[SOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);
});

// Tests that direct import paths are handled correctly
forgetest!(can_handle_direct_imports_into_src_for_resolc, |prj, cmd| {
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
    function check(Bar memory b) internal {}
    function check2(Foo f) internal {}
}
   "#,
    )
    .unwrap();

    cmd.args(["build", "--resolc"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [RESOLC_VERSION]
[RESOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);
});

forgetest!(can_use_absolute_imports_for_resolc, |prj, cmd| {
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

    cmd.args(["build", "--resolc"]).assert_success().stdout_eq(str![[r#"
[COMPILING_FILES] with [RESOLC_VERSION]
[RESOLC_VERSION] [ELAPSED]
Compiler run successful!

"#]]);
});
