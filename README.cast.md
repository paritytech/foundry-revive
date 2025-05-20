# Polkadot Foundry Supported Cast Commands Documentation with Examples

## Documentation Format and Color Scheme

This documentation is structured to provide a clear overview of the supported `cast` commands. Each command is presented in the following format:

- **Command Name**: The name of the command, colored to indicate its status (**<span style="color: green;">green</span>** for working, **<span style="color: red;">red</span>** for non-working).
- **Command**: The full command syntax with required parameters.
- **Required Parameters**: Parameters that must be provided for the command to execute, as specified in the help files.
- **Example**: A collapsible dropdown containing the complete command with its output or error message, ensuring all relevant details are included.

This format ensures clarity and ease of navigation, with the color scheme providing an immediate visual cue for command reliability.

## Rule of Thumb

- If the command is not listed, it is not supported.
- If the command is listed with a **<span style="color: red;">red</span>** color, it is not supported.
- If the command is listed with a **<span style="color: green;">green</span>** color, it is supported.

## [Cast Commands](https://github.com/paritytech/foundry-polkadot/issues/57)

### Cast Commands

#### <span style="color: green;">chain-id</span>
- **Command**: `cast chain-id [OPTIONS]`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast chain-id --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  ```
  </details>

#### <span style="color: green;">chain</span>
- **Command**: `cast chain [OPTIONS]`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast chain --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  westend-assethub
  ```
  </details>

#### <span style="color: green;">client</span>
- **Command**: `cast client [OPTIONS]`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast client --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  ```
  </details>

### Block Commands

#### <span style="color: green;">find-block</span>
- **Command**: `cast find-block [OPTIONS] <TIMESTAMP>`
- **Required Parameters**: `TIMESTAMP`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast find-block --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io 1744868532243
  ```
  </details>

#### <span style="color: green;">gas-price</span>
- **Command**: `cast gas-price [OPTIONS]`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast gas-price --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  ```
  </details>

#### <span style="color: green;">block-number</span>
- **Command**: `cast block-number [OPTIONS] [BLOCK]`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast block-number --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  ```
  </details>

#### <span style="color: green;">base-fee</span>
- **Command**: `cast base-fee [OPTIONS] [BLOCK]`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast base-fee latest --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  ```
  </details>

#### <span style="color: green;">block</span>
- **Command**: `cast block [OPTIONS] [BLOCK]`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast block latest -j --threads 1 --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  baseFeePerGas        1000
  difficulty           0
  extraData            0x
  gasLimit             786432000000000
  gasUsed              0
  hash                 0xd0a9eff00b163090b3db8ceb55dc964e215f131c5ef117f8a4b4844b11826da8
  logsBloom            0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
  miner                0x0000000000000000000000000000000000000000
  mixHash              0x0000000000000000000000000000000000000000000000000000000000000000
  nonce                0x0000000000000000
  number               11649117
  parentHash           0x39850941f800a862450392241e4b368ab78f0deefa6d06046bcd0f0c90b88a73
  parentBeaconRoot     
  transactionsRoot     0x5161b8d1123a52493fa98c406b1bcead73ba50f13969eaefb5e65d5618544d37
  receiptsRoot         0x5161b8d1123a52493fa98c406b1bcead73ba50f13969eaefb5e65d5618544d37
  sha3Uncles           0x0000000000000000000000000000000000000000000000000000000000000000
  size                 0
  stateRoot            0x4d626b879226c6f43b72958b844dc3fe18f3b3613c46c90d1153b033160399c7
  timestamp            1746706932 (Thu, 8 May 2025 12:22:12 +0000)
  withdrawalsRoot      
  totalDifficulty      
  blobGasUsed          
  excessBlobGas        
  requestsHash         
  transactions:        []
  ```
  </details>

#### <span style="color: green;">age</span>
- **Command**: `cast age [OPTIONS] [BLOCK]`
- **Required Parameters**: `BLOCK`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast age latest --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  ```
  </details>

#### <span style="color: red;">block (with threads error)</span>
- **Command**: `cast block [OPTIONS] [BLOCK]`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast block latest -j --threads 1 --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  error: a value is required for '--threads <THREADS>' but none was supplied
  ```
  </details>

### Account Commands

#### <span style="color: green;">balance</span>
- **Command**: `cast balance [OPTIONS] <WHO>`
- **Required Parameters**: `WHO`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast balance 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b -B latest -e --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  0.000000000000000000
  ```
  </details>

#### <span style="color: green;">storage</span>
- **Command**: `cast storage [OPTIONS] <ADDRESS> [SLOT]`
- **Required Parameters**: `ADDRESS`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast storage 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b 0 --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  0x000000000000000000000000000000000000000000000000000000000000000a
  ```
  </details>

#### <span style="color: green;">nonce</span>
- **Command**: `cast nonce [OPTIONS] <WHO>`
- **Required Parameters**: `WHO`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast nonce 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  0
  ```
  </details>

#### <span style="color: green;">code</span>
- **Command**: `cast code [OPTIONS] <WHO>`
- **Required Parameters**: `WHO`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast code 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  0x50564d0000b11f000000000000010700c15000c0004003405646ec651416c8d69a8
  ```
  </details>

#### <span style="color: red;">balance (block number)</span>
- **Command**: `cast balance [OPTIONS] <WHO>`
- **Required Parameters**: `WHO`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast balance -B 1 --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac
  [??]
  ```
  </details>

#### <span style="color: red;">storage (with Etherscan)</span>
- **Command**: `cast storage [OPTIONS] <ADDRESS> [SLOT]`
- **Required Parameters**: `ADDRESS`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast storage --etherscan-api-key 5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  [??]
  ```
  </details>

