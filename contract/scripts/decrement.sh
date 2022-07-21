#!/bin/bash
near call test.crimsoft.testnet decrement_value '{}' --accountId crimsoft1.testnet

# decrement counter
near call test.crimsoft.testnet decrement_counter '{}' --accountId crimsoft1.testnet