use jsonrpc_core::{IoHandler, Params};
use jsonrpc_http_server::ServerBuilder;
use serde_json::{json};
use blockchain::BlockChain;
use blockchain::Transaction;
use std::sync::{Mutex};

mod blockchain;

fn main() {
    let genesis_block = BlockChain::generate_genesis_block();
    let blockchain = BlockChain::new(genesis_block);

    let mutex = Mutex::new(blockchain);
    let mut io = IoHandler::default();

    io.add_sync_method("submitTransaction", move |_params: Params| {
        let mut blockchain = mutex.lock().unwrap();
        blockchain.send_transaction(Transaction { payload: "payload".to_string() });
        let ref block = blockchain.produce_block();
        println!("New produced block: {}", block);
        Ok(json!({"block_index": block.index}))
    });

    let server = ServerBuilder::new(io)
        .threads(3)
        .start_http(&"127.0.0.1:3030".parse().unwrap())
        .unwrap();

    server.wait();
}