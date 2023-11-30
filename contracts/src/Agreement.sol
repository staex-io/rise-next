// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

// We need Empty because it is 0
// and we can check it as an empty values in methods.
enum Status {
    Empty,
    Created,
    Signed
}

error ErrAlreadySigned();
error ErrNoAgreement();
error ErrInvalidAmount(uint256);

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
            // We restrict to recreate alread signed agreement.
            // Because it can violate signed agreement rules without entitity party.
            revert ErrAlreadySigned();
        }
        agreements[msg.sender][entity] = Agreement(amount, Status.Created);
        emit Created(msg.sender, entity);
    }

    function sign(address station, uint256 amount) external {
        Agreement storage agreement = agreements[station][msg.sender];
        if (agreement.status == Status.Empty) {
            // We can't sign agreement which is not exists.
            revert ErrNoAgreement();
        }
        if (agreement.status == Status.Signed) {
            // What the point to sign signed agreement?
            // Also it can change previous agreement rules.
            revert ErrAlreadySigned();
        }
        if (agreement.amount != amount) {
            // We check for amount because creator can create another agreement after sending to entitity.
            // To prevent signing unexpected agreement we do such check.
            revert ErrInvalidAmount(agreement.amount);
        }
        agreement.status = Status.Signed;
        emit Signed(station, msg.sender);
    }

    function get(address station, address entity) external view returns (Agreement memory) {
        Agreement memory agreement = agreements[station][entity];
        if (agreement.status == Status.Empty) {
            revert ErrNoAgreement();
        }
        return agreement;
    }
}
