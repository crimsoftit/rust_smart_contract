//use near_sdk::{ near_bindgen, borsh::{ BorshDeserialize, BorshSerialize }, setup_alloc};

use near_sdk::borsh::{ self, BorshDeserialize, BorshSerialize };
use near_sdk::{ near_bindgen };



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
}