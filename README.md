## Getting started

1. First, you need to create the descriptors which you can use later for loading up the wallet.

```rust
Descriptor::new();
```
2. then use the console output for loading up the wallet. Example:
```rust
let descriptor_str: &str = "tr([a2f8ef2c/86'/1'/0']tpubDDabhdF9v5e4zXeCkhsczu1cD2PLR6mDwDeKEqq4XkrHasQRSvLXDXAngZ15vc7vhJiippdKb5ZUnVmo7zknkHj1zqvddS8q6j2uEerJ2L1/0/*)#6zkee8fx";
let change_descriptor_str: &str = "tr([a2f8ef2c/86'/1'/0']tpubDDabhdF9v5e4zXeCkhsczu1cD2PLR6mDwDeKEqq4XkrHasQRSvLXDXAngZ15vc7vhJiippdKb5ZUnVmo7zknkHj1zqvddS8q6j2uEerJ2L1/1/*)#tkncyje7";
```
3. You should also see the wallet address. 
Example: `tb1pyhh43adldc5ksl7hr9ax7fxulskaedg2srsmra9zdmlgvcmuku0s07675w`

4. Fund this address with some coins from the [faucet](https://signet257.bublina.eu.org/)

5. Run the program again and see if the balance was updated.

6. Send the funds back to the faucet which address is `tb1p4tp4l6glyr2gs94neqcpr5gha7344nfyznfkc8szkreflscsdkgqsdent4`.
In order to do that, you need to use private keys instead as the descriptors: `tr(tprv...`

