# ecsimple
> rust Elliptic Curve Cryptography implementation

### Release History
* Oct 20th 2023 release 0.1.0 for first test cases

### simple example
```rust
use ecsimple::keys::*;
use ecsimple::group::*;
use ecsimple::signature::*;

use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
	let grp :ECGroup = ecc_get_curve_group("prime256v1")?;
    let privkey :ECPrivateKey = ECPrivateKey::generate(&grp);
    let pubkey :ECPublicKey = privkey.export_pubkey();
    let signhash :Vec<u8> = vec![10,11,12,13,14];
    let sig :ECSignature = privkey.sign_base(&signhash)?;
    let retval :bool = pubkey.verify_base(&sig,&signhash)?;
    assert!(retval == true);
    Ok(())
}

```

### from and to der
```rust
use ecsimple::keys::*;
use ecsimple::group::*;

use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
	let grp :ECGroup = ecc_get_curve_group("prime256v1")?;
    let privkey :ECPrivateKey = ECPrivateKey::generate(&grp);
    let pubkey :ECPublicKey = privkey.export_pubkey();
    let privdata :Vec<u8> = privkey.to_der("compressed","")?;
    let pubdata  :Vec<u8> = pubkey.to_der("compressed","explicit")?;
    let _nprivkey :ECPrivateKey = ECPrivateKey::from_der(&privdata)?;
    let _npubkey :ECPublicKey = ECPublicKey::from_der(&pubdata)?;
    Ok(())
}

```
