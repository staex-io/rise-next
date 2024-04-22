// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.22;

contract DataProvingContract {
    string private inner_hash;

    function save(string calldata hash) external {
        inner_hash = hash;
    }
}
