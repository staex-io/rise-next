// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

error ErrNotASigner();
error ErrAlreadySigned();

enum Status {
    Created,
    Signed
}

struct Agreement {
    address creator;
    address signer;
    uint64 amount;
    Status status;
}

contract Contract {
    mapping(uint256 => Agreement) public agreements;
    uint256 public nextId;

    event Created(uint256 id, address signer);
    event Signed(uint256 id, address creator);

    function create(uint64 amount, address signer) public returns (uint256) {
        uint256 id = nextId;
        agreements[id] = Agreement(msg.sender, signer, amount, Status.Created);
        emit Created(id, signer);
        nextId++;
        return id;
    }

    function sign(uint256 id) public {
        Agreement storage agreement = agreements[id];
        if (agreement.status == Status.Signed) {
            revert ErrAlreadySigned();
        }
        if (agreement.signer != msg.sender) {
            revert ErrNotASigner();
        }
        agreement.signer = msg.sender;
        agreement.status = Status.Signed;
        emit Signed(id, agreement.creator);
    }

    function getById(uint256 id) public view returns (Agreement memory) {
        Agreement memory agreement = agreements[id];
        return agreement;
    }
}
