// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {AgreementContract, Agreement, Status} from "./Agreement.sol";

error ErrReceivedNotEnough(uint256, uint256);
error ErrNoLanding();
error ErrNotSigned();

struct Info {
    uint256 id;
    address drone;
    address payable station;
    address payable landlord;
}

contract GroundCycleContract {
    uint256 public nextId;
    AgreementContract agreementContract;

    // We use station address as a key.
    // Because station is a party in agreements with drone and landlord.
    mapping(address => Info) public landings;

    event Landing(uint256, address indexed, address indexed, address indexed);
    event Takeoff(uint256, address indexed, address indexed, address indexed);

    constructor(AgreementContract _agreementContract) {
        // Just to not start from 0.
        // Also we do check that landing is not exists by id = 0;
        nextId = 1;
        agreementContract = _agreementContract;
    }

    function landingByDrone(address payable station) external payable {
        checkAgreement(station, msg.sender);
        Info storage landing = landings[station];
        // It means drone executed smart contract before station did it.
        if (landing.drone == address(0)) {
            // So we just save drone action and return to wait station exection.
            // As drone doens't know landlord we just keep it empty as address(0).
            landings[station] = Info(0, msg.sender, station, payable(address(0)));
            return;
        }
        approve(landing);
    }

    function landingByStation(address drone, address payable landlord) external payable {
        checkAgreement(msg.sender, landlord);
        Info storage landing = landings[msg.sender];
        if (landing.drone == address(0)) {
            landings[msg.sender] = Info(0, drone, payable(msg.sender), landlord);
            return;
        } else if (landing.landlord == address(0)) {
            landings[msg.sender].landlord = landlord;
        }
        approve(landing);
    }

    function takeoff() external {
        Info memory landing = landings[msg.sender];
        if (landing.id == 0) {
            revert ErrNoLanding();
        }
        emit Takeoff(landing.id, landing.drone, landing.station, landing.landlord);
        delete landings[msg.sender];
    }

    function get(address station) external view returns (Info memory) {
        Info memory landing = landings[station];
        return landing;
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
        if (agreement.status != Status.Signed) revert ErrNotSigned();
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
