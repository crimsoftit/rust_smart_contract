#!/bin/bash

# read value
near view test.crimsoft.testnet read_value

# return contract/wallet address
near call test.crimsoft.testnet return_wallet_address --accountId crimsoft1.testnet

# return available and used gas - in that order. also attach gas
near call test.crimsoft.testnet return_gas_details --accountId crimsoft1.testnet --gas 7000000000000

# return attached deposit (available near balance - payable function)
near call test.crimsoft.testnet return_near_bal --accountId crimsoft1.testnet