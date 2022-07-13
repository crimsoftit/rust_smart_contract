//use near_sdk::{ near_bindgen, borsh::{ BorshDeserialize, BorshSerialize }, setup_alloc};

use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::{ env, near_bindgen, Gas };



#[near_bindgen]
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    value: u32
}


impl Default for Counter {
    fn default () -> Self {
        Self {
            value: 0
        }
    }
}

#[near_bindgen]
impl Counter {

    // view function: reads the state of the blockchain
    pub fn read_value (&self) -> u32 {
        self.value
    }

    // call function: write to the blockchain - change blockchain state
    pub fn decrement_value (&mut self) -> u32 {
        if self.value > 0 {
            self.value -= 1;
        }
        self.value
    }

    // call function: write to the blockchain - change blockchain state
    pub fn increment_value (&mut self) -> u32 {
        self.value += 1;
        self.value
    }

    // return contract/wallet address
    pub fn return_wallet_address (&self) -> String {
        let c_address = env::signer_account_id().to_string();
        c_address
    }

    // return gas cost
    pub fn return_gas_details (&self) -> (Gas, Gas) {
        let available_gas= env::prepaid_gas();
        let gas_used = env::used_gas();
        (available_gas, gas_used)
    }

    // return attached deposit
    #[payable]
    pub fn return_near_bal (&mut self) -> u128 {
        env::attached_deposit()
    }
}