// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {AgreementContract, Agreement, ErrNotASigner, ErrAlreadySigned} from "../src/Agreement.sol";
import "forge-std/console.sol";
import "forge-std/Vm.sol";

contract AgreementContractTest is Test {
    AgreementContract public agreementContract;

    address constant SIGNER_ADDRESS = address(169);
    uint64 constant AMOUNT = 691;

    function setUp() public {
        agreementContract = new AgreementContract();
    }

    function test_createAgreement() public {
        vm.expectEmit(true, true, false, true);
        emit AgreementContract.AgreementCreated(0, SIGNER_ADDRESS);
        uint256 agreementId = agreementContract.createAgreement(AMOUNT, SIGNER_ADDRESS);
        assertEq(0, agreementId);
        assertEq(1, agreementContract.nextAgreementId());
    }

    function test_getAgreement() public {
        uint256 agreementId = agreementContract.createAgreement(AMOUNT, SIGNER_ADDRESS);
        Agreement memory agreement = agreementContract.getAgreementById(agreementId);
        console.log("Agreement creator:", agreement.creator);
        console.log("Agreement signer:", agreement.signer);
        console.log("Agreement amount:", agreement.amount);
        console.log("Agreement isSigned:", agreement.isSigned);
        assertEq(address(this), agreement.creator);
        assertEq(SIGNER_ADDRESS, agreement.signer);
        assertEq(AMOUNT, agreement.amount);
        assertEq(false, agreement.isSigned);
    }

    function test_signAgreement() public {
        uint256 agreementId = agreementContract.createAgreement(AMOUNT, SIGNER_ADDRESS);
        Agreement memory agreement_before = agreementContract.getAgreementById(agreementId);
        assertEq(false, agreement_before.isSigned);
        vm.prank(SIGNER_ADDRESS);
        vm.expectEmit(true, true, false, true);
        emit AgreementContract.AgreementSigned(0, address(this));
        agreementContract.signAgreement(agreementId);
        Agreement memory agreement_after = agreementContract.getAgreementById(agreementId);
        assertEq(true, agreement_after.isSigned);
    }

    function test_RevertWhen_ContractAlreadySigned() public {
        uint256 agreementId = agreementContract.createAgreement(AMOUNT, SIGNER_ADDRESS);
        vm.prank(SIGNER_ADDRESS);
        agreementContract.signAgreement(agreementId);
        vm.expectRevert(ErrAlreadySigned.selector);
        agreementContract.signAgreement(agreementId);
    }

    function test_RevertWhen_NotASigner() public {
        uint256 agreementId = agreementContract.createAgreement(AMOUNT, SIGNER_ADDRESS);
        vm.expectRevert(ErrNotASigner.selector);
        agreementContract.signAgreement(agreementId);
    }
}
