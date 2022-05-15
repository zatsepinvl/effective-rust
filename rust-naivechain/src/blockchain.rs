use std::collections::hash_map::DefaultHasher;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};

use chrono::prelude::*;

#[derive(Debug, Clone, Hash)]
pub struct Transaction {
    pub payload: String,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub hash: u64,
    pub prev_block_hash: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug)]
pub struct BlockChainError {
    pub message: String,
}

#[derive(Debug)]
pub struct BlockChain {
    pub chain: Vec<Block>,
    pub pending_transactions: Vec<Transaction>,
}

impl BlockChain {
    pub fn generate_genesis_block() -> Block {
        let index = 0;
        let timestamp: i64 = Utc::now().timestamp();
        let ref mut hasher = DefaultHasher::new();
        timestamp.hash(hasher);
        index.hash(hasher);
        let hash = hasher.finish();
        return Block {
            index: 0,
            timestamp,
            hash,
            transactions: vec![],
            prev_block_hash: 0,
        };
    }

    pub fn new(genesis_block: Block) -> BlockChain {
        BlockChain {
            chain: vec![genesis_block],
            pending_transactions: vec![],
        }
    }

    pub fn latest_block(&self) -> &Block {
        let block_optional = self.chain.last();
        block_optional.unwrap()
    }

    pub fn send_transaction(&mut self, transaction: Transaction) {
        self.pending_transactions.push(transaction)
    }

    // Produce new block with all pending transactions in it
    pub fn produce_block(&mut self) -> Block {
        let prev_block = self.latest_block();
        let prev_block_index = prev_block.index + 1;
        let prev_block_hash = prev_block.hash;
        let transactions = &mut vec![];
        for transaction in self.pending_transactions.drain(..) {
            transactions.push(transaction)
        }
        let next_block = self.generate_block(prev_block_index, prev_block_hash, transactions.to_owned());
        self.add_block(next_block.clone());
        next_block
    }

    fn generate_block(&self, index: u64, prev_block_hash: u64, transactions: Vec<Transaction>) -> Block {
        let utc: DateTime<Utc> = Utc::now();
        let timestamp = utc.timestamp();
        let ref mut hasher = DefaultHasher::new();
        index.hash(hasher);
        timestamp.hash(hasher);
        transactions.iter().for_each(|tr| tr.hash(hasher));
        prev_block_hash.hash(hasher);
        let hash = hasher.finish();
        return Block {
            index,
            timestamp,
            hash,
            transactions,
            prev_block_hash,
        };
    }

    fn add_block(&mut self, new_block: Block) {
        let last_block = self.latest_block();
        if new_block.prev_block_hash == last_block.hash {
            self.chain.push(new_block);
        } else {
            panic!("Block should refer to the latest block in the chain")
        }
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Block #{} | hash: {} | timestamp: {} | txs: {} | prev block: {} |",
               self.index,
               self.hash,
               self.timestamp,
               self.transactions.len(),
               self.prev_block_hash
        )
    }
}

impl Display for BlockChain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Blockchain: [\n")?;
        for block in self.chain.iter() {
            write!(f, "\t{}\n", block)?;
        }
        write!(f, "]")
    }
}