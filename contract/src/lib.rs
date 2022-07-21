//use near_sdk::{ near_bindgen, borsh::{ BorshDeserialize, BorshSerialize }, setup_alloc};

use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::{ env, near_bindgen, Gas };
use near_sdk::collections::Vector;


#[near_bindgen]
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct Counter {
    value: u32,
    names: Vec<String>
}


impl Default for Counter {
    fn default () -> Self {
        Self {
            value: 0,
            names: vec![]
        }
    }
}

#[near_bindgen]
impl Counter {

    pub fn save_name (&mut self, name: String) -> String {
        self.names.push(name.to_string());
        name.to_string()
    }

    pub fn get_names (&self) -> Vec<String> {
        let mut dem_names: Vec<String> = vec![];

        for name in self.names.iter() {
            dem_names.push(name.to_string());
        }
        dem_names
    }

    pub fn pop_name(&mut self) -> bool {
        self.names.pop();
        true
    }
 
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

    // return contract/wallet address - call function
    pub fn return_wallet_address (&self) -> String {
        let c_address = env::signer_account_id().to_string();
        c_address
    }

    // return gas cost - call function
    pub fn return_gas_details (&self) -> (Gas, Gas) {
        let available_gas= env::prepaid_gas();
        let gas_used = env::used_gas();
        (available_gas, gas_used)
    }

    // return attached deposit - call function
    #[payable]
    pub fn return_near_bal (&mut self) -> u128 {
        env::attached_deposit()
    }
}