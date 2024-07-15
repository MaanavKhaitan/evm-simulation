# EVM Simulation with Tracing

This program simulates [this transaction](https://etherscan.io/tx/0x72c97d7c4a5c4cb8797dc8017dd3ef0218bb9b770827dc2261f01c4018784680) on a fork of Ethereum mainnet.

## How to run the simulation

```
cargo run
```

## Expected output

```
Success: true
Gas used: 43084
Block number: 20307936
Exit reason: Stop
Return data: 0x
Logs: [Log { address: 0x00000000000c2e074ec69a0dfb2997ba6c7d2e1e, data: LogData { topics: [0xce0457fe73731f824cc272376169235128c118b49d344817417c6d108d155e82, 0x93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae, 0x5936b3e602feca68a08a35cf6306445745b1d1a1ad00c5587e77f6d2ec15c9bc], data: 0x000000000000000000000000c50d163d143c244b3dafe84c9b58f1156eeeeeee } }]

==================== Traces ====================
 CallTraceNode {
||  idx: 0,
||  parent: None,
||  trace: CallTrace { depth: 0, success: true, caller: 0xb59cdc85cacd15097ece4c77ed9d225014b4d56d, address: 0x57f1887a8bf19b14fc0df6fd9b2acc9af147ea85, maybe_precompile: None, selfdestruct_refund_target: None, selfdestruct_transferred_value: None, kind: Call, value: 0, data: 0x28ed4f6c5936b3e602feca68a08a35cf6306445745b1d1a1ad00c5587e77f6d2ec15c9bc000000000000000000000000c50d163d143c244b3dafe84c9b58f1156eeeeeee, output: 0x, gas_used: 21152, gas_limit: 1000000, status: Stop, steps: [], decoded: DecodedCallTrace { label: None, return_data: None, call_data: None } }
||  children: [
---- CallTraceNode {
|        |  idx: 1,
|        |  parent: Some(0),
|        |  trace: CallTrace { depth: 1, success: true, caller: 0x57f1887a8bf19b14fc0df6fd9b2acc9af147ea85, address: 0x00000000000c2e074ec69a0dfb2997ba6c7d2e1e, maybe_precompile: None, selfdestruct_refund_target: None, selfdestruct_transferred_value: None, kind: StaticCall, value: 0, data: 0x02571be393cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae, output: 0x00000000000000000000000057f1887a8bf19b14fc0df6fd9b2acc9af147ea85, gas_used: 2951, gas_limit: 955439, status: Return, steps: [], decoded: DecodedCallTrace { label: None, return_data: None, call_data: None } }
|        |  children: [
|        |  ],
|        |  logs: [
|        |  ],
|        |  ordering: [],
|        }
---- CallTraceNode {
|        |  idx: 2,
|        |  parent: Some(0),
|        |  trace: CallTrace { depth: 1, success: true, caller: 0x57f1887a8bf19b14fc0df6fd9b2acc9af147ea85, address: 0x00000000000c2e074ec69a0dfb2997ba6c7d2e1e, maybe_precompile: None, selfdestruct_refund_target: None, selfdestruct_transferred_value: None, kind: Call, value: 0, data: 0x06ab592393cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae5936b3e602feca68a08a35cf6306445745b1d1a1ad00c5587e77f6d2ec15c9bc000000000000000000000000c50d163d143c244b3dafe84c9b58f1156eeeeeee, output: 0xa293caedc51d186ddb83a6270742c4a7caf9c3dd88f4add66b3a9a56c4dd835d, gas_used: 5169, gas_limit: 947176, status: Return, steps: [], decoded: DecodedCallTrace { label: None, return_data: None, call_data: None } }
|        |  children: [
|        |  ],
|        |  logs: [
|        |    CallLog { raw_log: LogData { topics: [0xce0457fe73731f824cc272376169235128c118b49d344817417c6d108d155e82, 0x93cdeb708b7545dc668eb9280176169d1c33cfd8ed6f04690a0bcc88a93fc4ae, 0x5936b3e602feca68a08a35cf6306445745b1d1a1ad00c5587e77f6d2ec15c9bc], data: 0x000000000000000000000000c50d163d143c244b3dafe84c9b58f1156eeeeeee }, decoded: DecodedCallLog { name: None, params: None } },
|        |  ],
|        |  ordering: [Log(0)],
|        }
||  ],
||  logs: [
||  ],
||  ordering: [Call(0), Call(1)],
|}
```