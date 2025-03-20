use crate::utils::generate_large_init_contract;
use foundry_test_utils::{forgetest, snapbox::IntoData, str};
use foundry_compilers::artifacts::BytecodeHash;
use foundry_config::Config;

forgetest_init!(can_build_with_revive, |prj, cmd| {
    // BytecodeHash issue workaround https://github.com/paritytech/revive/issues/219
    prj.write_config(Config { bytecode_hash: BytecodeHash::None, ..Default::default() });
    cmd.args(["build", "--revive-compile"]).assert_success();
});

forgetest_init!(force_buid_with_revive, |prj, cmd| {
    // BytecodeHash issue workaround https://github.com/paritytech/revive/issues/219
    prj.write_config(Config { bytecode_hash: BytecodeHash::None, ..Default::default() });
    cmd.args(["build", "--revive-compile", "--force"]).assert_success();
});

forgetest!(code_size_exceeds_limit_with_revive, |prj, cmd| {
    prj.write_config(Config { bytecode_hash: BytecodeHash::None, ..Default::default() });
    prj.add_source("LargeContract.sol", generate_large_init_contract(25_000).as_str()).unwrap();
    cmd.args(["build", "--revive-compile", "--sizes"]).assert_failure().stdout_eq(str![[r#"
[COMPILING_FILES] with resolc and [SOLC_VERSION]
resolc and [SOLC_VERSION] [ELAPSED]
Compiler run successful!

╭---------------+------------------+-------------------+--------------------+---------------------╮
| Contract      | Runtime Size (B) | Initcode Size (B) | Runtime Margin (B) | Initcode Margin (B) |
+=================================================================================================+
| LargeContract | 268,040          | 268,040           | -18,040            | -18,040             |
╰---------------+------------------+-------------------+--------------------+---------------------╯


"#]]);

    cmd.forge_fuse().args(["build", "--revive-compile", "--sizes", "--json"]).assert_failure().stdout_eq(
        str![[r#"
{
  "LargeContract": {
    "runtime_size": 268040,
    "init_size": 268040,
    "runtime_margin": -18040,
    "init_margin": -18040
  }
}
"#]]
        .is_json(),
    );
});
