struct Farm {
    total_reward: u64,
    reward_per_block: u64,
    acc_token_per_share: u64,
    start_block: u64,
    last_block_update: u64,
    total_supply: u64,
}

impl Farm {
    pub fn update_farm(&mut self, block_now: u64) {
        if self.last_block_update >= block_now || self.total_reward == 0 {
            return;
        }

        self.acc_token_per_share = self.acc_token_per_share + ((block_now - self.last_block_update) * self.reward_per_block / self.total_supply);
        self.total_reward = self.total_reward - (block_now - self.last_block_update) * self.reward_per_block;
    }
}

struct Farmer {
    farm: *mut Farm,
    amount: u64,
    last_reward: u64
}

impl Farmer {
    pub fn deposit(&mut self, amount: u64, block_now: u64) {
        self.amount = amount;
        unsafe {
            (*self.farm).update_farm(block_now);
            self.last_reward = self.amount * (*self.farm).acc_token_per_share;
        }
    }

    pub fn claim(&mut self, block_now: u64) {
        unsafe {
            (*self.farm).update_farm(block_now);
            self.last_reward = self.amount * (*self.farm).acc_token_per_share;
        }
    }
}

fn main() {
    let mut farm = Farm{
        total_reward: 100,
        reward_per_block: 1,
        acc_token_per_share: 0,
        start_block: 0,
        last_block_update: 0
    };

    let farm_pointer = &Farm as *mut Farm;

    let farmer1 = Farmer {
        farm: ,
        amount: 0,
        last_reward: 0
    };

    farmer1.deposit(10, 1);
}