#### <span style="color: red;">storage (block tags)</span>
- **Command**: `cast storage [OPTIONS] <ADDRESS> [SLOT]`
- **Required Parameters**: `ADDRESS`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast storage 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b 0 --block latest --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  [??]
  ```
  </details>

### Transaction and Contract Interaction Commands

#### <span style="color: green;">sig-event</span>
- **Command**: `cast sig-event [OPTIONS] [EVENT_STRING]`
- **Required Parameters**: `EVENT_STRING`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast sig-event "CounterChanged(int)"
  0xb68ce3d4f35f8b562c4caf11012045e29a80cc1082438f785646ec651416c8d6
  ```
  </details>

#### <span style="color: green;">4byte-event</span>
- **Command**: `cast 4byte-event [OPTIONS] [TOPIC_0]`
- **Required Parameters**: `TOPIC_0`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast 4byte-event 0xb68ce3d4f35f8b562c4caf11012045e29a80cc1082438f785646ec651416c8d6
  CounterChanged(int256)
  ```
  </details>

#### <span style="color: green;">decode-event</span>
- **Command**: `cast decode-event [OPTIONS] <DATA>`
- **Required Parameters**: `DATA`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast decode-event --sig "CounterChanged(int256)" 0x000000000000000000000000000000000000000000000000000000000000002a
  42
  ```
  </details>

#### <span style="color: green;">decode-error</span>
- **Command**: `cast decode-error [OPTIONS] <DATA>`
- **Required Parameters**: `DATA`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast decode-error --sig "Panic(uint256)" 0x4e487b710000000000000000000000000000000000000000000000000000000000000011
  17
  ```
  </details>

#### <span style="color: green;">rpc</span>
- **Command**: `cast rpc [OPTIONS] <METHOD> [PARAMS]...`
- **Required Parameters**: `METHOD`, `RPC`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast rpc eth_getBlockByNumber --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io 0xB00000 false
  ```
  </details>

#### <span style="color: green;">abi-encode</span>
- **Command**: `cast abi-encode [OPTIONS] <SIG> [ARGS]...`
- **Required Parameters**: `SIG`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast abi-encode "getCount()"
  ```
  </details>

#### <span style="color: green;">calldata</span>
- **Command**: `cast calldata [OPTIONS] <SIG> [ARGS]...`
- **Required Parameters**: `SIG`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast calldata "rewardController()"
  0x8cc5ce99
  ```
  </details>

#### <span style="color: green;">decode-abi</span>
- **Command**: `cast decode-abi [OPTIONS] <SIG> <CALLDATA>`
- **Required Parameters**: `SIG`, `CALLDATA`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast decode-abi "balanceOf(address)(uint256)" 0x000000000000000000000000000000000000000000000000000000000000000a
  10
  ```
  </details>

#### <span style="color: green;">decode-calldata</span>
- **Command**: `cast decode-calldata [OPTIONS] <SIG> <CALLDATA>`
- **Required Parameters**: `SIG`, `CALLDATA`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast decode-calldata "transfer(address,uint256)" 0xa9059cbb000000000000000000000000e78388b4ce79068e89bf8aa7f218ef6b9ab0e9d0000000000000000000000000000000000000000000000000008a8e4b1a3d8000
  0xE78388b4CE79068e89Bf8aA7f218eF6b9AB0e9d0
  39000000000000000 [3.9e16]
  ```
  </details>

#### <span style="color: red;">estimate</span>
- **Command**: `cast estimate [OPTIONS] [TO] [SIG] [ARGS]... [COMMAND]`
- **Required Parameters**: `TO`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast estimate 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b --value 0.1ether 'getCount()' --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  [Fails due to eth_feeHistory not being supported]
  ```
  </details>

#### <span style="color: red;">logs</span>
- **Command**: `cast logs [OPTIONS] [SIG_OR_TOPIC] [TOPICS_OR_ARGS]...`
- **Required Parameters**: `SIG_OR_TOPIC`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast logs [EVENT_SIG] --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io --address [SOME]
  [Fails due to schema differences, e.g., --from-block earliest unsupported]
  ```
  </details>

#### <span style="color: red;">mktx</span>
- **Command**: `cast mktx [OPTIONS] [TO] [SIG] [ARGS]... [COMMAND]`
- **Required Parameters**: `TO`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast mktx [ADDRESS] [FUNCTION_NAME] [DATA] --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  [Fails due to eth_feeHistory not being supported]
  ```
  </details>

#### <span style="color: red;">proof</span>
- **Command**: `cast proof [OPTIONS] <ADDRESS> [SLOTS]...`
- **Required Parameters**: `ADDRESS`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast proof --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io [OPTION]
  [Fails due to eth_getProof not being supported]
  ```
  </details>

#### <span style="color: red;">storage-root</span>
- **Command**: `cast storage-root [OPTIONS] <WHO> [SLOTS]...`
- **Required Parameters**: `WHO`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast storage-root
  [Not supported by Westend Asset Hub node]
  ```
  </details>

#### <span style="color: red;">send</span>
- **Command**: `cast send [OPTIONS] [TO] [SIG] [ARGS]... [COMMAND]`
- **Required Parameters**: `TO`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast send [TO] [SIG] [ARGS] --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  [Likely fails due to transaction signing or gas estimation issues]
  ```
  </details>

### Miscellaneous Commands

#### <span style="color: red;">index</span>
- **Command**: `cast index [OPTIONS] <KEY_TYPE> <KEY> <SLOT_NUMBER>`
- **Required Parameters**: `KEY_TYPE`, `KEY`, `SLOT_NUMBER`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast index int "getCount" 1
  Error: Could not parse value

  Context:
  - parser error:
  getCount
  ^
  expected at least one digit
  ```
  </details>
