use ecsimple::keys::{PrivateKey,PublicKey};
use ecsimple::consts::*;
use ecsimple::signature::{ECCSignature};
use ecsimple::{ecsimple_error_class,ecsimple_new_error};
use ecsimple::curves::{get_ecc_curve_by_name};

use std::error::Error;
use sha1::{Sha1,Digest};
use std::io::{Write,Read};

ecsimple_error_class!{FileOpError}


pub fn write_file_bytes(fname :&str, byts :&[u8]) -> Result<(),Box<dyn Error>> {
	if fname.len() == 0 {
		let res = std::io::stdout().write_all(byts);
		if res.is_err() {
			let err = res.err().unwrap();
			ecsimple_new_error!{FileOpError,"write [stdout] len[{}] error[{:?}]", byts.len(),err}	
		}
	} else {
		let fo  = std::fs::File::create(fname);
		if fo.is_err() {
			let err = fo.err().unwrap();
			ecsimple_new_error!{FileOpError,"create [{}] error[{:?}]", fname,err}
		}
		let mut fp : std::fs::File = fo.unwrap();
		let res = fp.write_all(byts);
		if res.is_err() {
			let err = res.err().unwrap();
			ecsimple_new_error!{FileOpError,"write [{}] len[{}] error[{:?}]", fname, byts.len(),err}	
		}
	}
	Ok(())
}


pub fn read_file_bytes(fname :&str) -> Result<Vec<u8>,Box<dyn Error>> {
	if fname.len() == 0 {
		let f = std::io::stdin();
		let mut reader = std::io::BufReader::new(f);
		let mut buf :Vec<u8> = Vec::new();
		let res = reader.read_to_end(&mut buf);
		if res.is_err() {
			let err = res.err().unwrap();
			ecsimple_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}
		Ok(buf)
	} else {
		let fo = std::fs::File::open(fname);
		if fo.is_err() {
			let err = fo.err().unwrap();
			ecsimple_new_error!{FileOpError,"can not open [{}] error[{:?}]", fname, err}
		}
		let f = fo.unwrap();
		let mut reader = std::io::BufReader::new(f);
		let mut buf :Vec<u8> = Vec::new();
		let res = reader.read_to_end(&mut buf);
		if res.is_err() {
			let err = res.err().unwrap();
			ecsimple_new_error!{FileOpError,"read [{}] error [{:?}]", fname,err}
		}

		Ok(buf)		
	}
}

fn sha1_calc(data :&[u8]) -> Vec<u8> {
	    let mut hasher = Sha1::new();
	    hasher.update(&data);
	    let res = hasher.finalize();
	    return res.to_vec();    

}

fn main() -> Result<(),Box<dyn Error>> {
	let privkey :PrivateKey;
	let mut ecname :String = SECP224k1_NAME.to_string();
	let mut basefile :String = "examples.txt".to_string();
	let argv :Vec<String> = std::env::args().collect();
	if argv.len() > 2 {
		ecname = format!("{}",argv[2]);
	}
	if argv.len() > 1 {
		if argv[1] == "-h" || argv[1] == "--help" {
			println!("{} [basefile] [ecname]", argv[0]);
			return Ok(());
		}

		basefile = format!("{}",argv[1]);
	}
	let eccurve = get_ecc_curve_by_name(&ecname)?;
	privkey = PrivateKey::generate(&eccurve,None)?;
	let r = read_file_bytes(&basefile)?;
	let digdata = sha1_calc(&r);
	println!("digdata {:?}", digdata);
	loop {
		/*because the sign will be zero so make multiple cases*/
		let ores = privkey.sign_digest(&digdata);
		if ores.is_ok() {
			let sigv :ECCSignature = ores.unwrap();
			let asn1sigv = sigv.to_der()?;
			write_file_bytes("sigv.bin",&asn1sigv)?;
			break;
		}
	}
	let pubk = privkey.get_public_key();
	let pubkder = pubk.to_der(EC_UNCOMPRESSED,EC_SSLEAY_TYPE)?;

	write_file_bytes("pubk.bin",&pubkder)?;

	let vdata = read_file_bytes("sigv.bin")?;
	let pubdata = read_file_bytes("pubk.bin")?;
	let pubkey :PublicKey = PublicKey::from_der(&pubdata)?;
	let sigvret :ECCSignature = ECCSignature::from_der(&vdata)?;
	let bval = pubkey.verify_digest(&digdata,&sigvret);
	println!("sign verfiy {:?}", bval);
	println!("privkey {:?}", privkey);
	println!("pubkey {:?}", pubkey);
	Ok(())
}
