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
