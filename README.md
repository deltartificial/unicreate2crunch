# unicreate2crunch

> A Rust program for mining salts that generate high-scoring, gas-efficient Ethereum addresses for Uniswap v4 deployment.

This program mines salts using the CREATE2 function to produce Ethereum addresses that score above 100 points according to Uniswap's v4 scoring system. It leverages Uniswap's unique criteria, rewarding addresses with specific patterns of zeros and the prominent use of the number 4. By using this tool, participants can uncover candidate addresses optimized for gas efficiency and high scores, contributing to the Uniswap v4 Address Mining Challenge and potentially creating the official deployment address.

Scoring System: This program integrates Uniswap’s scoring formula:

- 10 points for each leading 0 nibble
- 40 points if the address starts with four consecutive 4s
- 20 points if the first nibble after the four 4s is NOT a 4
- 20 points if the last four nibbles are all 4s
- 1 point for each additional 4 elsewhere in the address

Provide three arguments: a factory address (or contract that will call CREATE2), a caller address (for factory addresses that require it as a protection against frontrunning), and the keccak-256 hash of the initialization code of the contract that the factory will deploy.
(The example below references the `Create2Factory`'s address on one of the 21 chains where it has been deployed to.)

Live `Create2Factory` contracts can be found [here](https://blockscan.com/address/0x0000000000ffe8b47b3e2130213b802212439497).

```sh
$ git clone https://github.com/0age/create2crunch
$ cd create2crunch
$ export FACTORY="0x0000000000ffe8b47b3e2130213b802212439497"
$ export CALLER="0x48E516B34A1274f49457b9C6182097796D0498Cb"
$ export INIT_CODE_HASH="0x94d114296a5af85c1fd2dc039cdaa32f1ed4b0fe0868f02d888bfc91feb645d9"
$ cargo run --release $FACTORY $CALLER $INIT_CODE_HASH
```

---

For each efficient address found, the salt, resultant addresses, and value _(i.e. approximate rarity)_ will be written to `efficient_addresses.txt`. Verify that one of the salts actually results in the intended address before getting in too deep - ideally, the CREATE2 factory will have a view method for checking what address you'll get for submitting a particular salt. Be sure not to change the factory address or the init code without first removing any existing data to prevent the two salt types from becoming commingled. There's also a _very_ simple monitoring tool available if you run `$python3 analysis.py` in another tab.

This tool was originally built for use with [`Pr000xy`](https://github.com/0age/Pr000xy), including with [`Create2Factory`](https://github.com/0age/Pr000xy/blob/master/contracts/Create2Factory.sol) directly.

There is also an experimental OpenCL feature that can be used to search for addresses using a GPU. To give it a try, include a fourth parameter specifying the device ID to use, and optionally a fifth and sixth parameter to filter returned results by a threshold based on leading zero bytes and total zero bytes, respectively. By way of example, to perform the same search as above, but using OpenCL device 2 and only returning results that create addresses with at least four leading zeroes or six total zeroes, use `$ cargo run --release $FACTORY $CALLER $INIT_CODE_HASH 2 4 6` (you'll also probably want to try tweaking the `WORK_SIZE` parameter in `src/lib.rs`).

:)

```sh
cargo run --bin list_devices
```

PRs welcome!
