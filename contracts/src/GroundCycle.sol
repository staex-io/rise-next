// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.22;

import {AgreementContract, Agreement, Status} from "./Agreement.sol";

error ErrReceivedNotEnough(uint256, uint256);
error ErrNoLanding();
error ErrNoApprovedLanding();
error ErrAgreementNotSigned();
error ErrRejectTooEarly();
error ErrRejectApprovedLanding();
error ErrTakeoffRequired();

struct Info {
    // If id is not zero it means landing was approved and has an id.
    uint256 id;
    address payable drone;
    address payable station;
    address payable landlord;
    uint256 timestamp;
}

contract GroundCycleContract {
    uint256 private nextId;
    // Landing wait time to check on reject request in seconds.
    uint256 private landingWaitTime;
    AgreementContract agreementContract;

    // We use station address as a key.
    // Because station is a party in agreements with drone and landlord.
    mapping(address => Info) public landings;

    event Landing(uint256, address indexed, address indexed, address indexed);
    event Takeoff(uint256, address indexed, address indexed, address indexed);
    event Reject(address indexed, address indexed);

    constructor(uint256 _landingWaitTime, AgreementContract _agreementContract) {
        // Just to not start from 0.
        // Also we do check that landing is not exists by id = 0;
        nextId = 1;
        landingWaitTime = _landingWaitTime;
        agreementContract = _agreementContract;
    }

    function landingByDrone(address payable station) external payable {
        checkAgreement(station, msg.sender);
        Info storage landing = landings[station];
        // If landing id is not zero it means landing was approved.
        if (landing.id != 0) {
            revert ErrTakeoffRequired();
        }
        // It means drone executed smart contract before station did it.
        if (landing.drone == address(0)) {
            // So we just save drone action and return to wait station exection.
            // As drone doens't know landlord we just keep it empty as address(0).
            landings[station] = Info(0, payable(msg.sender), station, payable(address(0)), block.timestamp);
            return;
        }
        // It means it is second or more time when drone executes this method, so skip.
        // But station is not land yet.
        if (landing.landlord == address(0)) {
            payable(msg.sender).transfer(msg.value);
            return;
        }
        approve(landing);
    }

    function landingByStation(address payable drone, address payable landlord) external payable {
        checkAgreement(msg.sender, landlord);
        Info storage landing = landings[msg.sender];
        // If landing id is not zero it means landing was approved.
        if (landing.id != 0) {
            revert ErrTakeoffRequired();
        }
        if (landing.drone == address(0)) {
            // It means there are landing from drone.
            landings[msg.sender] = Info(0, drone, payable(msg.sender), landlord, block.timestamp);
            return;
        } else if (landing.landlord == address(0)) {
            // It means there was landing by drone.
            landings[msg.sender].landlord = landlord;
        } else if (landing.landlord != address(0) && landing.id == 0) {
            // It means there are no landing by drone and it is not first landing by station.
            payable(msg.sender).transfer(msg.value);
            return;
        }
        approve(landing);
    }

    function takeoff() external {
        Info memory landing = landings[msg.sender];
        if (landing.id == 0) {
            revert ErrNoApprovedLanding();
        }
        emit Takeoff(landing.id, landing.drone, landing.station, landing.landlord);
        delete landings[msg.sender];
    }

    function get(address station) external view returns (Info memory) {
        Info memory landing = landings[station];
        return landing;
    }

    function reject(address station) external {
        Info memory landing = landings[station];
        if (landing.id != 0) {
            // We can't reject approved landing before takeoff because tokens (fees) were sent.
            revert ErrRejectApprovedLanding();
        }
        // Landing was not present at the moment from both sides.
        if (landing.drone == address(0)) {
            revert ErrNoLanding();
        }
        if (block.timestamp - landing.timestamp < landingWaitTime) {
            // Restrict to call this method if 5 minutes is not passed.
            // To give a time to another entity to proceed landing.
            revert ErrRejectTooEarly();
        }
        // We can check who was initiate a landing by checking landlord field in landing information.
        // If landlord is not present it means it was drone otherwise station.
        if (landing.landlord == address(0)) {
            // Landing was initiated by drone.
            require(msg.sender == landing.drone, "caller should be drone of this landing");
            sendTokens(station, landing.drone, landing.drone);
        } else {
            // Landing was initiated by station.
            require(msg.sender == landing.station, "caller should be station of this landing");
            sendTokens(station, landing.landlord, landing.station);
        }
        delete landings[station];
        emit Reject(landing.drone, station);
    }

    function approve(Info storage landing) private {
        sendTokens(landing.station, landing.drone, landing.station);
        sendTokens(landing.station, landing.landlord, landing.landlord);
        uint256 id = nextId;
        emit Landing(id, landing.drone, landing.station, landing.landlord);
        landing.id = id;
        nextId++;
    }

    function sendTokens(address station, address entity, address payable to) private {
        Agreement memory agreement = agreementContract.get(station, entity);
        (bool sent,) = to.call{value: agreement.amount}("");
        require(sent, "failed to send ether");
    }

    function checkAgreement(address station, address entity) private {
        Agreement memory agreement = agreementContract.get(station, entity);
        if (agreement.status != Status.Signed) revert ErrAgreementNotSigned();
        if (msg.value < agreement.amount) {
            // If drone sends not enough tokens to pay for agreement
            // we revert execution.
            revert ErrReceivedNotEnough(msg.value, agreement.amount);
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
