use alloy::primitives::{Uint, Address, Bytes};
use foundry_evm::executors::RawCallResult;
use foundry_evm::{
    backend::Backend, executors::ExecutorBuilder, fork::CreateFork, opts::EvmOpts, traces::CallTraceNode,
};
use hex::decode;
use ansi_term::Colour::{Green, Red, Blue};
use ansi_term::Style;

// Values are hard-coded to simulate this transaction: https://etherscan.io/tx/0x72c97d7c4a5c4cb8797dc8017dd3ef0218bb9b770827dc2261f01c4018784680

#[tokio::main]
async fn main() {
    // ETH mainnet RPC
    let eth_http_url = String::from("https://eth.llamarpc.com"); // You can replace this with your own ETH mainnet RPC URL

    // Block number to fork at
    let block_number = Some(20307936);
    
    let evm_opts = EvmOpts {
        fork_url: Some(eth_http_url.clone()),
        fork_block_number: block_number,
        memory_limit: foundry_config::Config::default().memory_limit,
        ..Default::default()
    };
    // Await the future returned by `evm_env`
    let evm_env = evm_opts.evm_env().await.unwrap();

    let create_fork = CreateFork {
        url: eth_http_url,
        env: evm_env,
        evm_opts,
        enable_caching: true,
    };

    let executor_builder = ExecutorBuilder::default().gas_limit(1000000);
    let backend_db = Backend::spawn(Some(create_fork.clone()));
    let mut binding = executor_builder.build(create_fork.env.clone(), backend_db);
    let executor = binding.set_tracing(foundry_evm::traces::TraceMode::Call);

    // Transaction data
    let tx_data_hex = "28ed4f6c5936b3e602feca68a08a35cf6306445745b1d1a1ad00c5587e77f6d2ec15c9bc000000000000000000000000c50d163d143c244b3dafe84c9b58f1156eeeeeee";
    let tx_data_byte_array = decode(tx_data_hex).expect("Decoding failed");
    let tx_data_alloy_bytes: Option<Bytes> = Some(Bytes::from(tx_data_byte_array));

    let value: i64 = 0;
    let response = executor.call_raw(
        Address::parse_checksummed("0xB59Cdc85Cacd15097ecE4C77ed9D225014b4D56D", None).unwrap(), // from
        Address::parse_checksummed("0x57f1887a8BF19b14fC0dF6Fd9B2acc9Af147eA85", None).unwrap(), // to
        tx_data_alloy_bytes.unwrap_or_default(), // data
        Some(Uint::from(value)).unwrap_or_default(), // value
    );

    match response {
        Ok(result) => {
            print_result(result);
        }
        Err(err) => {
            println!("Error: {:?}", err);
        }
    }
}

fn print_result(result: RawCallResult) {
    if !result.reverted {
        println!("{} {}", Style::new().bold().fg(Blue).paint("Success:"), Style::new().bold().fg(Green).paint("true"));
    } else {
        println!("{} {}", Style::new().bold().fg(Blue).paint("Success:"), Style::new().bold().fg(Red).paint("false"));
    }
    println!("{} {:?}", Style::new().bold().fg(Blue).paint("Gas used:"), result.gas_used);
    println!("{} {:?}", Style::new().bold().fg(Blue).paint("Block number:"), result.env.block.number);
    println!("{} {:?}", Style::new().bold().fg(Blue).paint("Exit reason:"), result.exit_reason);
    println!("{} {:?}", Style::new().bold().fg(Blue).paint("Return data:"), result.result);
    println!("{} {:?}", Style::new().bold().fg(Blue).paint("Logs:"), result.logs);
    println!("");

    println!("==================== {} ====================", Style::new().bold().fg(Blue).paint("Traces"));
    print_traces(&result.traces.unwrap_or_default().into_nodes());
}

fn print_trace_node(trace: &CallTraceNode, traces: &[CallTraceNode], depth: usize) {
    let indent = "  ".repeat(depth*4);

    println!("{} CallTraceNode {{", "-".repeat(depth*4));
    println!("|{}|  idx: {},", indent, trace.idx);
    println!("|{}|  parent: {:?},", indent, trace.parent);
    println!("|{}|  trace: {:?}", indent, trace.trace);
    println!("|{}|  children: [", indent);
    for &child_idx in &trace.children {
        print_trace_node(&traces[child_idx], traces, depth + 1);
    }
    println!("|{}|  ],", indent);
    println!("|{}|  logs: [", indent);
    for log in &trace.logs {
        println!("|{}|    {:?},", indent, log);
    }
    println!("|{}|  ],", indent);
    println!("|{}|  ordering: {:?},", indent, trace.ordering);
    println!("|{}}}", indent);
}

fn print_traces(traces: &[CallTraceNode]) {
    for trace in traces.iter().filter(|t| t.parent.is_none()) {
        print_trace_node(trace, traces, 0);
    }
}
