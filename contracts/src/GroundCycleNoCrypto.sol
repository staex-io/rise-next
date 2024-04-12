// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.22;

error ErrNoLanding();
error ErrNoApprovedLanding();
error ErrRejectTooEarly();
error ErrRejectApprovedLanding();
error ErrTakeoffRequired();
error ErrHandshake();

struct Info {
    // If id is not zero it means landing was approved and has an id.
    uint256 id;
    address drone;
    address station;
    address landlord;
    uint256 timestamp;
}

contract GroundCycleNoCryptoContract {
    uint256 private nextId;
    // Landing wait time to check on reject request in seconds.
    uint256 private landingWaitTime;

    // We use station address as a key.
    // Because station is a party in agreements with drone and landlord.
    mapping(address => Info) public landings;

    event Landing(uint256, address indexed, address indexed, address indexed);
    event Takeoff(uint256, address indexed, address indexed, address indexed);
    event Reject(address indexed, address indexed);

    constructor(uint256 _landingWaitTime) {
        // Just to not start from 0.
        // Also we do check that landing is not exists by id = 0;
        nextId = 1;
        landingWaitTime = _landingWaitTime;
    }

    function landingByDrone(address station) external {
        Info storage landing = landings[station];
        // If landing id is not zero it means landing was approved.
        if (landing.id != 0) {
            revert ErrTakeoffRequired();
        }
        // It means drone executed smart contract before station did it.
        if (landing.drone == address(0)) {
            // So we just save drone action and return to wait station exection.
            // As drone doens't know landlord we just keep it empty as address(0).
            landings[station] = Info(0, msg.sender, station, address(0), block.timestamp);
            return;
        }
        // It means it is second or more time when drone executes this method, so skip.
        // But station is not landed yet.
        if (landing.landlord == address(0)) {
            return;
        }
        // Passing previous check means station is already executed
        // landing, so we can try to approve it.
        if (msg.sender != landing.drone) {
            revert ErrHandshake();
        }
        approve(landing);
    }

    function landingByStation(address drone, address landlord) external {
        Info storage landing = landings[msg.sender];
        // If landing id is not zero it means landing was approved.
        if (landing.id != 0) {
            revert ErrTakeoffRequired();
        }
        if (landing.drone == address(0)) {
            // It means there are landing from drone.
            landings[msg.sender] = Info(0, drone, msg.sender, landlord, block.timestamp);
            return;
        } else if (landing.landlord == address(0)) {
            // It means there was landing by drone.
            landings[msg.sender].landlord = landlord;
        } else if (landing.landlord != address(0) && landing.id == 0) {
            // It means there are no landing by drone and it is not first landing by station.
            return;
        }
        // Passing previous check means drone is already executed
        // landing, so we can try to approve it.
        if (landing.drone != drone) {
            revert ErrHandshake();
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
        } else {
            // Landing was initiated by station.
            require(msg.sender == landing.station, "caller should be station of this landing");
        }
        delete landings[station];
        emit Reject(landing.drone, station);
    }

    function approve(Info storage landing) private {
        uint256 id = nextId;
        emit Landing(id, landing.drone, landing.station, landing.landlord);
        landing.id = id;
        nextId++;
    }
}
