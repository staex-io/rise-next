// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

error ErrNotASigner();
error ErrAlreadySigned();

// We need Empty because it is 0
// and we can check it as an empty values in methods.
enum Status {
    Empty,
    Created,
    Signed
}

struct Agreement {
    uint256 amount;
    Status status;
}

contract Contract {
    // We use station address as a key because station
    // should have agreements with drones and landlords.
    mapping(address => mapping(address => Agreement)) agreements;

    event Created(address indexed, address indexed);
    event Signed(address indexed, address indexed);

    function create(address entity, uint256 amount) external {
        if (agreements[msg.sender][entity].status == Status.Signed) {
            // todo: revert and why
        }
        agreements[msg.sender][entity] = Agreement(amount, Status.Created);
        emit Created(msg.sender, entity);
    }

    function sign(address station, uint256 amount) external {
        Agreement storage agreement = agreements[station][msg.sender];
        // todo: not a signer?
        if (agreement.status == Status.Empty) {
            // todo: revert and why
        }
        if (agreement.status == Status.Signed) {
            // todo: revert and why
        }
        if (agreement.amount != amount) {
            // todo: maybe amount as a paramenter because station can create another contract before signing?
            // todo: revert and why
        }
        agreement.status = Status.Signed;
        emit Signed(station, msg.sender);
    }

    function get(address station, address entity) external view returns (Agreement memory) {
        Agreement memory agreement = agreements[station][entity];
        if (agreement.status == Status.Empty) {
            // todo: revert and why
        }
        return agreement;
    }
}
