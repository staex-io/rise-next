// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import {AgreementContract} from "../src/Agreement.sol";
import {GroundCycleContract} from "../src/GroundCycle.sol";

contract GroundCycleScript is Script {
    function run() external {
        uint256 deployerPrivateKey = vm.envUint("PRIVATE_KEY");
        vm.startBroadcast(deployerPrivateKey);
        AgreementContract agreement = new AgreementContract();
        GroundCycleContract groundCycle = new GroundCycleContract(agreement);
        vm.stopBroadcast();
    }
}
