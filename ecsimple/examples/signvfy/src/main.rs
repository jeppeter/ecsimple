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
