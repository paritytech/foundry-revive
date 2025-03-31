//! Contains various tests for checking the `forge create` subcommand

use crate::{
    constants::*,
    utils::{self, network_private_key, network_rpc_key},
};
use alloy_primitives::Address;
use foundry_compilers::artifacts::{remappings::Remapping, BytecodeHash};
use foundry_test_utils::{
    forgetest, forgetest_async, str,
    util::{OutputExt, TestProject},
};
use std::str::FromStr;

/// This will insert _dummy_ contract that uses a library
///
/// **NOTE** This is intended to be linked against a random address and won't actually work. The
/// purpose of this is _only_ to make sure we can deploy contracts linked against addresses.
///
/// This will create a library `remapping/MyLib.sol:MyLib`
///
/// returns the contract argument for the create command
fn setup_with_simple_remapping(prj: &TestProject) -> String {
    // explicitly set remapping and libraries
    prj.update_config(|config| {
        config.remappings = vec![Remapping::from_str("remapping/=lib/remapping/").unwrap().into()];
        config.libraries = vec![format!("remapping/MyLib.sol:MyLib:{:?}", Address::random())];
    });

    prj.add_source(
        "LinkTest",
        r#"
import "remapping/MyLib.sol";
contract LinkTest {
    function foo() public returns (uint256) {
        return MyLib.foobar(1);
    }
}
"#,
    )
    .unwrap();

    prj.add_lib(
        "remapping/MyLib",
        r"
library MyLib {
    function foobar(uint256 a) public view returns (uint256) {
    	return a * 100;
    }
}
",
    )
    .unwrap();

    "src/LinkTest.sol:LinkTest".to_string()
}

fn setup_oracle(prj: &TestProject) -> String {
    prj.update_config(|c| {
        c.libraries = vec![format!(
            "./src/libraries/ChainlinkTWAP.sol:ChainlinkTWAP:{:?}",
            Address::random()
        )];
    });

    prj.add_source(
        "Contract",
        r#"
import {ChainlinkTWAP} from "./libraries/ChainlinkTWAP.sol";
contract Contract {
    function getPrice() public view returns (int latest) {
        latest = ChainlinkTWAP.getLatestPrice(0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE);
    }
}
"#,
    )
    .unwrap();

    prj.add_source(
        "libraries/ChainlinkTWAP",
        r"
library ChainlinkTWAP {
   function getLatestPrice(address base) public view returns (int256) {
        return 0;
   }
}
",
    )
    .unwrap();

    "src/Contract.sol:Contract".to_string()
}

fn westend_assethub_args() -> Vec<String> {
    [
        "--rpc-url".to_string(),
        network_rpc_key("westend_assethub")
            .unwrap_or("https://westend-asset-hub-eth-rpc.polkadot.io".to_string()),
        "--private-key".to_string(),
        network_private_key("westend_assethub").unwrap_or(
            "5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133".to_string(),
        ),
    ]
    .to_vec()
}

// tests `forge` create on goerli if correct env vars are set
forgetest!(can_create_simple_on_westend_assethub, |prj, cmd| {
    let contract_path = setup_with_simple_remapping(&prj);
    let output = cmd
        .arg("create")
        .arg("--revive")
        .arg("--legacy")
        .arg("--broadcast")
        .args(westend_assethub_args())
        .arg(contract_path)
        .assert_success()
        .get_output()
        .stdout_lossy();
    let _address = utils::parse_deployed_address(output.as_str())
        .unwrap_or_else(|| panic!("Failed to parse deployer {output}"));
});

// tests `forge` create on goerli if correct env vars are set
forgetest!(can_create_oracle_on_westend_assethub, |prj, cmd| {
    let contract_path = setup_oracle(&prj);
    let output = cmd
        .arg("create")
        .arg("--revive")
        .arg("--legacy")
        .arg("--broadcast")
        .args(westend_assethub_args())
        .arg(contract_path)
        .assert_success()
        .get_output()
        .stdout_lossy();
    let _address = utils::parse_deployed_address(output.as_str())
        .unwrap_or_else(|| panic!("Failed to parse deployer {output}"));
});

// tests that we can deploy the template contract
// forgetest_async!(can_create_using_unlocked_on_westend_assethub, |prj, cmd| {
//     foundry_test_utils::util::initialize(prj.root());

//     // explicitly byte code hash for consistent checks
//     prj.update_config(|c| c.bytecode_hash = BytecodeHash::None);

