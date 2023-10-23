use ecsimple::keys::*;
use ecsimple::group::*;
use ecsimple::signature::*;

use std::error::Error;

use sha1::{Sha1,Digest};

fn get_hash_value(data :&[u8]) -> Vec<u8> {
	let mut hasher = Sha1::new();
	hasher.update(data);
	return hasher.finalize().to_vec();
}

/*
	this is command like
	openssl ecparam -genkey -name prime256v1 -noout -out ecpriv.pem
	openssl ec -in ecpriv.pem -pubout -out ecpub.pem
	openssl dgst -sha1 -sign ecpriv.pem -out sig.bin realdata.bin
	openssl dgst -sha1 -verify ecpub.pem realdata.bin
*/
fn main() -> Result<(),Box<dyn Error>> {
	let grp :ECGroup = ecc_get_curve_group("prime256v1")?;
    let privkey :ECPrivateKey = ECPrivateKey::generate(&grp);
    let pubkey :ECPublicKey = privkey.export_pubkey();
    let realdata :Vec<u8> = vec![0x11,0x22,0x33];
    let signhash :Vec<u8> = get_hash_value(&realdata);
    let sig :ECSignature = privkey.sign_base(&signhash)?;
    let retval :bool = pubkey.verify_base(&sig,&signhash)?;
    /*
    	privdata = privkey.to_der("compressed","")?;
    	pubdata = pubkey.to_der("compressed","")?;
    	sigdata = sig.encode_asn1()?;

    */
    assert!(retval == true);
    Ok(())
}
