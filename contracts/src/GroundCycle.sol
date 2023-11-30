// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Contract as AgreementContract, Agreement, Status} from "./Agreement.sol";

error ErrReceivedNotEnough(uint256);

struct PendingLanding {
    address drone;
    address payable station;
    address payable landlord;
}

contract Contract {
    uint256 public nextId;
    AgreementContract agreementContract;

    // We use station address as a key.
    // Because station is a party in agreements with drone and landlord.
    mapping(address => PendingLanding) public pendingLandings;

    event Landging(uint256, address indexed, address indexed, address indexed);

    constructor(AgreementContract _agreementContract) {
        // Just to not start from 0.
        nextId = 1;
        agreementContract = _agreementContract;
    }

    // todo: how to make it idempotency?
    // todo: what if drone executed droneLanding twice?
    function droneLanding(address payable station) external payable {
        checkAgreement(station, msg.sender);
        PendingLanding storage pending = pendingLandings[station];
        // It means drone executed smart contract before station did it.
        if (pending.drone == address(0)) {
            // So we just save drone action and return to wait station exection.
            // As drone doens't know landlord we just keep it empty as address(0).
            pendingLandings[station] = PendingLanding(msg.sender, station, payable(address(0)));
            return;
        }
        approveLanding(pending);
    }

    // todo: how to make it idempotency?
    // todo: what if drone executed stationLanding twice?
    function stationLanding(address drone, address payable landlord) external payable {
        checkAgreement(msg.sender, landlord);
        PendingLanding storage pending = pendingLandings[msg.sender];
        if (pending.drone == address(0)) {
            pendingLandings[msg.sender] = PendingLanding(drone, payable(msg.sender), landlord);
            return;
        } else if (pending.landlord == address(0)) {
            pendingLandings[msg.sender].landlord = landlord;
        }
        approveLanding(pending);
    }

    function getPending(address station) external view returns (PendingLanding memory) {
        PendingLanding memory pending = pendingLandings[station];
        return pending;
    }

    function approveLanding(PendingLanding storage pending) private {
        sendTokens(pending.station, pending.drone, pending.station);
        sendTokens(pending.station, pending.landlord, pending.landlord);
        uint256 id = nextId;
        emit Landging(id, pending.drone, pending.station, pending.landlord);
        nextId++;
        delete pendingLandings[pending.station];
    }

    function sendTokens(address statio, address entity, address payable to) private {
        Agreement memory agreement = agreementContract.get(statio, entity);
        (bool sent,) = to.call{value: agreement.amount}("");
        require(sent, "failed to send ether");
    }

    function checkAgreement(address station, address entity) private {
        Agreement memory agreement = agreementContract.get(station, entity);
        if (msg.value < agreement.amount) {
            // If drone sends not enough tokens to pay for agreement
            // we revert execution.
            revert ErrReceivedNotEnough(agreement.amount);
        }
    }

    // Following methods are need to received tokens.
    receive() external payable {}
    fallback() external payable {}

    // To check contract balance.
    function getBalance() public view returns (uint256) {
        return address(this).balance;
    }
}