//     cmd.forge_fuse()
//         .arg("create")
//         .arg("--revive")
//         .arg("--legacy")
//         .arg("--broadcast")
//         .arg("--unlocked")
//         .arg("--from=f24FF3a9CF04c71Dbc94D0b566f7A27B94566cac")
//         .arg(format!("./src/{TEMPLATE_CONTRACT}.sol:{TEMPLATE_CONTRACT}").as_str())
//         .args(westend_assethub_args());

//     cmd.assert_success().stdout_eq(str![[r#"
// [COMPILING_FILES] with [REVIVE_VERSION]
// [REVIVE_VERSION] [ELAPSED]
// Compiler run successful!
// Deployer: [..]
// Deployed to: [..]
// [TX_HASH]

// "#]]);

//     cmd.assert_success().stdout_eq(str![[r#"
// No files changed, compilation skipped
// Deployer: [..]
// Deployed to: [..]
// [TX_HASH]

// "#]]);
// });

// tests that we can deploy with constructor args
forgetest_async!(can_create_with_constructor_args_on_westend_assethub, |prj, cmd| {
    foundry_test_utils::util::initialize(prj.root());

    // explicitly byte code hash for consistent checks
    prj.update_config(|c| c.bytecode_hash = BytecodeHash::None);

    prj.add_source(
        "ConstructorContract",
        r#"
contract ConstructorContract {
    string public name;

    constructor(string memory _name) {
        name = _name;
    }
}
"#,
    )
    .unwrap();

    cmd.forge_fuse()
        .arg("create")
        .arg("--revive")
        .arg("--legacy")
        .arg("--broadcast")
        .arg("./src/ConstructorContract.sol:ConstructorContract")
        .args(westend_assethub_args())
        .args(["--constructor-args", "My Constructor"])
        .assert_success()
        .stdout_eq(str![[r#"
[COMPILING_FILES] with [REVIVE_VERSION]
[REVIVE_VERSION] [ELAPSED]
Compiler run successful!
Deployer: [..]
Deployed to: [..]
[TX_HASH]

"#]]);

    prj.add_source(
        "TupleArrayConstructorContract",
        r#"
struct Point {
    uint256 x;
    uint256 y;
}

contract TupleArrayConstructorContract {
    constructor(Point[] memory _points) {}
}
"#,
    )
    .unwrap();

    cmd.forge_fuse()
        .arg("create")
        .arg("--revive")
        .arg("--legacy")
        .arg("--broadcast")
        .arg("./src/TupleArrayConstructorContract.sol:TupleArrayConstructorContract")
        .args(westend_assethub_args())
        .args(["--constructor-args", "[(1,2), (2,3), (3,4)]"])
        .assert()
        .stdout_eq(str![[r#"
[COMPILING_FILES] with [REVIVE_VERSION]
[REVIVE_VERSION] [ELAPSED]
Compiler run successful!
Deployer: [..]
Deployed to: [..]
[TX_HASH]

"#]]);
});

// <https://github.com/foundry-rs/foundry/issues/6332>
forgetest_async!(can_create_and_call_on_westend_assethub, |prj, cmd| {
    foundry_test_utils::util::initialize(prj.root());

    // explicitly byte code hash for consistent checks
    prj.update_config(|c| c.bytecode_hash = BytecodeHash::None);

    prj.add_source(
        "UniswapV2Swap",
        r#"
contract UniswapV2Swap {

    function pairInfo() public view returns (uint reserveA, uint reserveB, uint totalSupply) {
       (reserveA, reserveB, totalSupply) = (0,0,0);
    }

}
"#,
    )
    .unwrap();
    cmd.forge_fuse()
        .arg("create")
        .arg("--revive")
        .arg("--legacy")
        .arg("--broadcast")
        .arg("./src/UniswapV2Swap.sol:UniswapV2Swap")
        .args(westend_assethub_args())
        .assert_success()
        .stdout_eq(str![[r#"
[COMPILING_FILES] with [REVIVE_VERSION]
[REVIVE_VERSION] [ELAPSED]
Compiler run successful with warnings:
Warning (2018): Function state mutability can be restricted to pure
 [FILE]:6:5:
  |
6 |     function pairInfo() public view returns (uint reserveA, uint reserveB, uint totalSupply) {
  |     ^ (Relevant source part starts here and spans across multiple lines).

Deployer: [..]
Deployed to: [..]
[TX_HASH]

"#]]);
});
