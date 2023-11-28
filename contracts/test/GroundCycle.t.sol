// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {Contract as GroundCycleContract} from "../src/GroundCycle.sol";

contract GroundCycleContractTest is Test {
    GroundCycleContract public gcContract;

    function setUp() public {
        gcContract = new GroundCycleContract();
    }
}
