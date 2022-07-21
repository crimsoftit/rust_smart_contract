#!/bin/bash
near call test.crimsoft.testnet increment_value '{}' --accountId crimsoft1.testnet

# increment counter
near call test.crimsoft.testnet increment_counter '{}' --accountId crimsoft1.testnet