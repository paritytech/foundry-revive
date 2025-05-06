use std::str::FromStr;

use foundry_compilers::artifacts::Remapping;

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
    function check(Bar memory b) public {}
    function check2(Foo f) public {}
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