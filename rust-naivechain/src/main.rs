use blockchain::BlockChain;
use blockchain::Transaction;

mod blockchain;

fn main() {
    let genesis_block = BlockChain::generate_genesis_block();
    let blockchain = &mut BlockChain::new(genesis_block);

    blockchain.send_transaction(Transaction { payload: "payload".to_string() });
    let block = blockchain.produce_block();

    println!("{}", block);
    println!("{}", blockchain);
}