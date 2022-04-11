# Farming
1. Farm have `total_reward`, `start_block` and `reward_per_block` 
2. Farm start with `acc_token_per_share` is 0 and `last_reward_block` is block where Farm has been
   created
3. Every action of Farmer will update Farm
4. Update farm will update `acc_token_per_share` and `last_reward_block`
5. Update farm by:
    - Get `lp_supply` = balance_of (Farm lp token address)
    - `acc_token_per_share` = (`block now` - `last_reward_block`) * `reward_per_block` / `lp_supply`
    - `last_reward_block` is block where Farm was update
6. Farmer can `deposit` lp token, when Farmer call `deposit`, flow is:
    - Update Farm with rule was defined at step 5
    - `amount` amount of deposit
    - `last_reward` = `amount` * `Farm.acc_token_per_share`
7. Farmer can `claim` lb token, when Farmer call `claim`, flow is:
    - Update Farm with rule was defined at step 5
    - Calculate reward = `Farmer.amount` * `Farm.acc_token_per_share` - `Farmer.last_reward`
    - `Farmer.last_reward` = `Farmer.amount` * `Farm.acc_token_per_share`
