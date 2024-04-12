// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.22;

import "forge-std/Test.sol";
import {
    GroundCycleContract,
    Info,
    ErrNoApprovedLanding,
    ErrReceivedNotEnough,
    ErrAgreementNotSigned,
    ErrNoLanding,
    ErrRejectApprovedLanding,
    ErrTakeoffRequired,
    ErrRejectTooEarly,
    ErrHandshake
} from "../src/GroundCycle.sol";
import {AgreementContract, ErrNoAgreement} from "../src/Agreement.sol";
import "forge-std/Vm.sol";
import {VmSafe} from "forge-std/Vm.sol";

contract GroundCycleTest is Test {
    string constant DRONE_WALLET_NAME = "drone";
    string constant FAKE_DRONE_WALLET_NAME = "fake_drone";
    string constant STATION_WALLET_NAME = "station";
    string constant LANDLORD_WALLET_NAME = "landlord";
    uint256 constant START_TOKENS = 100 ether;
    uint256 constant DRONE_STATION_AMOUNT = 3 ether;
    uint256 constant STATION_LANDLORD_AMOUNT = 5 ether;

    GroundCycleContract public groundCycleContract;
    AgreementContract public agreementContract;

    // We need to copy that events from contract to avoid visibility errors.
    event Landing(uint256, address indexed, address indexed, address indexed);
    event Takeoff(uint256, address indexed, address indexed, address indexed);
    event Reject(address indexed, address indexed);

    function setUp() public {
        agreementContract = new AgreementContract();
        groundCycleContract = new GroundCycleContract(300, agreementContract);
    }

    function test_All() public {
        // Create wallets for entities.
        VmSafe.Wallet memory drone = vm.createWallet(DRONE_WALLET_NAME);
        VmSafe.Wallet memory fakeDrone = vm.createWallet(FAKE_DRONE_WALLET_NAME);
        VmSafe.Wallet memory station = vm.createWallet(STATION_WALLET_NAME);
        VmSafe.Wallet memory landlord = vm.createWallet(LANDLORD_WALLET_NAME);
        // Set balances.
        vm.deal(drone.addr, START_TOKENS);
        vm.deal(fakeDrone.addr, START_TOKENS);
        vm.deal(station.addr, START_TOKENS);
        vm.deal(landlord.addr, START_TOKENS);

        /*
          Check no agreements logic for drone and for station.
        */
        // Check for drone.
        vm.prank(drone.addr);
        vm.expectRevert(ErrNoAgreement.selector);
        groundCycleContract.landingByDrone{value: 1 ether}(payable(station.addr));
        // Check for station.
        vm.prank(station.addr);
        vm.expectRevert(ErrNoAgreement.selector);
        groundCycleContract.landingByStation{value: 1 ether}(payable(drone.addr), payable(landlord.addr));

        // Initiate agreements between entities.
        vm.startPrank(station.addr);
        agreementContract.create(drone.addr, DRONE_STATION_AMOUNT);
        agreementContract.create(landlord.addr, STATION_LANDLORD_AMOUNT);
        vm.stopPrank();

        /*
          Check landing with not signed agreement.
        */
        // Check for drone.
        vm.prank(drone.addr);
        vm.expectRevert(ErrAgreementNotSigned.selector);
        groundCycleContract.landingByDrone{value: 1 ether}(payable(station.addr));
        // Check for station.
        vm.prank(station.addr);
        vm.expectRevert(ErrAgreementNotSigned.selector);
        groundCycleContract.landingByStation{value: 1 ether}(payable(drone.addr), payable(landlord.addr));

        vm.prank(drone.addr);
        agreementContract.sign(station.addr, DRONE_STATION_AMOUNT);
        vm.prank(landlord.addr);
        agreementContract.sign(station.addr, STATION_LANDLORD_AMOUNT);

        /* 
          Create and sign an agreemnt between fake drone and station.
        */
        vm.prank(station.addr);
        agreementContract.create(fakeDrone.addr, DRONE_STATION_AMOUNT);
        vm.prank(fakeDrone.addr);
        agreementContract.sign(station.addr, DRONE_STATION_AMOUNT);

        /*
          Check landing rejection.
        */
        // Check rejection no landing.
        vm.prank(drone.addr);
        vm.expectRevert(ErrNoLanding.selector);
        groundCycleContract.reject(station.addr);
        // Check reject too early.
        vm.prank(drone.addr);
        groundCycleContract.landingByDrone{value: DRONE_STATION_AMOUNT}(payable(station.addr));
        assertEq(drone.addr.balance, START_TOKENS - DRONE_STATION_AMOUNT);
        vm.prank(drone.addr);
        vm.expectRevert(ErrRejectTooEarly.selector);
        groundCycleContract.reject(station.addr);
        assertEq(drone.addr.balance, START_TOKENS - DRONE_STATION_AMOUNT);
        // Check successfull rejection.
        vm.prank(drone.addr);
        vm.warp(block.timestamp + 5 minutes);
        vm.expectEmit(true, true, false, false);
        emit Reject(drone.addr, station.addr);
        groundCycleContract.reject(station.addr);
        assertEq(drone.addr.balance, START_TOKENS);
        // Check that there are no landing entry after rejection.
        vm.prank(drone.addr);
        Info memory landing = groundCycleContract.get(station.addr);
        assertEq(address(0), landing.drone);

        /*
          Check sent not enough tokens logic for drone and station.
        */
        bytes4 errRecvNotEnoughSelector = bytes4(keccak256("ErrReceivedNotEnough(uint256,uint256)"));
        // Check for drone.
        vm.prank(drone.addr);
        vm.expectRevert(abi.encodeWithSelector(errRecvNotEnoughSelector, 1 ether, DRONE_STATION_AMOUNT));
        groundCycleContract.landingByDrone{value: 1 ether}(payable(station.addr));
        // Check for station.
        vm.prank(station.addr);
        vm.expectRevert(abi.encodeWithSelector(errRecvNotEnoughSelector, 1 ether, STATION_LANDLORD_AMOUNT));
        groundCycleContract.landingByStation{value: 1 ether}(payable(drone.addr), payable(landlord.addr));

        /*
          Check that drone can send their transaction first and after that station.
        */
        // Check that drone can execute their landing method.
        vm.prank(drone.addr);
        groundCycleContract.landingByDrone{value: DRONE_STATION_AMOUNT}(payable(station.addr));
        assertEq(groundCycleContract.getBalance(), DRONE_STATION_AMOUNT);
        Info memory afterDroneLanding_1 = groundCycleContract.get(station.addr);
        assertEq(0, afterDroneLanding_1.id);
        assertEq(drone.addr, afterDroneLanding_1.drone);
        assertEq(station.addr, afterDroneLanding_1.station);
        assertEq(address(0), afterDroneLanding_1.landlord);
        // Check that drone can execute landing twice.
        vm.prank(drone.addr);
        groundCycleContract.landingByDrone{value: DRONE_STATION_AMOUNT}(payable(station.addr));
        // Check that station cannot land drone which is not landed to it.
        vm.prank(station.addr);
        vm.expectRevert(ErrHandshake.selector);
        groundCycleContract.landingByStation{value: STATION_LANDLORD_AMOUNT}(
            payable(fakeDrone.addr), payable(landlord.addr)
        );
        // Check that station can execute their landing method and landing should be approved.
        vm.prank(station.addr);
        vm.expectEmit(true, true, true, true);
        emit Landing(1, drone.addr, station.addr, landlord.addr);
        groundCycleContract.landingByStation{value: STATION_LANDLORD_AMOUNT}(
            payable(drone.addr), payable(landlord.addr)
        );
        // Now we have balance 0 because we send tokens from smart contract to station and to landlord.
        assertEq(groundCycleContract.getBalance(), 0);
        // Check that balances were updated.
        assertEq(START_TOKENS - DRONE_STATION_AMOUNT, drone.addr.balance);
        assertEq(START_TOKENS - STATION_LANDLORD_AMOUNT + DRONE_STATION_AMOUNT, station.addr.balance);
        assertEq(START_TOKENS + STATION_LANDLORD_AMOUNT, landlord.addr.balance);
        Info memory afterStationLanding_1 = groundCycleContract.get(station.addr);
        // Now we have an id as landing was approved.
        assertEq(1, afterStationLanding_1.id);
        assertEq(drone.addr, afterStationLanding_1.drone);
        assertEq(station.addr, afterStationLanding_1.station);
        assertEq(landlord.addr, afterStationLanding_1.landlord);

        // Check that we can't reject approved landing.
        vm.prank(drone.addr);
        vm.expectRevert(ErrRejectApprovedLanding.selector);
        groundCycleContract.reject(station.addr);

        // Check that we can't execute landing twice without a takeoff.
        vm.prank(drone.addr);
        vm.expectRevert(ErrTakeoffRequired.selector);
        groundCycleContract.landingByDrone{value: DRONE_STATION_AMOUNT}(payable(station.addr));

        /*
         Test takeoff
        */
        // Test takeoff if there is no active landing.
        vm.prank(address(777));
        vm.expectRevert(ErrNoApprovedLanding.selector);
        groundCycleContract.takeoff();
        // Test real and ok takeoff.
        vm.prank(station.addr);
        vm.expectEmit(true, true, true, true);
        emit Takeoff(1, drone.addr, station.addr, landlord.addr);
        groundCycleContract.takeoff();

        // Revert balances for future tests.
        vm.deal(drone.addr, START_TOKENS);
        vm.deal(station.addr, START_TOKENS);
        vm.deal(landlord.addr, START_TOKENS);

        // Check that station can reject landing.
        vm.prank(station.addr);
        groundCycleContract.landingByStation{value: STATION_LANDLORD_AMOUNT}(
            payable(drone.addr), payable(landlord.addr)
        );
        vm.prank(station.addr);
        vm.warp(block.timestamp + 5 minutes);
        vm.expectEmit(true, true, false, false);
        emit Reject(drone.addr, station.addr);
        groundCycleContract.reject(station.addr);

        // Check that landlord cannot execute landing rejection from drone.
        vm.prank(drone.addr);
        groundCycleContract.landingByDrone{value: DRONE_STATION_AMOUNT}(payable(station.addr));
        vm.prank(landlord.addr);
        vm.warp(block.timestamp + 5 minutes);
        vm.expectRevert("caller should be drone of this landing");
        groundCycleContract.reject(station.addr);
        vm.prank(drone.addr);
        vm.warp(block.timestamp + 5 minutes);
        groundCycleContract.reject(station.addr);
        // Check that landlord cannot execute landing rejection from station.
        vm.prank(station.addr);
        groundCycleContract.landingByStation{value: STATION_LANDLORD_AMOUNT}(
            payable(drone.addr), payable(landlord.addr)
        );
        vm.prank(landlord.addr);
        vm.warp(block.timestamp + 5 minutes);
        vm.expectRevert("caller should be station of this landing");
        groundCycleContract.reject(station.addr);
        vm.prank(station.addr);
        vm.warp(block.timestamp + 5 minutes);
        groundCycleContract.reject(station.addr);

        /*
          Check that station can send their transaction first and after that drone.
        */
        // Check that station can execute their landing method.
        vm.prank(station.addr);
        groundCycleContract.landingByStation{value: STATION_LANDLORD_AMOUNT}(
            payable(drone.addr), payable(landlord.addr)
        );
        assertEq(groundCycleContract.getBalance(), STATION_LANDLORD_AMOUNT);
        Info memory afterStationLanding_2 = groundCycleContract.get(station.addr);
        assertEq(0, afterStationLanding_2.id);
        assertEq(drone.addr, afterStationLanding_2.drone);
        assertEq(station.addr, afterStationLanding_2.station);
        assertEq(landlord.addr, afterStationLanding_2.landlord);
        // Check that station can execute landing twice.
        vm.prank(station.addr);
        groundCycleContract.landingByStation{value: STATION_LANDLORD_AMOUNT}(
            payable(drone.addr), payable(landlord.addr)
        );
        // Check that fake drone cannot execute their landing method and landing is not approved.
        vm.prank(fakeDrone.addr);
        vm.expectRevert(ErrHandshake.selector);
        groundCycleContract.landingByDrone{value: DRONE_STATION_AMOUNT}(payable(station.addr));
        // Check that drone can execute their landing method and landing should be approved.
        vm.prank(drone.addr);
        vm.expectEmit(true, true, true, true);
        emit Landing(2, drone.addr, station.addr, landlord.addr);
        groundCycleContract.landingByDrone{value: DRONE_STATION_AMOUNT}(payable(station.addr));
        // Now we have balance 0 because we send tokens from smart contract to station and to landlord.
        assertEq(groundCycleContract.getBalance(), 0);
        // Check that balances were updated.
        assertEq(START_TOKENS - DRONE_STATION_AMOUNT, drone.addr.balance);
        assertEq(START_TOKENS - STATION_LANDLORD_AMOUNT + DRONE_STATION_AMOUNT, station.addr.balance);
        assertEq(START_TOKENS + STATION_LANDLORD_AMOUNT, landlord.addr.balance);
        Info memory afterDroneLanding_2 = groundCycleContract.get(station.addr);
        // Now we have an id as landing was approved.
        assertEq(2, afterDroneLanding_2.id);
        assertEq(drone.addr, afterDroneLanding_2.drone);
        assertEq(station.addr, afterDroneLanding_2.station);
        assertEq(landlord.addr, afterDroneLanding_2.landlord);

        // Check that we can't execute landing twice without a takeoff.
        vm.prank(station.addr);
        vm.expectRevert(ErrTakeoffRequired.selector);
        groundCycleContract.landingByStation{value: STATION_LANDLORD_AMOUNT}(
            payable(drone.addr), payable(landlord.addr)
        );

        // Make takeoff for future tests.
        vm.prank(station.addr);
        groundCycleContract.takeoff();

        /*
          Check situation when contract doens't have enough tokens to transfer to station and landlord.
        */
        vm.prank(station.addr);
        groundCycleContract.landingByStation{value: STATION_LANDLORD_AMOUNT}(
            payable(drone.addr), payable(landlord.addr)
        );
        // Set contract balance to zero.
        vm.deal(address(groundCycleContract), 0);
        vm.prank(drone.addr);
        vm.expectRevert("failed to send ether");
        groundCycleContract.landingByDrone{value: DRONE_STATION_AMOUNT}(payable(station.addr));

        /*
          Check fallback function.
        */
        vm.prank(station.addr);
        (bool sent,) = address(groundCycleContract).call{value: 1 ether}("asd");
        assertTrue(sent);
    }
}
