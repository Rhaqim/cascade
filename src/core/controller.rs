use web3::Transport;

use crate::log::logger;
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

    logger("info", &format!("Logs length: {:?}", logs.len()));

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

    logger(
        "info",
        &format!(
            "Block hash: {:?} \n Block: {:?} ",
            block_hash, get_block_by_hash
        ),
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

    logger(
        "info",
        &format!(
            "Block hash: {:?} \n Block txn count: {:?}",
            block_hash, get_block_transaction_count_by_hash
        ),
    );
}

pub async fn test_arbitriuem() {
    logger("info", &format!("Starting test_arbitriuem"));

    let http = Http::new("http://192.18.137.131").unwrap();
    let web3s = Web3::new(http);

    let start_time = Instant::now(); // Start the timer

    let logs = get_arb_logs(web3s.clone()).await;

    for log in logs {
        let block_hash = log.block_hash.unwrap();
        // test_get_block_by_hash(block_hash, web3s.clone()).await

        test_get_block_transaction_count_by_hash(block_hash, web3s.clone()).await

        //logger(
        //    "info",
        //    &format!("Txn: {:?}, Block: {:?}", txn.hash, txn.block_number),
        //)
    }

    let elapsed_time = start_time.elapsed(); // Calculate elapsed time
    logger("info", &format!("Elapsed time: {:?}", elapsed_time));

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

pub async fn run(args: CliArgs) {}

pub async fn run_with_function() {}

pub async fn run_with_defaults() {}
