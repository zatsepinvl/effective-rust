use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use chrono::prelude::*;

#[derive(Clone)]
struct Block {
    index: u64,
    timestamp: i64,
    hash: u64,
    prev_block_hash: u64,
    payload: String,
}

struct BlockChainError {
    message: &'static str
}

struct BlockChain {
    chain: RefCell<Vec<Block>>,
}

impl BlockChain {
    fn new(genesis_block: Block) -> BlockChain {
        BlockChain {
            chain: RefCell::new(vec![genesis_block])
        }
    }

    fn add_block(self: &BlockChain, new_block: Block) -> Result<(), BlockChainError> {
        let last_block = self.get_last_block();
        if new_block.prev_block_hash == last_block.hash {
            self.chain.borrow_mut().push(new_block);
            Ok(())
        } else {
            Err(BlockChainError { message: "Block should refer to the latest block in the chain" })
        }
    }

    fn get_last_block(self: &BlockChain) -> &Block {
        let chain = self.chain.borrow();
        chain.last().unwrap()
    }
}

fn generate_genesis_block(payload: String) -> Block {
    let index: u64 = 0;
    let prev_block_hash: u64 = 0;
    generate_block(index, prev_block_hash, payload)
}

fn generate_next_block(prev_block: &Block, payload: String) -> Block {
    generate_block(prev_block.index + 1, prev_block.hash, payload)
}

fn generate_block(index: u64, prev_block_hash: u64, payload: String) -> Block {
    let utc: DateTime<Utc> = Utc::now();
    let timestamp = utc.timestamp();
    let mut hasher = DefaultHasher::new();
    index.hash(&mut hasher);
    timestamp.hash(&mut hasher);
    payload.hash(&mut hasher);
    prev_block_hash.hash(&mut hasher);
    let hash = hasher.finish();
    return Block {
        index,
        timestamp,
        hash,
        payload,
        prev_block_hash: 0,
    };
}

fn main() {
    let genesis_block = generate_genesis_block(String::from("block_0_payload"));
    let blockchain = BlockChain::new(genesis_block);

    let last_block = blockchain.get_last_block();
    let new_block = generate_next_block(last_block, String::from("block_1_payload"));
    blockchain.add_block(new_block);
}

