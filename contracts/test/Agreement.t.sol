// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {Contract as AgreementContract, Agreement, ErrNotASigner, ErrAlreadySigned} from "../src/Agreement.sol";
import "forge-std/console.sol";
import "forge-std/Vm.sol";

contract AgreementContractTest is Test {
    AgreementContract public agreementContract;

    address constant SIGNER_ADDRESS = address(169);
    uint64 constant AMOUNT = 691;

    // We need to copy that events from contract to avoid visibility errors.
    event Created(uint256 id, address signer);
    event Signed(uint256 id, address creator);

    function setUp() public {
        agreementContract = new AgreementContract();
    }

    function test_createAgreement() public {
        vm.expectEmit(true, true, false, true);
        emit Created(0, SIGNER_ADDRESS);
        uint256 agreementId = agreementContract.create(AMOUNT, SIGNER_ADDRESS);
        assertEq(0, agreementId);
        assertEq(1, agreementContract.nextId());
    }

    function test_getAgreement() public {
        uint256 agreementId = agreementContract.create(AMOUNT, SIGNER_ADDRESS);
        Agreement memory agreement = agreementContract.getById(agreementId);
        console.log("Agreement creator:", agreement.creator);
        console.log("Agreement signer:", agreement.signer);
        console.log("Agreement amount:", agreement.amount);
        console.log("Agreement status:", uint256(agreement.status));
        assertEq(address(this), agreement.creator);
        assertEq(SIGNER_ADDRESS, agreement.signer);
        assertEq(AMOUNT, agreement.amount);
        assertEq(0, uint256(agreement.status));
    }

    function test_signAgreement() public {
        uint256 agreementId = agreementContract.create(AMOUNT, SIGNER_ADDRESS);
        Agreement memory agreement_before = agreementContract.getById(agreementId);
        assertEq(0, uint256(agreement_before.status));
        vm.prank(SIGNER_ADDRESS);
        vm.expectEmit(true, true, false, true);
        emit Signed(0, address(this));
        agreementContract.sign(agreementId);
        Agreement memory agreement_after = agreementContract.getById(agreementId);
        assertEq(1, uint256(agreement_after.status));
    }

    function test_RevertWhen_ContractAlreadySigned() public {
        uint256 agreementId = agreementContract.create(AMOUNT, SIGNER_ADDRESS);
        vm.prank(SIGNER_ADDRESS);
        agreementContract.sign(agreementId);
        vm.expectRevert(ErrAlreadySigned.selector);
        agreementContract.sign(agreementId);
    }

    function test_RevertWhen_NotASigner() public {
        uint256 agreementId = agreementContract.create(AMOUNT, SIGNER_ADDRESS);
        vm.expectRevert(ErrNotASigner.selector);
        agreementContract.sign(agreementId);
    }
}
