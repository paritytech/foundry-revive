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

## Known Issues

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
  > cast block latest --threads 1 --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  baseFeePerGas        1000
  difficulty           0
  extraData            0x
  gasLimit             786432000000000
  gasUsed              0
  hash                 0xaa46566b611466b75f4162588ecc72ac994975e62201b3b07735c718d288133b
  logsBloom            0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
  miner                0x0000000000000000000000000000000000000000
  mixHash              0x0000000000000000000000000000000000000000000000000000000000000000
  nonce                0x0000000000000000
  number               11785635
  parentHash           0xdd386c3903443ab8236919c18e2d5d5ff7be09e308292ce1ec900f299f53be68
  parentBeaconRoot     
  transactionsRoot     0x815769ac3ab76f2390f3a5e69aea2ff52523d70790fa54ccad28294749e2c5d8
  receiptsRoot         0x815769ac3ab76f2390f3a5e69aea2ff52523d70790fa54ccad28294749e2c5d8
  sha3Uncles           0x0000000000000000000000000000000000000000000000000000000000000000
  size                 0
  stateRoot            0xa835183182a2ca67244e5ba1e13858b95b1e848886a0b6310c4475075c475afa
  timestamp            1747912272 (Thu, 22 May 2025 11:11:12 +0000)
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

### Account Commands

#### <span style="color: green;">balance</span>
- **Command**: `cast balance [OPTIONS] <WHO>`
- **Required Parameters**: `WHO`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast balance 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b -B latest -e --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  10.000000000000000000
  ```
  </details>

#### <span style="color: green;">storage</span>
- **Command**: `cast storage [OPTIONS] <ADDRESS> [SLOT]`
- **Additional Flags**:
  - `--resolc`: Use the Resolc compiler.
- **Required Parameters**: `ADDRESS`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast storage 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b 0 --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io --resolc
  0x0000000000000000000000000000000000000000000000000000000000000011
  ```
  </details>

#### <span style="color: green;">nonce</span>
- **Command**: `cast nonce [OPTIONS] <WHO>`
- **Required Parameters**: `WHO`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast nonce 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io
  1427
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

#### <span style="color: green;">estimate</span>
- **Command**: `cast estimate [OPTIONS] [TO] [SIG] [ARGS]... [COMMAND]`
- **Required Parameters**: `TO`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast estimate --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io --from 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b "incrementCounter()"
  9693273291600
  ```
  </details>

#### <span style="color: green;">logs</span>
- **Command**: `cast logs [OPTIONS] [SIG_OR_TOPIC] [TOPICS_OR_ARGS]...`
- **Required Parameters**: `SIG_OR_TOPIC`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast logs --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io --address 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b --from-block 11462303 --to-block latest 0xb68ce3d4f35f8b562c4caf11012045e29a80cc1082438f785646ec651416c8d6
  ```
  </details>

#### <span style="color: green;">mktx</span>
- **Command**: `cast mktx [OPTIONS] [TO] [SIG] [ARGS]... [COMMAND]`
- **Required Parameters**: `TO`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast mktx 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b "incrementCounter()" --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io --from 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac --chain-id 420420421 --private-key 5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133 --format-json 0x02f87284190f1b45820584018207d18608cc3c04b35094c88d454a33610f4c73acc367ccaaf98e7ee78a1b80845b34b966c001a09a64c754b676a1c010d80ec82790c632f16f8fed5e7af8bd0aadaeccf6b2ea10a0232e21cf2b7aa007e4af2140e0aaaca8e33fd4a9d8009fe9be338e58ed3f04f0
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

#### <span style="color: green;">send</span>
- **Command**: `cast send [OPTIONS] [TO] [SIG] [ARGS]... [COMMAND]`
- **Required Parameters**: `TO`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast send 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b "incrementCounter()" --rpc-url https://westend-asset-hub-eth-rpc.polkadot.io --from 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac --chain-id 420420421 --private-key 5fb92d6e98884f76de468fa3f6278f8807c48bebc13595d45af5bdc4da702133
  blockHash            0xf090749fae363e5886fc70088110349a6f806479958e24ba54b79481753ec0a0
  blockNumber          11786028
  contractAddress      
  cumulativeGasUsed    0
  effectiveGasPrice    1000001000
  from                 0xf24FF3a9CF04c71Dbc94D0b566f7A27B94566cac
  gasUsed              27543090
  logs                 [{"address":"0xc88d454a33610f4c73acc367ccaaf98e7ee78a1b","topics":["0xb68ce3d4f35f8b562c4caf11012045e29a80cc1082438f785646ec651416c8d6"],"data":"0x0000000000000000000000000000000000000000000000000000000000000011","blockHash":"0xf090749fae363e5886fc70088110349a6f806479958e24ba54b79481753ec0a0","blockNumber":"0xb3d72c","transactionHash":"0xdf5f9fc91c12f0127b6bbf17b81903ece1f3ccf498a9192b5023ca8e89ebb369","transactionIndex":"0x2","logIndex":"0x4","removed":false}]
  logsBloom            0x00020000004000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000020000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000040000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
  root                 
  status               1 (success)
  transactionHash      0xdf5f9fc91c12f0127b6bbf17b81903ece1f3ccf498a9192b5023ca8e89ebb369
  transactionIndex     2
  type                 2
  blobGasPrice         
  blobGasUsed          
  to                   0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b
  ```
  </details>

### Miscellaneous Commands

#### <span style="color: green;">index</span>
- **Command**: `cast index [OPTIONS] <KEY_TYPE> <KEY> <SLOT_NUMBER>`
- **Required Parameters**: `KEY_TYPE`, `KEY`, `SLOT_NUMBER`
- **Example**:
  <details>
  <summary>Click to toggle contents of example</summary>

  ```bash
  > cast index address 0xC88d454A33610f4C73acc367cCAAf98E7Ee78a1b 1
  0xdc8ee9c32ef1c741cf41b54d4aff707dde6381fb94bfb7247e9084679a57d0db
  ```
  </details>
