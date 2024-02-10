// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.22;

struct DID {
    string location;
    uint256 price;
}

contract DIDContract {
    mapping(address => DID) dids;

    event Updated(address indexed, string location, uint256 price);
    event Removed(address indexed);

    function update(string calldata location, uint256 price) external {
        DID storage did = dids[msg.sender];
        if (keccak256(abi.encodePacked(location)) != keccak256(abi.encodePacked(did.location))) {
            did.location = location;
        }
        if (price != did.price) {
            did.price = price;
        }
        dids[msg.sender] = did;
        emit Updated(msg.sender, location, price);
    }

    // Remove did from smart contract storage and spawn a Removed event.
    function remove() external {
        delete dids[msg.sender];
        emit Removed(msg.sender);
    }

    function get(address entity) external view returns (DID memory) {
        return dids[entity];
    }
}
