# rust-new-project-template

A good starting point for a new Rust project

## References

* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)

## Project1: rust_combination_sum.rs

This program is to find all valid combinations of k numbers that sum up to n such that the following conditions are true:

Only numbers 1 through 9 are used.
Each number is used at most once.
Return a list of all possible valid combinations. The list must not contain the same combination twice, and the combinations may be returned in any order.

## Project2: rust_replace_blanks.rs

This program is to replace every blank with %20
Return a new string with every blank replaced with %20 according to the input string

## Project3: rust_sliding_window_maximum.rs

We are given an array of integers nums, there is a sliding window of size k which is moving from the very left of the array to the very right. We can only see the k numbers in the window. Each time the sliding window moves right by one position.

Return the max sliding window.

## Project4: rust_ip.rs

A valid IP address consists of exactly four integers separated by single dots. Each integer is between 0 and 255 (inclusive) and cannot have leading zeros.

For example, "0.1.2.201" and "192.168.1.1" are valid IP addresses, but "0.011.255.245", "192.168.1.312" and "192.168@1.1" are invalid IP addresses.
Given a string s containing only digits, return all possible valid IP addresses that can be formed by inserting dots into s. Return the valid IP addresses in any order.

##Project5: rust_lemonade_change.rs
At a lemonade stand, each lemonade costs $5. Customers are standing in a queue to buy from us and order one at a time (in the order specified by bills). Each customer will only buy one lemonade and pay with either a $5, $10, or $20 bill. We must provide the correct change to each customer so that the net transaction is that the customer pays $5.

Note that we do not have any change in hand at first.

Given an integer array bills where bills[i] is the bill the ith customer pays, return true if we can provide every customer with the correct change, or false otherwise.

## Project6: rust_stock.rs

We are given an array prices where prices[i] is the price of a given stock on the ith day.

We want to maximize our profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

Return the maximum profit we can achieve from this transaction. If we cannot achieve any profit, return 0.
