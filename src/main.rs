mod block;
mod blockchain;

use blockchain::Blockchain;

fn main() {
    let mut blockchain = Blockchain::new();
    println!("Genesis Block Hash: {}", blockchain.chain[0].hash);
    
    blockchain.add_block("Alice sends 1 BTC to Bob".to_string(), 2);
    blockchain.add_block("Bob sends 0.5 BTC to Carol".to_string(), 2);
    
    println!("{:#?}", blockchain);
    println!("Blockchain valid? {}", blockchain.is_valid());  // Should print "true"
    
    blockchain.chain[1].data = "Eve sends 100 BTC to herself".to_string();
    println!("Blockchain valid after tampering? {}", blockchain.is_valid()); // Should print "false"
}