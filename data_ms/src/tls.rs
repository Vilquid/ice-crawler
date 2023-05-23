use std::io::prelude::*;
use std::net::{TcpStream,Shutdown};
use std::thread;
use std::time::Duration;
use std::sync::mpsc;
use std::convert::TryInto;
use std::process::Termination;
use std::process::ExitCode;
use serde::Serialize;


pub struct Renvoi<'a> {
	repserv: [u8;10000],
	suite: &'a str,
	tlstab: [&'a str;5],
	extinction: &'a str,
}

struct Rapport {
	liste: Vec<String>,
	reserv: [u8;10000],
}

#[derive(Debug, Serialize)]
pub struct Retour{
	#[serde(with = "serde_json::json")]
	certificat: String,
	liste: Vec<String>,
	cyfaible: String,
	starttls: String,
	versions: [String;4],
	#[serde(serialize_with = "serialize_f32_without_quotes")]
	note: u16,
	ip: String,
}

impl Termination for Retour {
	fn report(self) -> ExitCode {
        match self {
        	_ => todo!(),
        }
    }
}

fn serialize_u16_without_quotes<S>(value: &u16, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

///le main prends en argument une addresse ip au format string, effectue une première connection toute simple pour vérifier si starttls est présent
///si ce n'est pas le cas il s'arrête et renvoie une note de 0. dans l'autre cas il lance pleins de threads qui seront chargés chacun de tester une suite
///cryptographique et, si la suite est prise en charge, récupèrent les versions tls prises en charge
///suite à ça le programme principal récupère les données et récupère le certificat CA ainsi que les extensions avant de calculer une note sur 1404 et de renvoyer
///il renvoie une structure de type Retour





pub(crate) fn tls(mut add: String, domain: String) -> Retour {
	const LEGROS:u64=65536;
	const LEPETIT:u64=256;
	const LAPS:u64=17;
		
	let mut ipa=add.clone();
		

	add=add+":25";
	let mut pol=String::new();
	// let domain = domain;
	let mut vec: Vec<String> = vec![pol.clone(); 600];
	let init="test";
	let mut fin=Retour{certificat: String::from(""), liste: vec.clone(), cyfaible: String::from(""), starttls: String::from("test"), versions: Default::default(), note: 0, ip: ipa.clone()};
	let mut juj=0;
	let (tx, rx) = mpsc::channel::<Renvoi>();
	pol.push_str("test");
	vec=vec![pol.clone(); 600];
	println!("pol={}",pol);
	let mut resultatsannalyse=vec.clone();
	let mut positionanalyse:usize=0;
	println!("{:?}",domain);
	let mut calcul=String::from("HELO ".to_owned() + &domain + "\r\n");
	let mut warp=TcpStream::connect(&add);
	let mut jaj=1;
	match warp {
		Err(_) => {println!("tentative échouée, reconnection en cours...");}
		Ok(_) => {jaj=0;}
	}
	while jaj != 0 {
		println!("tentative échouée, reconnection en cours...");
		warp=TcpStream::connect(&add);
		match warp {
			Err(_) => {println!("tentative échouée, reconnection en cours...");jaj+=1;}
			Ok(_) => {jaj=0;}
		}
		if jaj >= 10 {
			break;
		}
	}
	println!("mise en place de la cellule d'écoute");
	let mut status=warp.as_ref().expect("un truc").write(calcul.as_bytes());
	match status {
		Err(_) => juj=1,
		Ok(_) => (),
	}
	while juj == 1 {
		status=warp.as_ref().expect("un truc").write(calcul.as_bytes());
		match status {
			Err(_) => juj=1,
			Ok(_) => juj=0,
		}
	}
	warp.as_ref().expect("deux trucs").flush().unwrap();
	println!("message envoyé");
	let mut temp2 = [0;10000];
	let mut rep1=String::new();
	warp.as_ref().expect("reason").read(&mut temp2).unwrap();
	rep1=(&String::from_utf8_lossy(&temp2 [..])).to_string();
	println!("message reçu: {}",rep1);
	temp2 = [0;10000];
	thread::sleep(Duration::from_secs(2));
	calcul=String::from("STARTTLS\r\n");
	rep1=String::from("");
	warp.as_ref().expect("un truc").write(calcul.as_bytes()).unwrap();
	warp.as_ref().expect("deux trucs").flush().unwrap();
	println!("envoi du starttls...");
	warp.as_ref().expect("reason").read(&mut temp2).unwrap();
	rep1=(&String::from_utf8_lossy(&temp2 [..])).to_string();
	println!("message reçu: {}",rep1);
	temp2 = [0;10000];
	thread::sleep(Duration::from_secs(2));
	warp.as_ref().expect("reason").read(&mut temp2).unwrap();
	println!("reponse au starttls {}",&String::from_utf8_lossy(&temp2 [..]));
	let rep2=&String::from_utf8_lossy(&temp2 [0..2]);
	let compare=String::from("220");

	match rep2 {
		compare => {resultatsannalyse[positionanalyse].push_str("STARTTLS supporté");positionanalyse=1;},
		_ => resultatsannalyse[positionanalyse].push_str("pas de starttls: serveur pourri"),
	}
	if positionanalyse==0 {
		println!("{:#?}",resultatsannalyse[0]);
		fin.starttls=resultatsannalyse[0].clone();
		return fin;
	}
	warp.expect("oui").shutdown(Shutdown::Both);
	let reserve1=["test","TLS_NULL_WITH_NULL_NULL","TLS_RSA_WITH_NULL_MD5","TLS_RSA_WITH_NULL_SHA","TLS_RSA_EXPORT_WITH_RC4_40_MD5","TLS_RSA_WITH_RC4_128_MD5","TLS_RSA_WITH_RC4_128_SHA","TLS_RSA_EXPORT_WITH_RC2_CBC_40_MD5","TLS_RSA_WITH_IDEA_CBC_SHA","TLS_RSA_EXPORT_WITH_DES40_CBC_SHA","TLS_RSA_WITH_DES_CBC_SHA","TLS_RSA_WITH_3DES_EDE_CBC_SHA","TLS_DH_DSS_EXPORT_WITH_DES40_CBC_SHA","TLS_DH_DSS_WITH_DES_CBC_SHA","TLS_DH_DSS_WITH_3DES_EDE_CBC_SHA","TLS_DH_RSA_EXPORT_WITH_DES40_CBC_SHA","TLS_DH_RSA_WITH_DES_CBC_SHA","TLS_DH_RSA_WITH_3DES_EDE_CBC_SHA","TLS_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA","TLS_DHE_DSS_WITH_DES_CBC_SHA","TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA","TLS_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA","TLS_DHE_RSA_WITH_DES_CBC_SHA","TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA","TLS_DH_anon_EXPORT_WITH_RC4_40_MD5","TLS_DH_anon_WITH_RC4_128_MD5","TLS_DH_anon_EXPORT_WITH_DES40_CBC_SHA","TLS_DH_anon_WITH_DES_CBC_SHA","TLS_DH_anon_WITH_3DES_EDE_CBC_SHA","TLS_KRB5_WITH_DES_CBC_SHA","TLS_KRB5_WITH_3DES_EDE_CBC_SHA","TLS_KRB5_WITH_RC4_128_SHA","TLS_KRB5_WITH_IDEA_CBC_SHA","TLS_KRB5_WITH_DES_CBC_MD5","TLS_KRB5_WITH_3DES_EDE_CBC_MD5","TLS_KRB5_WITH_RC4_128_MD5","TLS_KRB5_WITH_IDEA_CBC_MD5","TLS_KRB5_EXPORT_WITH_DES_CBC_40_SHA","TLS_KRB5_EXPORT_WITH_RC2_CBC_40_SHA","TLS_KRB5_EXPORT_WITH_RC4_40_SHA","TLS_KRB5_EXPORT_WITH_DES_CBC_40_MD5","TLS_KRB5_EXPORT_WITH_RC2_CBC_40_MD5","TLS_KRB5_EXPORT_WITH_RC4_40_MD5","TLS_PSK_WITH_NULL_SHA","TLS_DHE_PSK_WITH_NULL_SHA","TLS_RSA_PSK_WITH_NULL_SHA","TLS_RSA_WITH_AES_128_CBC_SHA","TLS_DH_DSS_WITH_AES_128_CBC_SHA","TLS_DH_RSA_WITH_AES_128_CBC_SHA","TLS_DHE_DSS_WITH_AES_128_CBC_SHA","TLS_DHE_RSA_WITH_AES_128_CBC_SHA","TLS_DH_anon_WITH_AES_128_CBC_SHA","TLS_RSA_WITH_AES_256_CBC_SHA","TLS_DH_DSS_WITH_AES_256_CBC_SHA","TLS_DH_RSA_WITH_AES_256_CBC_SHA","TLS_DHE_DSS_WITH_AES_256_CBC_SHA","TLS_DHE_RSA_WITH_AES_256_CBC_SHA","TLS_DH_anon_WITH_AES_256_CBC_SHA","TLS_RSA_WITH_NULL_SHA256","TLS_RSA_WITH_AES_128_CBC_SHA256","TLS_RSA_WITH_AES_256_CBC_SHA256","TLS_DH_DSS_WITH_AES_128_CBC_SHA256","TLS_DH_RSA_WITH_AES_128_CBC_SHA256","TLS_DHE_DSS_WITH_AES_128_CBC_SHA256","TLS_RSA_WITH_CAMELLIA_128_CBC_SHA","TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA","TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA","TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA","TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA","TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA","TLS_DHE_RSA_WITH_AES_128_CBC_SHA256","TLS_DH_DSS_WITH_AES_256_CBC_SHA256","TLS_DH_RSA_WITH_AES_256_CBC_SHA256","TLS_DHE_DSS_WITH_AES_256_CBC_SHA256","TLS_DHE_RSA_WITH_AES_256_CBC_SHA256","TLS_DH_anon_WITH_AES_128_CBC_SHA256","TLS_DH_anon_WITH_AES_256_CBC_SHA256","TLS_RSA_WITH_CAMELLIA_256_CBC_SHA","TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA","TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA","TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA","TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA","TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA","TLS_PSK_WITH_RC4_128_SHA","TLS_PSK_WITH_3DES_EDE_CBC_SHA","TLS_PSK_WITH_AES_128_CBC_SHA","TLS_PSK_WITH_AES_256_CBC_SHA","TLS_DHE_PSK_WITH_RC4_128_SHA","TLS_DHE_PSK_WITH_3DES_EDE_CBC_SHA","TLS_DHE_PSK_WITH_AES_128_CBC_SHA","TLS_DHE_PSK_WITH_AES_256_CBC_SHA","TLS_RSA_PSK_WITH_RC4_128_SHA","TLS_RSA_PSK_WITH_3DES_EDE_CBC_SHA","TLS_RSA_PSK_WITH_AES_128_CBC_SHA","TLS_RSA_PSK_WITH_AES_256_CBC_SHA","TLS_RSA_WITH_SEED_CBC_SHA","TLS_DH_DSS_WITH_SEED_CBC_SHA","TLS_DH_RSA_WITH_SEED_CBC_SHA","TLS_DHE_DSS_WITH_SEED_CBC_SHA","TLS_DHE_RSA_WITH_SEED_CBC_SHA","TLS_DH_anon_WITH_SEED_CBC_SHA","TLS_RSA_WITH_AES_128_GCM_SHA256","TLS_RSA_WITH_AES_256_GCM_SHA384","TLS_DHE_RSA_WITH_AES_128_GCM_SHA256","TLS_DHE_RSA_WITH_AES_256_GCM_SHA384","TLS_DH_RSA_WITH_AES_128_GCM_SHA256","TLS_DH_RSA_WITH_AES_256_GCM_SHA384","TLS_DHE_DSS_WITH_AES_128_GCM_SHA256","TLS_DHE_DSS_WITH_AES_256_GCM_SHA384","TLS_DH_DSS_WITH_AES_128_GCM_SHA256","TLS_DH_DSS_WITH_AES_256_GCM_SHA384","TLS_DH_anon_WITH_AES_128_GCM_SHA256","TLS_DH_anon_WITH_AES_256_GCM_SHA384","TLS_PSK_WITH_AES_128_GCM_SHA256","TLS_PSK_WITH_AES_256_GCM_SHA384","TLS_DHE_PSK_WITH_AES_128_GCM_SHA256","TLS_DHE_PSK_WITH_AES_256_GCM_SHA384","TLS_RSA_PSK_WITH_AES_128_GCM_SHA256","TLS_RSA_PSK_WITH_AES_256_GCM_SHA384","TLS_PSK_WITH_AES_128_CBC_SHA256","TLS_PSK_WITH_AES_256_CBC_SHA384","TLS_PSK_WITH_NULL_SHA256","TLS_PSK_WITH_NULL_SHA384","TLS_DHE_PSK_WITH_AES_128_CBC_SHA256","TLS_DHE_PSK_WITH_AES_256_CBC_SHA384","TLS_DHE_PSK_WITH_NULL_SHA256","TLS_DHE_PSK_WITH_NULL_SHA384","TLS_RSA_PSK_WITH_AES_128_CBC_SHA256","TLS_RSA_PSK_WITH_AES_256_CBC_SHA384","TLS_RSA_PSK_WITH_NULL_SHA256","TLS_RSA_PSK_WITH_NULL_SHA384","TLS_RSA_WITH_CAMELLIA_128_CBC_SHA256","TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA256","TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA256","TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA256","TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA256","TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA256","TLS_RSA_WITH_CAMELLIA_256_CBC_SHA256","TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA256","TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA256","TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA256","TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA256","TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA256","TLS_SM4_GCM_SM3","TLS_SM4_CCM_SM3","TLS_AES_128_GCM_SHA256","TLS_AES_256_GCM_SHA384","TLS_CHACHA20_POLY1305_SHA256","TLS_AES_128_CCM_SHA256","TLS_AES_128_CCM_8_SHA256","TLS_AEGIS_256_SHA384","TLS_AEGIS_128L_SHA256","TLS_ECDH_ECDSA_WITH_NULL_SHA","TLS_ECDH_ECDSA_WITH_RC4_128_SHA","TLS_ECDH_ECDSA_WITH_3DES_EDE_CBC_SHA","TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA","TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA","TLS_ECDHE_ECDSA_WITH_NULL_SHA","TLS_ECDHE_ECDSA_WITH_RC4_128_SHA","TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA","TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA","TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA","TLS_ECDH_RSA_WITH_NULL_SHA","TLS_ECDH_RSA_WITH_RC4_128_SHA","TLS_ECDH_RSA_WITH_3DES_EDE_CBC_SHA","TLS_ECDH_RSA_WITH_AES_128_CBC_SHA","TLS_ECDH_RSA_WITH_AES_256_CBC_SHA","TLS_ECDHE_RSA_WITH_NULL_SHA","TLS_ECDHE_RSA_WITH_RC4_128_SHA","TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA","TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA","TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA","TLS_ECDH_anon_WITH_NULL_SHA","TLS_ECDH_anon_WITH_RC4_128_SHA","TLS_ECDH_anon_WITH_3DES_EDE_CBC_SHA","TLS_ECDH_anon_WITH_AES_128_CBC_SHA","TLS_ECDH_anon_WITH_AES_256_CBC_SHA","TLS_SRP_SHA_WITH_3DES_EDE_CBC_SHA","TLS_SRP_SHA_RSA_WITH_3DES_EDE_CBC_SHA","TLS_SRP_SHA_DSS_WITH_3DES_EDE_CBC_SHA","TLS_SRP_SHA_WITH_AES_128_CBC_SHA","TLS_SRP_SHA_RSA_WITH_AES_128_CBC_SHA","TLS_SRP_SHA_DSS_WITH_AES_128_CBC_SHA","TLS_SRP_SHA_WITH_AES_256_CBC_SHA","TLS_SRP_SHA_RSA_WITH_AES_256_CBC_SHA","TLS_SRP_SHA_DSS_WITH_AES_256_CBC_SHA","TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256","TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384","TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA256","TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA384","TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256","TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384","TLS_ECDH_RSA_WITH_AES_128_CBC_SHA256","TLS_ECDH_RSA_WITH_AES_256_CBC_SHA384","TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256","TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384","TLS_ECDH_ECDSA_WITH_AES_128_GCM_SHA256","TLS_ECDH_ECDSA_WITH_AES_256_GCM_SHA384","TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256","TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384","TLS_ECDH_RSA_WITH_AES_128_GCM_SHA256","TLS_ECDH_RSA_WITH_AES_256_GCM_SHA384","TLS_ECDHE_PSK_WITH_RC4_128_SHA","TLS_ECDHE_PSK_WITH_3DES_EDE_CBC_SHA","TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA","TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA","TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA256","TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA384","TLS_ECDHE_PSK_WITH_NULL_SHA","TLS_ECDHE_PSK_WITH_NULL_SHA256","TLS_ECDHE_PSK_WITH_NULL_SHA384","TLS_RSA_WITH_ARIA_128_CBC_SHA256","TLS_RSA_WITH_ARIA_256_CBC_SHA384","TLS_DH_DSS_WITH_ARIA_128_CBC_SHA256","TLS_DH_DSS_WITH_ARIA_256_CBC_SHA384","TLS_DH_RSA_WITH_ARIA_128_CBC_SHA256","TLS_DH_RSA_WITH_ARIA_256_CBC_SHA384","TLS_DHE_DSS_WITH_ARIA_128_CBC_SHA256","TLS_DHE_DSS_WITH_ARIA_256_CBC_SHA384","TLS_DHE_RSA_WITH_ARIA_128_CBC_SHA256","TLS_DHE_RSA_WITH_ARIA_256_CBC_SHA384","TLS_DH_anon_WITH_ARIA_128_CBC_SHA256","TLS_DH_anon_WITH_ARIA_256_CBC_SHA384","TLS_ECDHE_ECDSA_WITH_ARIA_128_CBC_SHA256","TLS_ECDHE_ECDSA_WITH_ARIA_256_CBC_SHA384","TLS_ECDH_ECDSA_WITH_ARIA_128_CBC_SHA256","TLS_ECDH_ECDSA_WITH_ARIA_256_CBC_SHA384","TLS_ECDHE_RSA_WITH_ARIA_128_CBC_SHA256","TLS_ECDHE_RSA_WITH_ARIA_256_CBC_SHA384","TLS_ECDH_RSA_WITH_ARIA_128_CBC_SHA256","TLS_ECDH_RSA_WITH_ARIA_256_CBC_SHA384","TLS_RSA_WITH_ARIA_128_GCM_SHA256","TLS_RSA_WITH_ARIA_256_GCM_SHA384","TLS_DHE_RSA_WITH_ARIA_128_GCM_SHA256","TLS_DHE_RSA_WITH_ARIA_256_GCM_SHA384","TLS_DH_RSA_WITH_ARIA_128_GCM_SHA256","TLS_DH_RSA_WITH_ARIA_256_GCM_SHA384","TLS_DHE_DSS_WITH_ARIA_128_GCM_SHA256","TLS_DHE_DSS_WITH_ARIA_256_GCM_SHA384","TLS_DH_DSS_WITH_ARIA_128_GCM_SHA256","TLS_DH_DSS_WITH_ARIA_256_GCM_SHA384","TLS_DH_anon_WITH_ARIA_128_GCM_SHA256","TLS_DH_anon_WITH_ARIA_256_GCM_SHA384","TLS_ECDHE_ECDSA_WITH_ARIA_128_GCM_SHA256","TLS_ECDHE_ECDSA_WITH_ARIA_256_GCM_SHA384","TLS_ECDH_ECDSA_WITH_ARIA_128_GCM_SHA256","TLS_ECDH_ECDSA_WITH_ARIA_256_GCM_SHA384","TLS_ECDHE_RSA_WITH_ARIA_128_GCM_SHA256","TLS_ECDHE_RSA_WITH_ARIA_256_GCM_SHA384","TLS_ECDH_RSA_WITH_ARIA_128_GCM_SHA256","TLS_ECDH_RSA_WITH_ARIA_256_GCM_SHA384","TLS_PSK_WITH_ARIA_128_CBC_SHA256","TLS_PSK_WITH_ARIA_256_CBC_SHA384","TLS_DHE_PSK_WITH_ARIA_128_CBC_SHA256","TLS_DHE_PSK_WITH_ARIA_256_CBC_SHA384","TLS_RSA_PSK_WITH_ARIA_128_CBC_SHA256","TLS_RSA_PSK_WITH_ARIA_256_CBC_SHA384","TLS_PSK_WITH_ARIA_128_GCM_SHA256","TLS_PSK_WITH_ARIA_256_GCM_SHA384","TLS_DHE_PSK_WITH_ARIA_128_GCM_SHA256","TLS_DHE_PSK_WITH_ARIA_256_GCM_SHA384","TLS_RSA_PSK_WITH_ARIA_128_GCM_SHA256","TLS_RSA_PSK_WITH_ARIA_256_GCM_SHA384","TLS_ECDHE_PSK_WITH_ARIA_128_CBC_SHA256","TLS_ECDHE_PSK_WITH_ARIA_256_CBC_SHA384","TLS_ECDHE_ECDSA_WITH_CAMELLIA_128_CBC_SHA256","TLS_ECDHE_ECDSA_WITH_CAMELLIA_256_CBC_SHA384","TLS_ECDH_ECDSA_WITH_CAMELLIA_128_CBC_SHA256","TLS_ECDH_ECDSA_WITH_CAMELLIA_256_CBC_SHA384","TLS_ECDHE_RSA_WITH_CAMELLIA_128_CBC_SHA256","TLS_ECDHE_RSA_WITH_CAMELLIA_256_CBC_SHA384","TLS_ECDH_RSA_WITH_CAMELLIA_128_CBC_SHA256","TLS_ECDH_RSA_WITH_CAMELLIA_256_CBC_SHA384","TLS_RSA_WITH_CAMELLIA_128_GCM_SHA256","TLS_RSA_WITH_CAMELLIA_256_GCM_SHA384","TLS_DHE_RSA_WITH_CAMELLIA_128_GCM_SHA256","TLS_DHE_RSA_WITH_CAMELLIA_256_GCM_SHA384","TLS_DH_RSA_WITH_CAMELLIA_128_GCM_SHA256","TLS_DH_RSA_WITH_CAMELLIA_256_GCM_SHA384","TLS_DHE_DSS_WITH_CAMELLIA_128_GCM_SHA256","TLS_DHE_DSS_WITH_CAMELLIA_256_GCM_SHA384","TLS_DH_DSS_WITH_CAMELLIA_128_GCM_SHA256","TLS_DH_DSS_WITH_CAMELLIA_256_GCM_SHA384","TLS_DH_anon_WITH_CAMELLIA_128_GCM_SHA256","TLS_DH_anon_WITH_CAMELLIA_256_GCM_SHA384","TLS_ECDHE_ECDSA_WITH_CAMELLIA_128_GCM_SHA256","TLS_ECDHE_ECDSA_WITH_CAMELLIA_256_GCM_SHA384","TLS_ECDH_ECDSA_WITH_CAMELLIA_128_GCM_SHA256","TLS_ECDH_ECDSA_WITH_CAMELLIA_256_GCM_SHA384","TLS_ECDHE_RSA_WITH_CAMELLIA_128_GCM_SHA256","TLS_ECDHE_RSA_WITH_CAMELLIA_256_GCM_SHA384","TLS_ECDH_RSA_WITH_CAMELLIA_128_GCM_SHA256","TLS_ECDH_RSA_WITH_CAMELLIA_256_GCM_SHA384","TLS_PSK_WITH_CAMELLIA_128_GCM_SHA256","TLS_PSK_WITH_CAMELLIA_256_GCM_SHA384","TLS_DHE_PSK_WITH_CAMELLIA_128_GCM_SHA256","TLS_DHE_PSK_WITH_CAMELLIA_256_GCM_SHA384","TLS_RSA_PSK_WITH_CAMELLIA_128_GCM_SHA256","TLS_RSA_PSK_WITH_CAMELLIA_256_GCM_SHA384","TLS_PSK_WITH_CAMELLIA_128_CBC_SHA256","TLS_PSK_WITH_CAMELLIA_256_CBC_SHA384","TLS_DHE_PSK_WITH_CAMELLIA_128_CBC_SHA256","TLS_DHE_PSK_WITH_CAMELLIA_256_CBC_SHA384","TLS_RSA_PSK_WITH_CAMELLIA_128_CBC_SHA256","TLS_RSA_PSK_WITH_CAMELLIA_256_CBC_SHA384","TLS_ECDHE_PSK_WITH_CAMELLIA_128_CBC_SHA256","TLS_ECDHE_PSK_WITH_CAMELLIA_256_CBC_SHA384","TLS_RSA_WITH_AES_128_CCM","TLS_RSA_WITH_AES_256_CCM","TLS_DHE_RSA_WITH_AES_128_CCM","TLS_DHE_RSA_WITH_AES_256_CCM","TLS_RSA_WITH_AES_128_CCM_8","TLS_RSA_WITH_AES_256_CCM_8","TLS_DHE_RSA_WITH_AES_128_CCM_8","TLS_DHE_RSA_WITH_AES_256_CCM_8","TLS_PSK_WITH_AES_128_CCM","TLS_PSK_WITH_AES_256_CCM","TLS_DHE_PSK_WITH_AES_128_CCM","TLS_DHE_PSK_WITH_AES_256_CCM","TLS_PSK_WITH_AES_128_CCM_8","TLS_PSK_WITH_AES_256_CCM_8","TLS_PSK_DHE_WITH_AES_128_CCM_8","TLS_PSK_DHE_WITH_AES_256_CCM_8","TLS_ECDHE_ECDSA_WITH_AES_128_CCM","TLS_ECDHE_ECDSA_WITH_AES_256_CCM","TLS_ECDHE_ECDSA_WITH_AES_128_CCM_8","TLS_ECDHE_ECDSA_WITH_AES_256_CCM_8","TLS_ECCPWD_WITH_AES_128_GCM_SHA256","TLS_ECCPWD_WITH_AES_256_GCM_SHA384","TLS_ECCPWD_WITH_AES_128_CCM_SHA256","TLS_ECCPWD_WITH_AES_256_CCM_SHA384","TLS_SHA256_SHA256","TLS_SHA384_SHA384","TLS_GOSTR341112_256_WITH_KUZNYECHIK_CTR_OMAC","TLS_GOSTR341112_256_WITH_MAGMA_CTR_OMAC","TLS_GOSTR341112_256_WITH_28147_CNT_IMIT","TLS_GOSTR341112_256_WITH_KUZNYECHIK_MGM_L","TLS_GOSTR341112_256_WITH_MAGMA_MGM_L","TLS_GOSTR341112_256_WITH_KUZNYECHIK_MGM_S","TLS_GOSTR341112_256_WITH_MAGMA_MGM_S","TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256","TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256","TLS_DHE_RSA_WITH_CHACHA20_POLY1305_SHA256","TLS_PSK_WITH_CHACHA20_POLY1305_SHA256","TLS_ECDHE_PSK_WITH_CHACHA20_POLY1305_SHA256","TLS_DHE_PSK_WITH_CHACHA20_POLY1305_SHA256","TLS_RSA_PSK_WITH_CHACHA20_POLY1305_SHA256","TLS_ECDHE_PSK_WITH_AES_128_GCM_SHA256","TLS_ECDHE_PSK_WITH_AES_256_GCM_SHA384","TLS_ECDHE_PSK_WITH_AES_128_CCM_8_SHA256","TLS_ECDHE_PSK_WITH_AES_128_CCM_SHA256"];
	let mut clienthello1:[u8;494];
	println!("taille reserve={:?}",reserve1.len());
	clienthello1=[22, 3, 3, 1, 233, 1, 0, 1, 229, 3, 3, 99, 131, 134, 225, 156, 76, 100, 101, 240, 241, 134, 99, 187, 13, 53, 152, 15, 8, 125, 123, 20, 224, 174, 145, 240, 217, 170, 119, 210, 211, 81, 188, 32, 235, 2, 0, 0, 115, 8, 55, 131, 218, 163, 16, 239, 24, 118, 148, 227, 253, 5, 187, 1, 64, 244, 254, 204, 167, 137, 72, 220, 8, 193, 206, 142, 0, 2, 192, 44, 1, 0, 1, 154, 0, 0, 0, 26, 0, 24, 0, 0, 21, 111, 117, 116, 108, 111, 111, 107, 46, 111, 102, 102, 105, 99, 101, 51, 54, 53, 46, 99, 111, 109, 0, 5, 0, 5, 1, 0, 0, 0, 0, 0, 10, 0, 8, 0, 6, 0, 29, 0, 23, 0, 24, 0, 11, 0, 2, 1, 0, 0, 13, 0, 26, 0, 24, 8, 4, 8, 5, 8, 6, 4, 1, 5, 1, 2, 1, 4, 3, 5, 3, 2, 3, 2, 2, 6, 1, 6, 3, 0, 35, 1, 36, 0, 0, 0, 0, 2, 205, 114, 223, 90, 246, 135, 78, 184, 45, 90, 159, 228, 237, 119, 207, 116, 115, 190, 133, 102, 66, 1, 89, 98, 161, 81, 219, 190, 140, 188, 180, 57, 80, 151, 251, 147, 113, 181, 135, 169, 238, 147, 113, 181, 135, 40, 150, 91, 164, 159, 70, 122, 93, 18, 3, 42, 4, 249, 232, 142, 213, 105, 135, 22, 223, 196, 135, 175, 152, 127, 3, 80, 220, 134, 163, 238, 149, 10, 200, 218, 118, 86, 83, 249, 196, 153, 2, 134, 138, 185, 225, 205, 44, 143, 227, 187, 142, 234, 153, 234, 166, 137, 59, 243, 236, 25, 197, 115, 69, 253, 28, 81, 99, 32, 59, 72, 74, 108, 79, 78, 55, 227, 18, 82, 6, 74, 121, 216, 237, 187, 129, 222, 163, 24, 220, 9, 151, 78, 100, 173, 29, 255, 221, 38, 162, 11, 200, 240, 216, 225, 128, 30, 166, 222, 67, 63, 141, 111, 168, 46, 51, 214, 222, 65, 79, 38, 9, 179, 98, 84, 19, 164, 172, 162, 62, 53, 144, 54, 111, 126, 92, 51, 240, 174, 218, 111, 5, 236, 128, 229, 17, 16, 52, 19, 0, 242, 235, 152, 60, 74, 219, 202, 40, 168, 3, 243, 1, 44, 4, 243, 58, 43, 110, 240, 61, 54, 7, 35, 126, 199, 64, 174, 119, 165, 194, 111, 70, 122, 129, 88, 43, 211, 55, 107, 18, 125, 150, 7, 176, 117, 26, 72, 234, 100, 8, 100, 56, 17, 86, 147, 51, 114, 203, 122, 207, 33, 43, 77, 172, 40, 98, 110, 128, 81, 27, 118, 204, 8, 185, 142, 131, 97, 156, 130, 240, 242, 39, 173, 192, 171, 44, 70, 233, 166, 203, 96, 135, 0, 16, 0, 14, 0, 12, 2, 104, 50, 8, 104, 116, 116, 112, 47, 49, 46, 49, 0, 23, 0, 0, 255, 1, 0, 1, 0];



	let mut cypher1=[0,0];
	let mut tabcypher=[cypher1;362];
	let mut numero:usize=0;

	let mut index:usize=0;

	let mut tampon=resultatsannalyse.clone();
	let (tr, rec) = mpsc::channel();
	let mut unclone=positionanalyse;
	let mut reponse_serveur=temp2;
	let init=temp2;

	thread::spawn(move || {
		let mut flag=0;
		let mut tls0=0;
		let mut tls1=0;
		let mut tls2=0;
		let mut tls3=0;
		let mut rap=Rapport{liste: tampon.clone(), reserv: init};
		for i in rx {
			println!("il se passe des trucs");
			if i.extinction=="fin de communication"{
				break;
			}
			if i.repserv[0] != 0 && flag==0 {
				reponse_serveur=i.repserv;
				flag=1;
			}
			if tls0==0 && (i.tlstab[0]=="TLS 1.0 supportée" || i.tlstab[0]=="TLS 1.0 non supportée") {
				tampon[unclone]=i.tlstab[0].to_string();
				println!("je tls 1.0");
				tls0=1;
				unclone+=1;
			}
			if tls1==0 && (i.tlstab[1]=="TLS 1.1 supportée" || i.tlstab[1]=="TLS 1.1 non supportée") {
				tampon[unclone]=i.tlstab[1].to_string();
				println!("je tls 1.1");
				tls1=1;
				unclone+=1;
			}
			if tls2==0 && (i.tlstab[2]=="TLS 1.2 supportée" || i.tlstab[2]=="TLS 1.2 non supportée") {
				tampon[unclone]=i.tlstab[2].to_string();
				println!("je tls 1.2");
				tls2=1;
				unclone+=1;
			}
			if tls3==0 && (i.tlstab[3]=="TLS 1.3 supportée" || i.tlstab[3]=="TLS 1.3 non supportée") {
				tampon[unclone]=i.tlstab[3].to_string();
				println!("je tls 1.3");
				tls3=1;
				unclone+=1;
			}
			if i.suite != "" {
				tampon[unclone]=i.suite.to_string();
				println!("j'écoute");
				unclone+=1;
			}
		}
		rap.liste=tampon;
		rap.reserv=reponse_serveur;
		println!("tampon={:?}",rap.liste);
		thread::sleep(Duration::from_secs(3));
		println!("j'envoie");
		tr.send(rap).unwrap();
		thread::sleep(Duration::from_secs(3));

	});

	let mut id:usize=0;
	while cypher1 != [208,6] {
		let mut clienthello=clienthello1;
		let cypher=cypher1;
		let reserv=reserve1[index];
		let com=tx.clone();
		let mut temp=temp2;
		let mut rep=String::new();
		tabcypher[id]=cypher1;
		id+=1;
		let ad=add.clone();
		let dom=domain.clone();
		println!("{:?}",cypher1);

		thread::spawn(move || {

			let mut calcul=String::from("HELO".to_owned() + &dom +"\r\n");
			let mut wapr=TcpStream::connect(&ad);
			let mut jaj=1;
			match wapr {
				Err(_) => {println!("tentative échouée, reconnection en cours...");}
				Ok(_) => {jaj=0;}
			}
			while jaj != 0 {
				println!("tentative échouée, reconnection en cours...");
				wapr=TcpStream::connect(&ad);

				match wapr {
					Err(_) => {println!("tentative échouée, reconnection en cours...");jaj+=1;}
					Ok(_) => {jaj=0;}
				}
				if jaj >= 10 {
					break;
				}
			}


			let mut retour=Renvoi { repserv: [0;10000], suite: "", tlstab: ["";5], extinction: "" };
			println!("mise en place de la cellule d'écoute");
			wapr.as_ref().expect("un truc").write(calcul.as_bytes()).unwrap();
			wapr.as_ref().expect("deux trucs").flush().unwrap();
			println!("message envoyé");
			temp = [0;10000];
			rep=String::new();
			wapr.as_ref().expect("reason").read(&mut temp).unwrap();
			rep=(&String::from_utf8_lossy(&temp [..])).to_string();
			println!("message reçu: {}",rep);
			temp = [0;10000];
			thread::sleep(Duration::from_secs(1));
			calcul=String::from("STARTTLS\r\n");
			rep=String::from("");
			wapr.as_ref().expect("un truc").write(calcul.as_bytes()).unwrap();
			wapr.as_ref().expect("deux trucs").flush().unwrap();
			println!("envoi du starttls...");
			wapr.as_ref().expect("reason").read(&mut temp).unwrap();
			rep=(&String::from_utf8_lossy(&temp [..])).to_string();
			println!("message reçu: {}",rep);
			temp = [0;10000];
			thread::sleep(Duration::from_secs(1));
			wapr.as_ref().expect("reason").read(&mut temp).unwrap();
			println!("reponse au starttls {}",&String::from_utf8_lossy(&temp [..]));

			clienthello[78]=cypher[0];
			clienthello[79]=cypher[1];

			wapr.as_ref().expect("un truc").write(&clienthello).unwrap();
			wapr.as_ref().expect("deux trucs").flush().unwrap();

			wapr.as_ref().expect("reason").read(&mut temp).unwrap();

			println!("cypher={:?}\ncyphersuite={:?},{:?}",cypher,temp[44],temp[45]);


			if temp[44] == cypher[0] {
				if temp[45]==cypher[1] {
					if temp[5] == 2 {
						if temp[0] == 22 {
							println!("cyphersuite supportée");
							retour.suite=reserv;


							thread::sleep(Duration::from_secs(3));
							retour.repserv=temp;

							wapr.as_ref().expect("oui").shutdown(Shutdown::Both);


							thread::sleep(Duration::from_secs(1));
							let mut tip=0;

							calcul=String::from("HELO".to_owned() + &dom +"\r\n");
							wapr=TcpStream::connect(&ad);
							jaj=1;
							match wapr {
								Err(_) => {println!("tentative échouée, reconnection en cours...");}
								Ok(_) => {jaj=0;}
							}

							while jaj != 0 {
								println!("tentative échouée, reconnection en cours...");
								wapr=TcpStream::connect(&ad);
								match wapr {
									Err(_) => {println!("tentative échouée, reconnection en cours...");jaj+=1;}
									Ok(_) => {jaj=0;}
								}
								if jaj >= 10 {
									break;
								}
							}



							println!("mise en place de la cellule d'écoute");
							wapr.as_ref().expect("un truc").write(calcul.as_bytes()).unwrap();
							wapr.as_ref().expect("deux trucs").flush().unwrap();
							println!("message envoyé");
							temp = [0;10000];
							rep=String::new();
							wapr.as_ref().expect("reason").read(&mut temp).unwrap();
							rep=(&String::from_utf8_lossy(&temp [..])).to_string();
							//println!("message reçu: {}",rep);
							temp = [0;10000];
							thread::sleep(Duration::from_secs(1));
							calcul=String::from("STARTTLS\r\n");
							rep=String::from("");
							wapr.as_ref().expect("un truc").write(calcul.as_bytes()).unwrap();
							wapr.as_ref().expect("deux trucs").flush().unwrap();
							wapr.as_ref().expect("reason").read(&mut temp).unwrap();
							rep=(&String::from_utf8_lossy(&temp [..])).to_string();
							temp2 = [0;10000];
							thread::sleep(Duration::from_secs(1));
							wapr.as_ref().expect("reason").read(&mut temp).unwrap();

							clienthello[2]=0;
							wapr.as_ref().expect("un truc").write(&clienthello).unwrap();
							wapr.as_ref().expect("deux trucs").flush().unwrap();
							wapr.as_ref().expect("reason").read(&mut temp).unwrap();

							positionanalyse+=1;

							if temp2[0]==22 {
								if temp2[1]==3 {
									if temp2[2]==1 {
										tip=1;
										retour.tlstab[0]="TLS 1.0 supportée";

									}
								}
							}
							if tip != 1 {
								retour.tlstab[0]="TLS 1.0 non supportée";
							}

							tip=0;
							wapr.expect("oui").shutdown(Shutdown::Both);

							thread::sleep(Duration::from_secs(1));

							calcul=String::from("HELO".to_owned() + &dom +"\r\n");
							wapr=TcpStream::connect(&ad);
							jaj=1;
							match wapr {
								Err(_) => {println!("tentative échouée, reconnection en cours...");}
								Ok(_) => {jaj=0;}
							}
							while jaj != 0 {
								println!("tentative échouée, reconnection en cours...");
								wapr=TcpStream::connect(&ad);
								match wapr {
									Err(_) => {println!("tentative échouée, reconnection en cours...");jaj+=1;}
									Ok(_) => {jaj=0;}
								}
								if jaj >= 10 {
									break;
								}
							}



							println!("mise en place de la cellule d'écoute");
							wapr.as_ref().expect("un truc").write(calcul.as_bytes()).unwrap();
							wapr.as_ref().expect("deux trucs").flush().unwrap();
							println!("message envoyé");
							temp2 = [0;10000];
							rep=String::new();
							wapr.as_ref().expect("reason").read(&mut temp2).unwrap();
							rep=(&String::from_utf8_lossy(&temp2 [..])).to_string();
							//println!("message reçu: {}",rep);
							temp2 = [0;10000];
							thread::sleep(Duration::from_secs(1));
							calcul=String::from("STARTTLS\r\n");
							rep=String::from("");
							wapr.as_ref().expect("un truc").write(calcul.as_bytes()).unwrap();
							wapr.as_ref().expect("deux trucs").flush().unwrap();
							wapr.as_ref().expect("reason").read(&mut temp2).unwrap();
							rep=(&String::from_utf8_lossy(&temp2 [..])).to_string();
							temp2 = [0;10000];
							thread::sleep(Duration::from_secs(1));
							wapr.as_ref().expect("reason").read(&mut temp2).unwrap();

							clienthello[2]=1;
							wapr.as_ref().expect("un truc").write(&clienthello).unwrap();
							wapr.as_ref().expect("deux trucs").flush().unwrap();
							wapr.as_ref().expect("reason").read(&mut temp2).unwrap();
							positionanalyse+=1;
							if temp2[0]==22 {
								if temp2[1]==3 {
									if temp2[2]==2 {
										tip=1;
										retour.tlstab[1]="TLS 1.1 supportée";

									}
								}
							}
							if tip != 1 {
								retour.tlstab[1]="TLS 1.1 non supportée";
							}
							tip=0;
							wapr.expect("oui").shutdown(Shutdown::Both);

							thread::sleep(Duration::from_secs(1));

							calcul=String::from("HELO".to_owned() + &dom +"\r\n");
							wapr=TcpStream::connect(&ad);
							jaj=1;
							match wapr {
								Err(_) => {println!("tentative échouée, reconnection en cours...");}
								Ok(_) => {jaj=0;}
							}
							while jaj != 0 {
								println!("tentative échouée, reconnection en cours...");
								wapr=TcpStream::connect(&ad);
								match wapr {
									Err(_) => {println!("tentative échouée, reconnection en cours...");jaj+=1}
									Ok(_) => {jaj=0;}
								}
								if jaj >= 10 {
									break;
								}
							}



							println!("mise en place de la cellule d'écoute");
							wapr.as_ref().expect("un truc").write(calcul.as_bytes()).unwrap();
							wapr.as_ref().expect("deux trucs").flush().unwrap();
							println!("message envoyé");
							temp2 = [0;10000];
							rep=String::new();
							wapr.as_ref().expect("reason").read(&mut temp2).unwrap();
							rep=(&String::from_utf8_lossy(&temp2 [..])).to_string();
							println!("message reçu: {}",rep);
							temp2 = [0;10000];
							thread::sleep(Duration::from_secs(1));
							calcul=String::from("STARTTLS\r\n");
							rep=String::from("");
							wapr.as_ref().expect("un truc").write(calcul.as_bytes()).unwrap();
							wapr.as_ref().expect("deux trucs").flush().unwrap();
							wapr.as_ref().expect("reason").read(&mut temp2).unwrap();
							rep=(&String::from_utf8_lossy(&temp2 [..])).to_string();
							temp2 = [0;10000];
							thread::sleep(Duration::from_secs(1));
							wapr.as_ref().expect("reason").read(&mut temp2).unwrap();

							clienthello[2]=2;
							wapr.as_ref().expect("un truc").write(&clienthello).unwrap();
							wapr.as_ref().expect("deux trucs").flush().unwrap();
							wapr.as_ref().expect("reason").read(&mut temp2).unwrap();
							positionanalyse+=1;
							if temp2[0]==22 {
								if temp2[1]==3 {
									if temp2[2]==3 {
										tip=1;
										retour.tlstab[2]="TLS 1.2 supportée";
									}
								}
							}
							if tip != 1 {
								retour.tlstab[2]="TLS 1.2 non supportée";
							}

							tip=0;
							wapr.expect("oui").shutdown(Shutdown::Both);

							thread::sleep(Duration::from_secs(1));

							calcul=String::from("HELO".to_owned() + &dom +"\r\n");
							wapr=TcpStream::connect(&ad);
							jaj=1;
							match wapr {
								Err(_) => {println!("tentative échouée, reconnection en cours...");}
								Ok(_) => {jaj=0;}
							}
							while jaj != 0 {
								println!("tentative échouée, reconnection en cours...");
								wapr=TcpStream::connect(&ad);
								match wapr {
									Err(_) => {println!("tentative échouée, reconnection en cours...");jaj+=1;}
									Ok(_) => {jaj=0;}
								}
								if jaj >= 10 {
									break;
								}
							}



							println!("mise en place de la cellule d'écoute");
							wapr.as_ref().expect("un truc").write(calcul.as_bytes()).unwrap();
							wapr.as_ref().expect("deux trucs").flush().unwrap();
							println!("message envoyé");
							temp2 = [0;10000];
							rep=String::new();
							wapr.as_ref().expect("reason").read(&mut temp2).unwrap();
							rep=(&String::from_utf8_lossy(&temp2 [..])).to_string();
							println!("message reçu: {}",rep);
							temp2 = [0;10000];
							thread::sleep(Duration::from_secs(1));
							calcul=String::from("STARTTLS\r\n");
							rep=String::from("");
							wapr.as_ref().expect("un truc").write(calcul.as_bytes()).unwrap();
							wapr.as_ref().expect("deux trucs").flush().unwrap();
							wapr.as_ref().expect("reason").read(&mut temp2).unwrap();
							rep=(&String::from_utf8_lossy(&temp2 [..])).to_string();
							temp2 = [0;10000];
							thread::sleep(Duration::from_secs(1));
							wapr.as_ref().expect("reason").read(&mut temp2).unwrap();

							clienthello[2]=4;
							wapr.as_ref().expect("un truc").write(&clienthello).unwrap();
							wapr.as_ref().expect("deux trucs").flush().unwrap();
							wapr.as_ref().expect("reason").read(&mut temp2).unwrap();
							positionanalyse+=1;
							if temp2[0]==22 {
								if temp2[1]==3 {
									if temp2[2]==4 {
										tip=1;
										retour.tlstab[3]="TLS 1.3 supportée";
									}
								}
							}
							if tip != 1 {
								retour.tlstab[3]="TLS 1.3 non supportée";
							}
							wapr.as_ref().expect("oui").shutdown(Shutdown::Both);
							thread::sleep(Duration::from_secs(2));


						}

					}


				}
			}
			if cypher==[208,5] {
				thread::sleep(Duration::from_secs(20));
				retour.extinction="fin de communication";
				println!("le dernier thread est mort. longue vie au main !");
				com.send(retour).unwrap();
			}
			else {
				com.send(retour).unwrap();
				wapr.expect("oui").shutdown(Shutdown::Both);
				thread::sleep(Duration::from_secs(2));
				println!("je m'éteint");
			}
		});

		if cypher1[1] >= 255 {
			cypher1[0]=*&cypher1[0]+1;
			cypher1[1]=0;
		}
		else{
			cypher1[1]=*&cypher1[1]+1;
		}
		match cypher1 {
			[0,28] => cypher1=[0,30],
			[0,70] => cypher1=[0,103],
			[0,110] => cypher1=[0,132],
			[0,200] => cypher1=[0,255],
			[1,0] => cypher1=[19,1],
			[19,8] => cypher1=[192,1],
			[192,182] => cypher1=[193,0],
			[193,7] => cypher1=[204,168],
			[204,175] => cypher1=[208,1],
			[208,4] => cypher1=[208,5],
			_ =>  (),
		}
		index+=1;

		thread::sleep(Duration::from_millis(250));
	}

	println!("je suis prêt pour transfer de données");
	let mut comp=0;
	for i in rec {
		println!("ça mouline");
		resultatsannalyse=i.liste.clone();
		temp2=i.reserv;
		break;
	}
	//recherche du premier emplacement libre
	pol=resultatsannalyse[1].clone();
	positionanalyse=1 as usize;
	while pol.ne("test") {
		positionanalyse+=1;
		pol=resultatsannalyse[positionanalyse].clone();
	}
	positionanalyse-=2;





	//partie annalyse de satan que je touche plus jamais

	let tamp1:u64=(temp2[3]).into();
	let tamp2:u64=(temp2[4]).into();
	let mut position:u64=tamp1*LEPETIT+tamp2+LAPS;
	let pos2:usize=position as usize;
	let mut tel:u64=(temp2[pos2]).into();
	let mut longcert:u64=tel*LEGROS;
	position+=1;
	let pos2:usize=position as usize;
	tel=(temp2[pos2]).into();
	longcert=longcert+tel*LEPETIT;
	position+=1;
	let pos2:usize=position as usize;
	tel=(temp2[pos2]).into();
	longcert=longcert+tel;
	tel=longcert;
	position=position+tel+1;
	let pos2:usize=position as usize;
	tel=(temp2[pos2]).into();
	longcert=tel*LEGROS;
	position+=1;
	let pos2:usize=position as usize;
	tel=(temp2[pos2]).into();
	longcert=longcert+tel*LEPETIT;
	position+=1;
	let pos2:usize=position as usize;
	tel=(temp2[pos2]).into();
	longcert=longcert+tel;
	println!("longeur du CA certificat {}",longcert);
	position+=1;
	let pos2:usize=position as usize;
	let pos3:usize=(position+longcert).try_into().unwrap();
	let certif=&temp2[pos2..pos3];

	let mut traduction=String::from_utf8_lossy(&certif [..]);
	let mut tradfinal=String::from("");


	for fi in traduction.chars() {
		match fi {
			'a' => tradfinal=tradfinal+"a",
			'b' => tradfinal=tradfinal+"b",
			'c' => tradfinal=tradfinal+"c",
			'd' => tradfinal=tradfinal+"d",
			'e' => tradfinal=tradfinal+"e",
			'f' => tradfinal=tradfinal+"f",
			'g' => tradfinal=tradfinal+"g",
			'h' => tradfinal=tradfinal+"h",
			'i' => tradfinal=tradfinal+"i",
			'j' => tradfinal=tradfinal+"j",
			'k' => tradfinal=tradfinal+"k",
			'l' => tradfinal=tradfinal+"l",
			'm' => tradfinal=tradfinal+"m",
			'n' => tradfinal=tradfinal+"n",
			'o' => tradfinal=tradfinal+"o",
			'p' => tradfinal=tradfinal+"p",
			'q' => tradfinal=tradfinal+"q",
			'r' => tradfinal=tradfinal+"r",
			's' => tradfinal=tradfinal+"s",
			't' => tradfinal=tradfinal+"t",
			'u' => tradfinal=tradfinal+"u",
			'v' => tradfinal=tradfinal+"v",
			'w' => tradfinal=tradfinal+"w",
			'x' => tradfinal=tradfinal+"x",
			'y' => tradfinal=tradfinal+"y",
			'z' => tradfinal=tradfinal+"z",
			'A' => tradfinal=tradfinal+"A",
			'B' => tradfinal=tradfinal+"B",
			'C' => tradfinal=tradfinal+"C",
			'D' => tradfinal=tradfinal+"D",
			'E' => tradfinal=tradfinal+"E",
			'F' => tradfinal=tradfinal+"F",
			'G' => tradfinal=tradfinal+"G",
			'H' => tradfinal=tradfinal+"H",
			'I' => tradfinal=tradfinal+"I",
			'J' => tradfinal=tradfinal+"J",
			'K' => tradfinal=tradfinal+"K",
			'L' => tradfinal=tradfinal+"L",
			'M' => tradfinal=tradfinal+"M",
			'N' => tradfinal=tradfinal+"N",
			'O' => tradfinal=tradfinal+"O",
			'P' => tradfinal=tradfinal+"P",
			'Q' => tradfinal=tradfinal+"Q",
			'R' => tradfinal=tradfinal+"R",
			'S' => tradfinal=tradfinal+"S",
			'T' => tradfinal=tradfinal+"T",
			'U' => tradfinal=tradfinal+"U",
			'V' => tradfinal=tradfinal+"V",
			'W' => tradfinal=tradfinal+"W",
			'X' => tradfinal=tradfinal+"X",
			'Y' => tradfinal=tradfinal+"Y",
			'Z' => tradfinal=tradfinal+"Z",
			_ => tradfinal=tradfinal+" ",
		}
	}
	positionanalyse+=1;
	println!("CERTIFICAT NETTOYÉ: {:?}\n",tradfinal);

	//resultatsannalyse[positionanalyse]=&tradfinal;








	//récup des extensions
	let mut pos2=43;
	println!("pos2={}\ntaille={:?}",pos2,temp2[pos2]);
	let tailleid=temp2[pos2];
	pos2=tailleid as usize;
	pos2=pos2+43+4;
	position=(temp2[pos2]).into();
	let mut tailleext:u64=(position*LEPETIT).into();
	pos2=pos2+1;
	position=(temp2[pos2]).into();
	tailleext=tailleext+position;
	pos2=pos2+1;
	let mut pos3=tailleext as usize;
	pos3=pos3+pos2;
	let extensions=&temp2[pos2..pos3];
	println!("taille extentions: {}\n",pos3-pos2);
	println!("extensions= {:?}",extensions);

	//traitement des extensions
	let mut pointex:usize=0;
	let taille:usize=pos3-pos2;
	let mut grosse:usize=0;
	let mut petite:usize=0;
	while pointex < taille {
		positionanalyse+=1;
		match extensions[pointex] {
			0 => {pointex+=1;
				match extensions[pointex] {
					0 => resultatsannalyse[positionanalyse]=String::from("server name"),
					1 => resultatsannalyse[positionanalyse]=String::from("max fragment length"),
					2 => resultatsannalyse[positionanalyse]=String::from("client_certificate_url"),
					3 => resultatsannalyse[positionanalyse]=String::from("trusted_ca_keys"),
					4 => resultatsannalyse[positionanalyse]=String::from("truncated_hmac"),
					5 => resultatsannalyse[positionanalyse]=String::from("status_request"),
					6 => resultatsannalyse[positionanalyse]=String::from("user_mapping"),
					7 => resultatsannalyse[positionanalyse]=String::from("client_authz"),
					8 => resultatsannalyse[positionanalyse]=String::from("server_authz"),
					0 => resultatsannalyse[positionanalyse]=String::from("server name"),
					9 => resultatsannalyse[positionanalyse]=String::from("cert_type"),
					10 => resultatsannalyse[positionanalyse]=String::from("supported_groups (renamed from elliptic_curves)"),
					11 => resultatsannalyse[positionanalyse]=String::from("ec_point_formats"),
					12 => resultatsannalyse[positionanalyse]=String::from("srp"),
					13 => resultatsannalyse[positionanalyse]=String::from("signature_algorithms"),
					14 => resultatsannalyse[positionanalyse]=String::from("use_srtp"),
					15 => resultatsannalyse[positionanalyse]=String::from("heartbeat"),
					16 => resultatsannalyse[positionanalyse]=String::from("application_layer_protocol_negotiation"),
					17 => resultatsannalyse[positionanalyse]=String::from("status_request_v2"),
					18 => resultatsannalyse[positionanalyse]=String::from("signed_certificate_timestamp"),
					19 => resultatsannalyse[positionanalyse]=String::from("client_certificate_type"),
					20 => resultatsannalyse[positionanalyse]=String::from("server_certificate_type"),
					21 => resultatsannalyse[positionanalyse]=String::from("padding"),
					22 => resultatsannalyse[positionanalyse]=String::from("encrypt_then_mac"),
					23 => resultatsannalyse[positionanalyse]=String::from("extended_master_secret"),
					24 => resultatsannalyse[positionanalyse]=String::from("token_binding"),
					25 => resultatsannalyse[positionanalyse]=String::from("cached_info"),
					26 => resultatsannalyse[positionanalyse]=String::from("tls_lts"),
					27 => resultatsannalyse[positionanalyse]=String::from("compress_certificate"),
					28 => resultatsannalyse[positionanalyse]=String::from("record_size_limit"),
					29 => resultatsannalyse[positionanalyse]=String::from("pwd_protect"),
					30 => resultatsannalyse[positionanalyse]=String::from("pwd_clear"),
					31 => resultatsannalyse[positionanalyse]=String::from("password_salt"),
					32 => resultatsannalyse[positionanalyse]=String::from("ticket_pinning"),
					33 => resultatsannalyse[positionanalyse]=String::from("tls_cert_with_extern_psk"),
					34 => resultatsannalyse[positionanalyse]=String::from("delegated_credentials"),
					35 => resultatsannalyse[positionanalyse]=String::from("session_ticket (renamed from SessionTicket TLS)"),
					36 => resultatsannalyse[positionanalyse]=String::from("TLMSP"),
					37 => resultatsannalyse[positionanalyse]=String::from("TLMSP_proxying"),
					38 => resultatsannalyse[positionanalyse]=String::from("TLMSP_delegate"),
					39 => resultatsannalyse[positionanalyse]=String::from("supported_ekt_ciphers"),
					41 => resultatsannalyse[positionanalyse]=String::from("pre_shared_key"),
					42 => resultatsannalyse[positionanalyse]=String::from("early_data"),
					43 => resultatsannalyse[positionanalyse]=String::from("supported_versions"),
					44 => resultatsannalyse[positionanalyse]=String::from("cookie"),
					45 => resultatsannalyse[positionanalyse]=String::from("psk_key_exchange_modes"),
					47 => resultatsannalyse[positionanalyse]=String::from("certificate_authorities"),
					48 => resultatsannalyse[positionanalyse]=String::from("oid_filters"),
					49 => resultatsannalyse[positionanalyse]=String::from("post_handshake_auth"),
					50 => resultatsannalyse[positionanalyse]=String::from("signature_algorithms_cert"),
					51 => resultatsannalyse[positionanalyse]=String::from("key_share"),
					52 => resultatsannalyse[positionanalyse]=String::from("transparency_info"),
					53 => resultatsannalyse[positionanalyse]=String::from("connection_id (deprecated)"),
					54 => resultatsannalyse[positionanalyse]=String::from("connection_id"),
					55 => resultatsannalyse[positionanalyse]=String::from("external_id_hash"),
					56 => resultatsannalyse[positionanalyse]=String::from("external_session_id"),
					57 => resultatsannalyse[positionanalyse]=String::from("quic_transport_parameters"),
					58 => resultatsannalyse[positionanalyse]=String::from("ticket_request"),
					59 => resultatsannalyse[positionanalyse]=String::from("dnssec_chain"),
					_ => resultatsannalyse[positionanalyse]=String::from("erreur: extension non-reconnue par l'aiana"),
				}},
			255 => {pointex+=1; match extensions[pointex] {
				1 => resultatsannalyse[positionanalyse]=String::from("renegotiation_info"),
				_ => resultatsannalyse[positionanalyse]=String::from("erreur: extension non-reconnue par l'aiana"),
			}},
			_ => {resultatsannalyse[positionanalyse]=String::from("erreur: extension non-reconnue par l'aiana");pointex+=1;},
		}
		pointex+=1;
		grosse=extensions[pointex] as usize;
		println!("grosse= {}",grosse);
		grosse=grosse*65536;
		pointex+=1;
		petite=extensions[pointex] as usize;
		println!("petite= {}",petite);
		petite+=1;
		grosse=grosse+petite;
		//pointex+=1;
		pointex=pointex+grosse;
		println!("extensions= {:?}",resultatsannalyse[positionanalyse]);
		println!("{:#?}",resultatsannalyse);


	}

	//trouver les versions de TLS
	println!(" {:?}\n",resultatsannalyse);


	fin.certificat.push_str(&tradfinal);
	println!("clone={:?}",fin.certificat);
	let mut ind:usize=0;

	for i in resultatsannalyse.clone() {
		//println!("i={}",i);
		//let k=&i;
		for j in reserve1 {
			//println!("travail terminé");
			if i.eq(j) {
				fin.liste[ind].push_str(j);
				ind+=1;
				break;
			}
		}

	}
	fin.versions[0].push_str(&resultatsannalyse[1]);
	fin.versions[1].push_str(&resultatsannalyse[2]);
	fin.versions[2].push_str(&resultatsannalyse[3]);
	fin.versions[3].push_str(&resultatsannalyse[4]);

	//calcul de la note=place de la cypher la plus faible*la version TLS la plus faible
	let mut indexfaibl:usize=0;
	let mut note:u16=reserve1.len().try_into().unwrap();
	let mut suspect=String::new();
	positionanalyse=5 as usize;
	suspect.push_str(&resultatsannalyse[positionanalyse]);
	let mut portraitrobot=String::new();
	portraitrobot.push_str(&reserve1[indexfaibl]);


	//place de la cypher la plus faible
	while /*suspect.ne(&String::from("test"))*/ positionanalyse < 550 {
		while portraitrobot.ne(&suspect) && indexfaibl < (reserve1.len()-1) {
			indexfaibl+=1;
			portraitrobot.push_str(&reserve1[indexfaibl]);
		}
		if usize::from(note) > indexfaibl {
			note=indexfaibl as u16;
			fin.cyfaible.push_str(&reserve1[indexfaibl]);
		}
		indexfaibl=0 as usize;
		positionanalyse+=1;
		suspect.push_str(&resultatsannalyse[positionanalyse]);
	}
	//version tls la plus faible
	let a=String::from("TLS 1.0 supportée");
	let b=String::from("TLS 1.1 supportée");
	let c=String::from("TLS 1.2 supportée");
	let d=String::from("TLS 1.3 supportée");
	for i in &fin.versions {
		match &i {
			a => {note=note*0;break;},
			b => {note=note*1;break;},
			c => {note=note*2;break;},
			d => {note=note*3;break;},
			_ => (),
		}
	}

	fin.note=note;





	println!("cyfaible={:?}",fin.cyfaible);
	println!("fin.liste={:?}",fin.liste);
	println!("la note={:?}",fin.note);
	println!("ip={:?}",fin.ip);

	return fin;

}
