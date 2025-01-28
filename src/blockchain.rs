use crate::block::Block;  

#[derive(Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        let mut genesis_block = Block::new(
            0,
            "Genesis Block".to_string(),
            "0".to_string(),
            0 
        );

        genesis_block.hash = genesis_block.calculate_hash();
        
        return Blockchain{chain: vec![genesis_block]};
    }

    pub fn add_block(&mut self, data: String, difficulty: u32) {
        let previous_block = self.chain.last().unwrap();
        let mut new_block = Block::new(
            previous_block.index + 1,
            data, 
            previous_block.hash.clone(), 
            0
        );
        new_block.mine_block(difficulty);
        self.chain.push(new_block);
    }

    pub fn is_valid(&self) -> bool {
        for i in 1..self.chain.len() {
            let current = &self.chain[i];
            let previous = &self.chain[i - 1];
            
            if current.previous_hash != previous.hash {
                return false;
            }
            
            if current.hash != current.calculate_hash() {
                return false;
            }
        }
        return true;
    }
}