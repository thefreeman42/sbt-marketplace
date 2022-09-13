Call consumer to call request

```
near call dev-1663022713091-98553793614396 request '{"account_id": "tituszban.testnet", "public_key": "ed25519:4FrM8JRiAqWnnmWJD5zTLkz5AveqVrv5eKnhftegR3HU", "message": "some message"}' --accountId tituszban.testnet --gas 300000000000000
```

Call oracle

```
cd sbt-marketplace-oracle
python -m oracle_caller tituszban.testnet [private_key]
```