// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

struct Agreement {
    address creator;
    address signer;
    uint64 amount;
    bool isSigned;
}

error ErrNotASigner();
error ErrAlreadySigned();

contract AgreementContract {
    mapping(uint256 => Agreement) public agreements;
    uint256 public nextAgreementId;

    event AgreementCreated(uint256 agreementId, address signer);
    event AgreementSigned(uint256 agreementId, address creator);

    function createAgreement(uint64 amount, address signer) public returns (uint256) {
        uint256 agreementId = nextAgreementId;
        agreements[agreementId] = Agreement(msg.sender, signer, amount, false);
        emit AgreementCreated(agreementId, signer);
        nextAgreementId++;
        return agreementId;
    }

    function signAgreement(uint256 agreementId) public {
        Agreement storage agreement = agreements[agreementId];
        if (agreement.isSigned) {
            revert ErrAlreadySigned();
        }
        if (agreement.signer != msg.sender) {
            revert ErrNotASigner();
        }
        agreement.signer = msg.sender;
        agreement.isSigned = true;
        emit AgreementSigned(agreementId, agreement.creator);
    }

    function getAgreementById(uint256 agreementId) public view returns (Agreement memory) {
        Agreement memory agreement = agreements[agreementId];
        return agreement;
    }
}
