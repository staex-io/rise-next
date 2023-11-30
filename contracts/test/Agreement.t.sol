// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {
    Contract as AgreementContract,
    Agreement,
    ErrAlreadySigned,
    ErrNoAgreement,
    ErrInvalidAmount
} from "../src/Agreement.sol";
import "forge-std/Vm.sol";

contract AgreementContractTest is Test {
    address constant SIGNER_ADDRESS = address(169);
    uint256 constant AMOUNT = 691 ether;

    AgreementContract public agreementContract;

    // We need to copy that events from contract to avoid visibility errors.
    event Created(address indexed, address indexed);
    event Signed(address indexed, address indexed);

    function setUp() public {
        agreementContract = new AgreementContract();
    }

    function test_createAgreement() public {
        vm.expectEmit(true, true, true, true);
        emit Created(address(this), SIGNER_ADDRESS);
        agreementContract.create(SIGNER_ADDRESS, AMOUNT);
    }

    function test_getAgreement() public {
        agreementContract.create(SIGNER_ADDRESS, AMOUNT);
        Agreement memory agreement = agreementContract.get(address(this), SIGNER_ADDRESS);
        assertEq(AMOUNT, agreement.amount);
        assertEq(1, uint256(agreement.status));
    }

    function test_signAgreement() public {
        agreementContract.create(SIGNER_ADDRESS, AMOUNT);
        Agreement memory agreement_before = agreementContract.get(address(this), SIGNER_ADDRESS);
        assertEq(1, uint256(agreement_before.status));
        vm.prank(SIGNER_ADDRESS);
        vm.expectEmit(true, true, true, true);
        emit Signed(address(this), SIGNER_ADDRESS);
        agreementContract.sign(address(this), AMOUNT);
        Agreement memory agreement_after = agreementContract.get(address(this), SIGNER_ADDRESS);
        assertEq(2, uint256(agreement_after.status));
    }

    function test_RevertWhen_AlreadySigned_OnCreate() public {
        agreementContract.create(SIGNER_ADDRESS, AMOUNT);
        vm.prank(SIGNER_ADDRESS);
        agreementContract.sign(address(this), AMOUNT);
        vm.expectRevert(ErrAlreadySigned.selector);
        agreementContract.create(SIGNER_ADDRESS, AMOUNT);
    }

    function test_RevertWhen_AlreadySigned_OnSign() public {
        agreementContract.create(SIGNER_ADDRESS, AMOUNT);
        vm.startPrank(SIGNER_ADDRESS);
        agreementContract.sign(address(this), AMOUNT);
        vm.expectRevert(ErrAlreadySigned.selector);
        agreementContract.sign(address(this), AMOUNT);
        vm.stopPrank();
    }

    function test_RevertWhen_NoAgreement_OnSign() public {
        vm.prank(SIGNER_ADDRESS);
        vm.expectRevert(ErrNoAgreement.selector);
        agreementContract.sign(address(this), AMOUNT);
    }

    function test_RevertWhen_NoAgreement_OnGet() public {
        vm.expectRevert(ErrNoAgreement.selector);
        agreementContract.get(address(this), SIGNER_ADDRESS);
    }

    function test_RevertWhen_InvalidAmount() public {
        agreementContract.create(SIGNER_ADDRESS, AMOUNT);
        vm.prank(SIGNER_ADDRESS);
        bytes4 selector = bytes4(keccak256("ErrInvalidAmount(uint256)"));
        vm.expectRevert(abi.encodeWithSelector(selector, AMOUNT));
        agreementContract.sign(address(this), 1 ether);
    }
}
