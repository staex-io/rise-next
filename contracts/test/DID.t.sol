// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.22;

import "forge-std/Test.sol";
import {DIDContract, DID} from "../src/DID.sol";
import "forge-std/Vm.sol";
import {VmSafe} from "forge-std/Vm.sol";

contract GroundCycleTest is Test {
    string constant LOCATION = "1,1";
    uint256 constant PRICE = 69;

    DIDContract public didContract;

    // We need to copy that events from contract to avoid visibility errors.
    event Updated(address indexed, string location, uint256 price);
    event Removed(address indexed);

    function setUp() public {
        didContract = new DIDContract();
    }

    function test_All() public {
        VmSafe.Wallet memory station = vm.createWallet("station");
        vm.deal(station.addr, 1 ether);

        vm.startPrank(station.addr);

        DID memory did_before_update = didContract.get(station.addr);
        assertEq("", did_before_update.location);
        assertEq(0, did_before_update.price);

        vm.expectEmit(true, true, true, true);
        emit Updated(station.addr, LOCATION, PRICE);
        didContract.update(LOCATION, PRICE);

        DID memory did_after_update = didContract.get(station.addr);
        assertEq(LOCATION, did_after_update.location);
        assertEq(PRICE, did_after_update.price);

        vm.expectEmit(true, true, true, true);
        emit Removed(station.addr);
        didContract.remove();

        DID memory did_after_delete = didContract.get(station.addr);
        assertEq("", did_after_delete.location);
        assertEq(0, did_after_delete.price);

        vm.stopPrank();
    }
}
