# lit-blockchain

Lit - Blockchain related crate.

## Updating ABIs

```shell
make update
```

## Adding a new contract

First make sure you have checked out the `lit-os` repo as well as `LitNodeContracts` and they are
up to date (`git pull`).

```shell
make update
```

### Add contract key

Modify `blockchain/contracts/contracts/lit-os/ContractResolver.sol` and add a constant:

```solidity
bytes32 public constant ALLOWLIST_CONTRACT = keccak256("ALLOWLIST");
```

### Add contract constructors

Modify `src/contracts/mod.rs` and add a constant to the top:

```rust
pub const ALLOWLIST_CONTRACT: &'static str = "ALLOWLIST";
```

Then add the constructors towards the bottom:

```rust
// Allowlist

impl Allowlist<Provider<Http>> {
    pub(crate) fn load(cfg: &LitConfig, address: H160) -> Result<Allowlist<Provider<Http>>> {
        Ok(Allowlist::new(address, default_local_client_no_wallet(cfg)?))
    }
}

impl Allowlist<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>> {
    pub(crate) fn load_with_signer(
        cfg: &LitConfig, address: H160, wallet_key: Option<&str>,
    ) -> Result<Allowlist<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
        Ok(Allowlist::new(address, default_local_client(cfg, wallet_key)?))
    }
}
```

### Add the contract resolver accessors

Modify `src/resolver/contract/mod.rs` and add (at the bottom of the existing similar definitions):

```rust
// Allowlist

pub async fn allowlist_contract(&self, cfg: &LitConfig) -> Result<Allowlist<Provider<Http>>> {
    Allowlist::load(cfg, self.resolve(cfg, ALLOWLIST_CONTRACT).await?.address().clone())
}

pub async fn allowlist_contract_with_signer(
    &self, cfg: &LitConfig,
) -> Result<Allowlist<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>> {
    Allowlist::load_with_signer(
        cfg,
        self.resolve(cfg, ALLOWLIST_CONTRACT).await?.address().clone(),
        self.wallet_key_as_str(),
    )
}
```