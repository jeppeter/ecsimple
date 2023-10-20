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

### now supported types
```shell
SM2                     brainpoolP160r1         brainpoolP160t1         brainpoolP192r1         brainpoolP192t1        
brainpoolP224r1         brainpoolP224t1         brainpoolP256r1         brainpoolP256t1         brainpoolP320r1        
brainpoolP320t1         brainpoolP384r1         brainpoolP384t1         brainpoolP512r1         brainpoolP512t1        
c2pnb163v1              c2pnb163v2              c2pnb163v3              c2pnb176v1              c2pnb208w1             
c2pnb272w1              c2pnb304w1              c2pnb368w1              c2tnb191v1              c2tnb191v2             
c2tnb191v3              c2tnb239v1              c2tnb239v2              c2tnb239v3              c2tnb359v1             
c2tnb431r1              prime192v1              prime192v2              prime192v3              prime239v1             
prime239v2              prime239v3              prime256v1              secp112r1               secp112r2              
secp128r1               secp128r2               secp160k1               secp160r1               secp160r2              
secp192k1               secp224k1               secp224r1               secp256k1               secp384r1              
secp521r1               sect113r1               sect113r2               sect131r1               sect131r2              
sect163k1               sect163r1               sect163r2               sect193r1               sect193r2              
sect233k1               sect233r1               sect239k1               sect283k1               sect283r1              
sect409k1               sect409r1               sect571k1               sect571r1               wap-wsg-idm-ecid-wtls1 
wap-wsg-idm-ecid-wtls12 wap-wsg-idm-ecid-wtls8  wap-wsg-idm-ecid-wtls9 
```