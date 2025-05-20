# Polkadot Foundry Supported Forge Commands Documentation with Examples

## Documentation Format and Color Scheme

This documentation is structured to provide a clear overview of the supported `forge` commands. Each command is presented in the following format:

- **Command Name**: The name of the command, colored to indicate its status (**<span style="color: green;">green</span>** for working, **<span style="color: red;">red</span>** for non-working).
- **Command**: The full command syntax with required parameters.
- **Required Parameters**: Parameters that must be provided for the command to execute, as specified in the help files.
- **Example**: A collapsible dropdown containing the complete command with its output or error message, ensuring all relevant details are included.

This format ensures clarity and ease of navigation, with the color scheme providing an immediate visual cue for command reliability.

## Rule of Thumb

- If the command is not listed, it is not supported.
- If the command is listed with a **<span style="color: red;">red</span>** color, it is not supported.
- If the command is listed with a **<span style="color: green;">green</span>** color, it is supported.

## [Forge Commands](https://github.com/paritytech/foundry-polkadot/issues/54)

### Project Setup and Installation

#### <span style="color: green;">init</span>
- **Command**: `forge init`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > forge init
  Initializing /home/ec2-user/test-foundry/example...
  Installing forge-std in /home/ec2-user/test-foundry/example/lib/forge-std (url: Some("https://github.com/foundry-rs/forge-std"), tag: None)
  Cloning into '/home/ec2-user/test-foundry/example/lib/forge-std'...
  remote: Enumerating objects: 2111, done.
  remote: Counting objects: 100% (1042/1042), done.
  remote: Compressing objects: 100% (150/150), done.
  remote: Total 2111 (delta 955), reused 904 (delta 892), pack-reused 1069 (from 1)
  Receiving objects: 100% (2111/2111), 680.96 KiB | 17.92 MiB/s, done.
  Resolving deltas: 100% (1431/1431), done.
      Installed forge-std v1.9.7
      Initialized forge project
  ```
  </details>

### Compilation and Testing

#### <span style="color: green;">build</span>
- **Command**: `forge build`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > forge build
  [⠊] Compiling...
  No files changed, compilation skipped
  ```
  </details>

#### <span style="color: green;">test</span>
- **Command**: `forge test`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > forge test
  [⠊] Compiling...
  No files changed, compilation skipped

  Ran 2 tests for test/Counter.t.sol:CounterTest
  [PASS] testFuzz_SetNumber(uint256) (runs: 256, μ: 32198, ~: 32354)
  [PASS] test_Increment() (gas: 31851)
  Suite result: ok. 2 passed; 0 failed; 0 skipped; finished in 8.31ms (8.00ms CPU time)

  Ran 1 test suite in 13.37ms (8.31ms CPU time): 2 tests passed, 0 failed, 0 skipped (2 total tests)
  ```
  </details>

#### <span style="color: green;">snapshot</span>
- **Command**: `forge snapshot`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > forge snapshot
  [⠊] Compiling...
  No files changed, compilation skipped

  Ran 2 tests for test/Counter.t.sol:CounterTest
  [PASS] testFuzz_SetNumber(uint256) (runs: 256, μ: 32043, ~: 32354)
  [PASS] test_Increment() (gas: 31851)
  Suite result: ok. 2 passed; 0 failed; 0 skipped; finished in 8.42ms (8.15ms CPU time)

  Ran 1 test suite in 13.88ms (8.42ms CPU time): 2 tests passed, 0 failed, 0 skipped (2 total tests)
  ```
  </details>

#### <span style="color: green;">bind</span>
- **Command**: `forge bind`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > forge bind
  [⠒] Compiling...
  Compiler run successful!
  Generating bindings for 2 contracts
  Bindings have been generated to /home/ec2-user/test-foundry/out/bindings
  ```
  </details>

#### <span style="color: red;">cache test</span>
- **Command**: `forge cache [OPTIONS] <COMMAND>`
- **Required Parameters**: `COMMAND`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > forge cache test
  error: unrecognized subcommand 'test'

  Usage: forge cache [OPTIONS] <COMMAND>

  For more information, try '--help'.
  ```
  </details>

### Contract Deployment

#### <span style="color: red;">create</span>
- **Command**: `forge create [OPTIONS] <CONTRACT>`
- **Required Parameters**: `CONTRACT`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > forge create src/Counter.sol:Counter --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io --private-key 5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133
  [⠊] Compiling...
  No files changed, compilation skipped
  Error: server returned an error response: error code -32000: Failed to instantiate contract: Module(ModuleError { index: 60, error: [27, 0, 0, 0], message: Some("CodeRejected") })
  ```
  </details>

### Code Manipulation and Documentation

#### <span style="color: green;">flatten</span>
- **Command**: `forge flatten [OPTIONS] <PATH>`
- **Required Parameters**: `PATH`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > forge flatten src/Counter.sol
  // SPDX-License-Identifier: UNLICENSED
  pragma solidity ^0.8.13;

  // src/Counter.sol

  contract Counter {
      int private count;

      constructor(int _count) {
          count = _count;
      }

      function getCount() public view returns (int) {
          return count;
      }

      function incrementCounter() public {
          count +=1;
      }

      function decrementCounter() public {
          count -=1;
      }
  }
  ```
  </details>

#### <span style="color: green;">doc</span>
- **Command**: `forge doc`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > forge doc
  ```
  </details>

#### <span style="color: green;">cache clean</span>
- **Command**: `forge cache clean`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > forge cache clean
  ```
  </details>

#### <span style="color: green;">cache ls</span>
- **Command**: `forge cache ls`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > forge cache ls
  ```
  </details>

#### <span style="color: red;">doc (with build)</span>
- **Command**: `forge doc`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > forge doc --build --out ./documentation
  Error: prefix not found
  ```
  </details>

#### <span style="color: red;">selectors upload</span>
- **Command**: `forge selectors upload`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > forge upload-selectors Counter
  error: unrecognized subcommand 'upload-selectors'

    tip: some similar subcommands exist: 'se', 'selectors'

  Usage: forge [OPTIONS] <COMMAND>

  For more information, try '--help'.
  ```
  </details>
