#!/bin/bash

# read value
near view test.crimsoft.testnet read_value

# return contract/wallet address
near call test.crimsoft.testnet return_wallet_address --accountId crimsoft1.testnet

# return available and used gas - in that order. also attach gas
near call test.crimsoft.testnet return_gas_details --accountId crimsoft1.testnet --gas 7000000000000

# return attached deposit (available near balance - payable function)
near call test.crimsoft.testnet return_near_bal --accountId crimsoft1.testnet

# use vectors to save a name in the near testnet blockchain - call function
near call test.crimsoft.testnet save_name '{ "name": "manu" }' --accountId crimsoft1.testnet
near call test.crimsoft.testnet save_name '{ "name": "mmoja" }' --accountId crimsoft1.testnet
near call test.crimsoft.testnet save_name '{ "name": "mnasty" }' --accountId crimsoft1.testnet

# get names stored in a vector - view function
near view test.crimsoft.testnet get_names '{}'

# pop the last name stored in the vector - call function
near call test.crimsoft.testnet pop_name --accountId crimsoft1.testnet

# get names after popping the last one - view function
near view test.crimsoft.testnet get_names '{}'
