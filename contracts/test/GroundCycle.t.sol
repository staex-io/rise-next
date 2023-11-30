// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {
    Contract as GroundCycleContract,
    PendingLanding,
    ErrReceivedNotEnough,
    ErrNoAgreement
} from "../src/GroundCycle.sol";
import {Contract as AgreementContract} from "../src/Agreement.sol";
import "forge-std/Vm.sol";
import {VmSafe} from "forge-std/Vm.sol";

contract GroundCycleContractTest is Test {
    string constant DRONE_WALLET_NAME = "drone";
    string constant STATION_WALLET_NAME = "station";
    string constant LANDLORD_WALLET_NAME = "landlord";
    uint256 constant START_TOKENS = 100 ether;
    uint256 constant DRONE_STATION_AMOUNT = 3 ether;
    uint256 constant STATION_LANDLORD_AMOUNT = 5 ether;

    GroundCycleContract public gcContract;
    AgreementContract public agContract;

    event Landging(uint256, address indexed, address indexed, address indexed);

    function setUp() public {
        agContract = new AgreementContract();
        gcContract = new GroundCycleContract(agContract);
    }

    function test_All() public {
        // Create wallets for entities.
        VmSafe.Wallet memory drone = vm.createWallet(DRONE_WALLET_NAME);
        VmSafe.Wallet memory station = vm.createWallet(STATION_WALLET_NAME);
        VmSafe.Wallet memory landlord = vm.createWallet(LANDLORD_WALLET_NAME);
        // Set balances.
        vm.deal(drone.addr, START_TOKENS);
        vm.deal(station.addr, START_TOKENS);
        vm.deal(landlord.addr, START_TOKENS);

        /*
          Check no agreements logic for drone and for station.
        */
        // Check for drone.
        vm.prank(drone.addr);
        vm.expectRevert(ErrNoAgreement.selector);
        gcContract.droneLanding{value: 1 ether}(payable(station.addr));
        // Check for station.
        vm.prank(station.addr);
        vm.expectRevert(ErrNoAgreement.selector);
        gcContract.stationLanding{value: 1 ether}(drone.addr, payable(landlord.addr));

        // Initiate agreements between entities.
        vm.startPrank(station.addr);
        agContract.create(drone.addr, DRONE_STATION_AMOUNT);
        agContract.create(landlord.addr, STATION_LANDLORD_AMOUNT);
        vm.stopPrank();

        /*
          Check sent not enough tokens logic for drone and station.
        */
        // Check for drone.
        vm.prank(drone.addr);
        vm.expectRevert(ErrReceivedNotEnough.selector);
        gcContract.droneLanding{value: 1 ether}(payable(station.addr));
        // Check for station.
        vm.prank(station.addr);
        vm.expectRevert(ErrReceivedNotEnough.selector);
        gcContract.stationLanding{value: 1 ether}(drone.addr, payable(landlord.addr));

        /*
          Check that drone can send their transaction first and after that station.
        */
        // Check that drone can execute their landing method.
        vm.prank(drone.addr);
        gcContract.droneLanding{value: DRONE_STATION_AMOUNT}(payable(station.addr));
        assertEq(gcContract.getBalance(), DRONE_STATION_AMOUNT);
        PendingLanding memory pendingAfterDroneLanding = gcContract.getPending(station.addr);
        assertEq(drone.addr, pendingAfterDroneLanding.drone);
        assertEq(station.addr, pendingAfterDroneLanding.station);
        assertEq(address(0), pendingAfterDroneLanding.landlord);
        // Check that station can execute their landing method and landing should be approved.
        vm.prank(station.addr);
        vm.expectEmit(true, true, true, true);
        emit Landging(1, drone.addr, station.addr, landlord.addr);
        gcContract.stationLanding{value: STATION_LANDLORD_AMOUNT}(drone.addr, payable(landlord.addr));
        // Now we have balance 0 because we send tokens from smart contract to station and to landlord.
        assertEq(gcContract.getBalance(), 0);
        // Check that balances were updated.
        assertEq(START_TOKENS - DRONE_STATION_AMOUNT, drone.addr.balance);
        assertEq(START_TOKENS - STATION_LANDLORD_AMOUNT + DRONE_STATION_AMOUNT, station.addr.balance);
        assertEq(START_TOKENS + STATION_LANDLORD_AMOUNT, landlord.addr.balance);

        // Revert balances for future tests.
        vm.deal(drone.addr, START_TOKENS);
        vm.deal(station.addr, START_TOKENS);
        vm.deal(landlord.addr, START_TOKENS);

        /*
          Check that station can send their transaction first and after that drone.
        */
        // Check that station can execute their landing method.
        vm.prank(station.addr);
        gcContract.stationLanding{value: STATION_LANDLORD_AMOUNT}(drone.addr, payable(landlord.addr));
        assertEq(gcContract.getBalance(), STATION_LANDLORD_AMOUNT);
        PendingLanding memory pendingAfterStationLanding = gcContract.getPending(station.addr);
        assertEq(drone.addr, pendingAfterStationLanding.drone);
        assertEq(station.addr, pendingAfterStationLanding.station);
        assertEq(landlord.addr, pendingAfterStationLanding.landlord);
        // Check that drone can execute their landing method and landing should be approved.
        vm.prank(drone.addr);
        vm.expectEmit(true, true, true, true);
        emit Landging(2, drone.addr, station.addr, landlord.addr);
        gcContract.droneLanding{value: DRONE_STATION_AMOUNT}(payable(station.addr));
        // Now we have balance 0 because we send tokens from smart contract to station and to landlord.
        assertEq(gcContract.getBalance(), 0);
        // Check that balances were updated.
        assertEq(START_TOKENS - DRONE_STATION_AMOUNT, drone.addr.balance);
        assertEq(START_TOKENS - STATION_LANDLORD_AMOUNT + DRONE_STATION_AMOUNT, station.addr.balance);
        assertEq(START_TOKENS + STATION_LANDLORD_AMOUNT, landlord.addr.balance);

        /*
          Check situation when contract doens't have enough tokens to transfer to station and landlord.
        */
        vm.prank(station.addr);
        gcContract.stationLanding{value: STATION_LANDLORD_AMOUNT}(drone.addr, payable(landlord.addr));
        // Set contract balance to zero.
        vm.deal(address(gcContract), 0);
        vm.prank(drone.addr);
        vm.expectRevert("failed to send ether");
        gcContract.droneLanding{value: DRONE_STATION_AMOUNT}(payable(station.addr));

        /*
          Check fallback function.
        */
        vm.prank(station.addr);
        (bool sent,) = address(gcContract).call{value: 1 ether}("asd");
        assertTrue(sent);
    }
}
