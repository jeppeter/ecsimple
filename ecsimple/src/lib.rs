//!    simple implementation for Elliptic Curve Cryptography
//!    to sign and verify
//!    ```rust
//!    use ecsimple::keys::*;
//!    use ecsimple::group::*;
//!    use ecsimple::signature::*;
//!    
//!    use std::error::Error;
//!    
//!    fn main() -> Result<(),Box<dyn Error>> {
//!    	let grp :ECGroup = ecc_get_curve_group("prime256v1")?;
//!        let privkey :ECPrivateKey = ECPrivateKey::generate(&grp);
//!        let pubkey :ECPublicKey = privkey.export_pubkey();
//!        let signhash :Vec<u8> = vec![10,11,12,13,14];
//!        let sig :ECSignature = privkey.sign_base(&signhash)?;
//!        let retval :bool = pubkey.verify_base(&sig,&signhash)?;
//!        assert!(retval == true);
//!        Ok(())
//!    }
//!    
//!    ```
//!    to use from_der and to_der
//!    ```rust
//!    use ecsimple::keys::*;
//!    use ecsimple::group::*;
//!    
//!    use std::error::Error;
//!    
//!    fn main() -> Result<(),Box<dyn Error>> {
//!    	let grp :ECGroup = ecc_get_curve_group("prime256v1")?;
//!        let privkey :ECPrivateKey = ECPrivateKey::generate(&grp);
//!        let pubkey :ECPublicKey = privkey.export_pubkey();
//!        let privdata :Vec<u8> = privkey.to_der("compressed","")?;
//!        let pubdata  :Vec<u8> = pubkey.to_der("compressed","explicit")?;
//!        let _nprivkey :ECPrivateKey = ECPrivateKey::from_der(&privdata)?;
//!        let _npubkey :ECPublicKey = ECPublicKey::from_der(&pubdata)?;
//!        Ok(())
//!    }
//!    
//!    ```

#[allow(dead_code)]
#[allow(non_upper_case_globals)]
pub mod consts;
#[macro_use]
pub mod errors;
#[macro_use]
pub mod logger;
pub (crate) mod fileop;
pub mod randop;
//pub (crate) mod utils;
pub (crate) mod ecasn1;
pub mod utils;
pub mod mont;
pub mod bngf2m;
pub mod group;
pub mod point;
pub mod curve;
pub mod signature;
pub mod keys;