# Docs

## Current implementation

Currently we use: Foundry, Solidity, Ethereum, Rust, Vue.

### Smart contracts

#### Agreement

To update agreement statement (to which account should be mae fee transfers for landing and how much) you need to create execute "create" and "sign" methods. Creating should be made on behalf of station as station is a participant in any agreement in our system. Signing should be on behalf of the drone or landlord.

While signing it is required to provide agreement amount to avoid case when station create another agreement with different amount before signing and after sending for signing to another entity.

Creation and signing produce Ethereum events.

#### GroundCycle

To start landing process drone should scan qr code from station and vice-versa. After that drone can execute "landingByDrone" method. Station should execute "landingByStation". When two methods were executed landing is approved and smart contract makes transfers to station and landlord as fee for landing.

Also station has a possibility to execute "takeoff" method to make drone takeoff.

Approved landing and takeoff produce Ethereum event.

### Agent.

Agent has a test to make integration testing of the entire system.

Also agent has a software to scan qr codes and send transactions to the network.
