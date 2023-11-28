// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Contract as AgreementContract, Agreement} from "./Agreement.sol";

struct PendingLanding {
    address drone;
    address payable station;
    address payable landlord;
}

contract Contract {
    AgreementContract agreementContract;

    uint256 public nextId;

    // We use station address as a key.
    // Because station is a party in agreements with drone and landlord.
    mapping(address => PendingLanding) public pendingLandings;

    event Landging(uint256, address indexed, address indexed, address indexed);

    constructor() {
        // Just to not start from 0.
        nextId = 1;
    }

    // todo: how to make it idempotency?
    function droneLanding(address payable station) external payable {
        PendingLanding storage pending = pendingLandings[station];
        Agreement memory agreement = agreementContract.get(station, msg.sender);
        if (agreement.amount < msg.value) {
            //todo: revert and why
        }
        if (pending.drone == address(0)) {
            pendingLandings[station] = PendingLanding(msg.sender, station, payable(address(0)));
            return;
        }
        approveLanding(pending);
    }

    // todo: how to make it idempotency?
    function stationLanding(address drone, address payable landlord) external payable {
        PendingLanding storage pending = pendingLandings[msg.sender];
        Agreement memory agreement = agreementContract.get(msg.sender, landlord);
        if (agreement.amount < msg.value) {
            //todo: revert and why
        }
        if (pending.drone == address(0)) {
            pendingLandings[msg.sender] = PendingLanding(drone, payable(msg.sender), landlord);
            return;
        } else if (pending.landlord == address(0)) {
            pendingLandings[msg.sender].landlord = landlord;
        }
        approveLanding(pending);
    }

    function approveLanding(PendingLanding storage pending) private {
        sendTokens(pending.station, pending.drone, pending.station);
        sendTokens(pending.station, pending.landlord, pending.landlord);
        uint256 id = nextId;
        emit Landging(id, pending.drone, pending.station, pending.landlord);
        nextId++;
    }

    function sendTokens(address statio, address entity, address payable to) private {
        Agreement memory agreement = agreementContract.get(statio, entity);
        (bool sent, bytes memory data) = to.call{value: agreement.amount}("");
        require(sent, "failed to send ether");
    }

    // Following methods are need to received tokens.
    receive() external payable {}
    fallback() external payable {}

    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }
}
