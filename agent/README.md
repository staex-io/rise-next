# Agent

## Tests

For steps 1-3 see project root README.

1. `cd` to root project folder
2. Setup .env file as described in main [README](../README.md)
3. Run anvil node from root project folder: `make anvil`
4. Deploy smart contract: `make deploy`
5. `cd agent` and `make test`

## Examples

With default Anvil values.

```shell
# Landing by drone.
cargo run -- landing-by-drone 0x4bbbf85ce3377467afe5d46f804f221813b2bb87f24d81f60f1fcdbf7cbf4356 0x23618e81E3f5cdF7f54C3d65f7FBc0aBf5B21E8f
```

```shell
# Landing by station.
cargo run -- landing-by-station 0xdbda1821b80551c9d65939329250298aa3472ba22feea921c0cf5d620ea67b97 0x14dC79964da2C08b23698B3D3cc7Ca32193d9955 0xa0Ee7A142d267C1f36714E4a8F75612F20a79720
```
