# Plasma Cash

```sh
$ ganache-cli --mnemonic 'candy maple cake sugar pudding cream honey rich smooth crumble sweet treaf'
$ truffle migrate
```

### Run child chain

```sh
$ cd child_chain
$ cargo run
```

### Deposit

```sh
truffle(development)> let accounts = await web3.eth.getAccounts()
truffle(development)> let rootChain = await RootChain.deployed()
truffle(development)> rootChain.deposit(1, {from: accounts[0], value: web3.utils.toWei("1", "ether")})
```
