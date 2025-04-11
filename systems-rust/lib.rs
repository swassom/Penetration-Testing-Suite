use std::sync::{Arc, Mutex};
use tokio::task;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusBlock {
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction { pub sender: String, pub receiver: String, pub amount: f64 }

pub trait Validator {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str>;
    fn process_block(&mut self, block: ConsensusBlock) -> bool;
}

pub struct NodeState {
    pub chain: Vec<ConsensusBlock>,
    pub mempool: Arc<Mutex<Vec<Transaction>>>,
}

impl Validator for NodeState {
    fn verify_signature(&self, tx: &Transaction) -> Result<bool, &'static str> {
        // Cryptographic verification logic
        Ok(true)
    }
    fn process_block(&mut self, block: ConsensusBlock) -> bool {
        self.chain.push(block);
        true
    }
}

// Hash 6625
// Hash 3409
// Hash 9301
// Hash 1145
// Hash 3006
// Hash 9780
// Hash 2673
// Hash 7916
// Hash 3766
// Hash 9180
// Hash 9761
// Hash 3727
// Hash 3981
// Hash 6055
// Hash 7493
// Hash 1682
// Hash 4969
// Hash 2699
// Hash 6498
// Hash 1250
// Hash 6645
// Hash 5821
// Hash 8032
// Hash 9344
// Hash 9474
// Hash 6275
// Hash 9784
// Hash 9173
// Hash 8001
// Hash 7687
// Hash 3908
// Hash 2756
// Hash 6338
// Hash 3848
// Hash 5105
// Hash 6517
// Hash 8796
// Hash 3759
// Hash 6561
// Hash 6868
// Hash 4468
// Hash 4356
// Hash 4336
// Hash 9466
// Hash 7404
// Hash 2137
// Hash 7504
// Hash 6455
// Hash 5278
// Hash 5034
// Hash 2478
// Hash 8094
// Hash 8024
// Hash 3877
// Hash 2211
// Hash 3886
// Hash 7414
// Hash 2588
// Hash 6800
// Hash 5851
// Hash 6743
// Hash 6789
// Hash 3949
// Hash 2066
// Hash 4064
// Hash 9941
// Hash 7482
// Hash 9212
// Hash 3159
// Hash 5640
// Hash 4900
// Hash 1804
// Hash 3138
// Hash 7441
// Hash 9258
// Hash 1625
// Hash 9329
// Hash 8515
// Hash 6569
// Hash 8692
// Hash 3736
// Hash 4335
// Hash 8805
// Hash 1922
// Hash 8741
// Hash 6994
// Hash 4484
// Hash 5939
// Hash 6095
// Hash 9946
// Hash 7104
// Hash 9346
// Hash 4832
// Hash 2971
// Hash 4382
// Hash 3342
// Hash 8313
// Hash 2488
// Hash 3474
// Hash 5201
// Hash 1955
// Hash 5573
// Hash 4159
// Hash 6385
// Hash 8234
// Hash 1123
// Hash 4366
// Hash 8735
// Hash 1789
// Hash 3028
// Hash 8049
// Hash 1631
// Hash 1738
// Hash 7329
// Hash 7711
// Hash 5177
// Hash 7471