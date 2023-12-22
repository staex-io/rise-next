// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import {AgreementContract} from "../src/Agreement.sol";
import {GroundCycleContract} from "../src/GroundCycle.sol";

contract GroundCycleScript is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        // Landing wait time should be present in seconds.
        uint256 landingWaitTime = vm.envUint("LANDING_WAIT_TIME");
        require(landingWaitTime >= 5 minutes, "landing wait time cannot be less than 5 minutes");
        require(landingWaitTime <= 10 minutes, "landing wait time cannot be more than 10 minutes");
        vm.startBroadcast(deployerPrivateKey);
        AgreementContract agreement = new AgreementContract();
        new GroundCycleContract(landingWaitTime, agreement);
        vm.stopBroadcast();
    }
}
