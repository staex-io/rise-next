// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import {AgreementContract} from "../src/Agreement.sol";
import {GroundCycleContract} from "../src/GroundCycle.sol";
import "forge-std/console.sol";

contract GroundCycleScript is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        // Landing wait time should be present in seconds.
        uint256 landingWaitTime = vm.envUint("LANDING_WAIT_TIME");
        bool isTesting = vm.envBool("IS_TESTING");
        if (!isTesting) {
            require(landingWaitTime >= 5 minutes, "landing wait time cannot be less than 5 minutes");
            require(landingWaitTime <= 10 minutes, "landing wait time cannot be more than 10 minutes");
        }
        console.log("Landing wait time is:", landingWaitTime, "seconds");
        vm.startBroadcast(deployerPrivateKey);
        AgreementContract agreement = new AgreementContract();
        new GroundCycleContract(landingWaitTime, agreement);
        vm.stopBroadcast();
    }
}
