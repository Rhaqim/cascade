use web3::transports::WebSocket;
use web3::Transport;

use crate::info;
use crate::service::http_web3;
use crate::service::websocket_web3;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use web3::types::{BlockNumber, Log, U64};
use web3::{transports::Http, Web3};

use super::CliArgs;

async fn get_arb_logs(web3s: Web3<Http>) -> Vec<Log> {
    // let block_height: U64 = 101139981.into();
    let block_height: U64 = 22207815.into();
    let to_block = BlockNumber::Number(block_height.into());
    let from_block = BlockNumber::Number(block_height - 1000);

    let logs = web3s
        .eth()
        .logs(
            web3::types::FilterBuilder::default()
                .from_block(from_block)
                .to_block(to_block)
                .build(),
        )
        .await
        .expect("failed to fetch logs");

    info!("Logs length: {:?}", logs.len());

    logs
}

async fn test_get_block_by_hash(block_hash: web3::types::H256, web3s: Web3<Http>) {
    let transport = web3s.transport();

    // let block_hash_serde = serde_json::to_string(&block_hash).unwrap();

    let block_hash_serde = format!("0x{}", hex::encode(block_hash.as_bytes()));

    let get_block_by_hash = transport
        .execute(
            "eth_getBlockByHash",
            vec![block_hash_serde.into(), false.into()],
        )
        .await
        .unwrap();

    info!(
        "Block hash: {:?} \n Block: {:?} ",
        block_hash, get_block_by_hash
    );
}

async fn test_get_block_transaction_count_by_hash(
    block_hash: web3::types::H256,
    web3s: Web3<Http>,
) {
    let transport = web3s.transport();

    // let block_hash_serde = serde_json::to_string(&block_hash).unwrap();

    let block_hash_serde = format!("0x{}", hex::encode(block_hash.as_bytes()));

    let get_block_transaction_count_by_hash = transport
        .execute(
            // "eth_getBlockTransactionCountByHash",
            "arbtrace_block",
            vec![block_hash_serde.into()],
        )
        .await
        .unwrap();

    info!(
        "Block hash: {:?} \n Block txn count: {:?}",
        block_hash, get_block_transaction_count_by_hash
    );
}

pub async fn test_arbitriuem() {
    info!("Starting test_arbitriuem");

    let http = Http::new("").unwrap();
    let web3s = Web3::new(http);

    let start_time = Instant::now(); // Start the timer

    let logs = get_arb_logs(web3s.clone()).await;

    for log in logs {
        let block_hash = log.block_hash.unwrap();
        // test_get_block_by_hash(block_hash, web3s.clone()).await

        test_get_block_transaction_count_by_hash(block_hash, web3s.clone()).await

        // info!("Txn: {:?}, Block: {:?}", txn.hash, txn.block_number)
    }

    let elapsed_time = start_time.elapsed(); // Calculate elapsed time

    // Save elapsed time to file
    let mut file = File::create("arbt_test_time").expect("Failed to create file");
    let elapsed_time_secs = elapsed_time.as_secs();
    let elapsed_time_nanos = elapsed_time.subsec_nanos();
    let elapsed_time_str = format!(
        "{} seconds {} nanoseconds",
        elapsed_time_secs, elapsed_time_nanos
    );
    file.write_all(elapsed_time_str.as_bytes())
        .expect("Failed to write to file");
}

pub async fn run_concurrent(web3s: Web3<Http>) {
    use std::sync::{Arc, Mutex};
    use std::thread;

    let elements = get_arb_logs(web3s.clone()).await;

    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for element in elements {
        let counter = Arc::clone(&counter);

        // call the async function in a thread
        let handle = thread::spawn(move || {
            let http = Http::new("http://192.18.137.131").unwrap();
            let web3s = Web3::new(http);

            let block_hash = element.block_hash.unwrap();
            test_get_block_transaction_count_by_hash(block_hash, web3s.clone());

            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

pub mod cli_controller {

    use super::*;

    pub async fn run(args: CliArgs) {
        // check if node is websocket or http
        let node = "http://localhost:8545".to_string();
        let is_websocket = node.starts_with("wss://") || node.starts_with("ws://");

        // create web3 instance
        if is_websocket {
            let webs = websocket_web3(node).await;
            run_with_function::<Web3<Http>, Web3<WebSocket>>(args, None, Some(webs)).await;
        } else {
            let webs = http_web3(node);
            run_with_function::<Web3<Http>, Web3<WebSocket>>(args, Some(webs), None).await;
        }
    }

    pub async fn run_with_function<A, B>(
        args: CliArgs,
        a: Option<Web3<Http>>,
        b: Option<Web3<WebSocket>>,
    ) {
        info!("Starting run with args: {:?}", args);

        let address = args.address.clone();

        let web3h = a.clone();

        if address == "0x0" {
            run_with_defaults::<Web3<Http>>(web3h.unwrap(), args.clone()).await;
        }

        match b {
            Some(b) => {
                let logs = eth_query::<Web3<WebSocket>>(b, args).await;
                // info!("Logs length: {:?}", logs.len());
            }
            None => {
                let logs = eth_query::<Web3<Http>>(a.unwrap(), args).await;
                // info!("Logs length: {:?}", logs.len());
            }
        }
    }

    pub async fn run_with_defaults<A>(a: Web3<Http>, args: CliArgs) {
        let to_block = BlockNumber::Number(args.to.into());
        let from_block = BlockNumber::Number(args.from.into());

        let logs = a
            .eth()
            .logs(
                web3::types::FilterBuilder::default()
                    .from_block(from_block)
                    .to_block(to_block)
                    .build(),
            )
            .await
            .expect("failed to fetch logs");

        info!("Logs length: {:?}", logs.len());
    }

    pub async fn eth_query<T>(transport: T, args: CliArgs) {}
}
