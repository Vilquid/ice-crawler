#[allow(unused_imports)]
use actix_cors::Cors;
#[allow(unused_imports)]
	use std::ops::Deref;
#[allow(unused_imports)]
use actix_web::{get, post, error::ResponseError, web::Path, web::Json, web::Data, HttpResponse, http::{header::ContentType, StatusCode}, App, HttpServer, HttpRequest};
#[allow(unused_imports)]
use serde::{Serialize, Deserialize, Serializer};
#[allow(unused_imports)]
use serde_json::{json, to_string};
#[allow(unused_imports)]
use std::fs;
use sqlx::*;
#[allow(unused_imports)]
use std::thread::yield_now;
#[allow(unused_imports)]
use std::thread::sleep;
use futures::StreamExt;
#[allow(unused_imports)]
use sqlx::mysql::MySqlRow;
use std::net::{IpAddr, Ipv4Addr};
use regex::Regex;



#[derive(Deserialize,Serialize, Encode, Type)]
pub struct DATAResult
{
	pub dns: DNSRecord,
	pub tls: Retour,
}


#[derive(Deserialize,Serialize, Encode, Type)]
pub struct Retour{
	certificat: String,
	liste: Vec<String>,
	cyfaible: String,
	starttls: String,
	versions: [String;4],
	note: u16,
	ip: String,
}



#[derive(Deserialize,Serialize, Encode, Type)]
pub struct DNSRecord
{
	pub domain: String,
	pub dmarc: DMARCRecord,
	pub spf: SPFRecord,
	pub dane: DANERecord,
	pub bimi: BIMIRecord,
	pub mta: MTARecord,
	pub tls: TLSRecord,
	pub certificate: CertificateRecord,
	pub note: f32,
}

#[derive(Deserialize,Serialize, Encode, Type)]
pub struct DMARCRecord
{
	pub v: String,
	pub p: String,
	pub sp: String,
	pub pct: String,
	pub ruf: String,
	pub rua: String,
	pub ri: String,
	pub rf: String,
	pub aspf: String,
	pub adkim: String,
	pub fo: String,
	pub note: f32,
}


#[derive(Deserialize,Serialize, Encode, Type)]
pub struct SPFRecord
{
	pub version: String,
	pub mechanisms: Vec<String>,
	pub qualifier: String,
	pub ip: Vec<String>,
	pub include: Vec<String>,
	pub all: String,
	pub note: f32,
}

#[derive(Deserialize,Serialize, Encode, Type)]
pub struct DANERecord
{
	pub forme_certificat: String,
	pub signature_certificat: bool,
	pub signature_cle_publique: bool,
	pub presence_hash: bool,
	pub hash: String,
	pub note: f32,
}

#[derive(Deserialize,Serialize, Encode, Type)]
pub struct BIMIRecord
{
	pub version: String,
	pub url_expediteur: String,
	pub url_politique: String,
	pub url_reputation: String,
	pub hash: String,
	pub s: String,
	pub note: f32,
}

#[derive(Deserialize,Serialize, Encode, Type)]
pub struct MTARecord
{
	pub version: String,
	pub sn: String,
	pub note: f32,
}

#[derive(Deserialize,Serialize, Encode, Type)]
pub struct TLSRecord
{
	pub v: String,
	pub rua: String,
	pub note: f32,
}

#[derive(Deserialize,Serialize, Encode, Type)]
pub struct CertificateRecord
{
	pub domain: String,
	pub signature_algorithm_server: String,
	pub issuer_server: IssuerDetails,
	pub validity_server: ValidityDetails,
	pub subject_server: SubjectDetails,
	pub extensions_server: ExtensionsDetails,
	pub signature_algorithm_intermediate: String,
	pub issuer_intermediate: IssuerDetails,
	pub validity_intermediate: ValidityDetails,
	pub subject_intermediate: SubjectDetails,
	pub extensions_intermediate: ExtensionsDetails,
	pub note: f32,
}

#[derive(Deserialize,Serialize, Encode, Type)]
pub struct IssuerDetails
{
	pub city: String,
	pub state: String,
	pub locality: String,
	pub organization: String,
	pub common_name: String,
}

#[derive(Deserialize,Serialize, Encode, Type)]
pub struct SubjectDetails
{
	pub city: String,
	pub state: String,
	pub locality: String,
	pub organization: String,
	pub common_name: String,
}

#[derive(Deserialize,Serialize, Encode, Type)]
pub struct ValidityDetails
{
	pub not_before: String,
	pub not_after: String,
	pub is_valid: bool,
}

#[derive(Deserialize,Serialize, Encode, Type)]
pub struct ExtensionsDetails
{
	pub subject_alternative_names: Vec<String>,
}




#[derive(Serialize, Deserialize, Encode, Type)]
pub struct Re<T>{
	resultat: Vec<T>
}


#[derive(Serialize, Deserialize, Encode, Type)]
pub struct Utilisateur{
	pub mail: String,
	pub hash: String,
	pub sel: String,
}

#[derive(Serialize, Deserialize, Encode, Type)]
pub struct Cidr{
	CIDR: String,
}

#[derive(Serialize, Deserialize, Encode, Type)]
pub struct Domaine{
	domain: Vec<String>,
}
		
#[derive(Serialize, Deserialize, Encode, Type)]
pub struct IpRange
{
	pub debut: String,
	pub fin: String,
}

		

		
		
		



fn parse_cidr(cidr: &str) -> Result<(IpAddr, u8)>
{
	let parts: Vec<&str> = cidr.split("/").collect();
	let regex_cidr = Regex::new(r"^([0-9]{1,3}.){3}[0-9]{1,3}/(\d|[1-2]\d|3[0-2])$").unwrap();

	// Vérification de la validité de la notation CIDR
	if parts.len() != 2 || !regex_cidr.is_match(cidr)
	{
		return parse_cidr("192.168.0.0/24")
	}

	// Parsing de cidr
	let ip = parts[0].parse::<IpAddr>().unwrap();
	let bits = parts[1].parse::<u8>().unwrap();

	Ok((ip, bits))
}

pub(crate) fn cidr_notation(cidr: &str) -> IpRange
{
	let (ip, bits) = parse_cidr(cidr).unwrap();

	// Gestion du cas où cidr se termine par /0
	return if bits == 0
	{
		IpRange
		{
			debut: "0.0.0.0".to_string(),
			fin: "255.255.255.255".to_string(),
		}
	} else {
		// Initialisation de la variable ip_range
		let mut ip_range = IpRange
		{
			debut: "vide".to_string(),
			fin: "vide".to_string(),
		};

		// Calcul de la première adresse IP du réseau
		match ip
		{
			IpAddr::V4(ip) => {
				let mask = (1u32 << (32 - bits)) - 1;
				let network = u32::from(ip) & !mask;

				ip_range.debut = IpAddr::V4(Ipv4Addr::from(network)).to_string();
			}

			_ => {}
		}

		// Calcul de la dernière adresse IP du réseau
		match ip
		{
			IpAddr::V4(ip) => {
				let mask = (1u32 << (32 - bits)) - 1;
				let network = u32::from(ip) | mask;

				ip_range.fin = IpAddr::V4(Ipv4Addr::from(network)).to_string();
			}

			_ => {}
		}

		ip_range
	}
}


#[post("/domaine")]
async fn recupdomain(req: Json<Domaine>) -> HttpResponse {
	let test = req.domain.clone();
	if test[0].eq("") {
		return HttpResponse::Ok().body("rien reçu");
	}
	println!("test={:?}",test);
	let mut pool = mysql::MySqlConnectOptions::new()
    	.host("mysql.default")
    	.username("ice_crawler_user")
    	.password("fuI0hwM9bKhf0NrtZpM08xadJ1YtUB0XyanSZykG")
    	.database("ice_crawler_DB")
    	.connect().await.expect("defaut de connexion");
        
        
        let mut tamp=Vec::new();
	let mut reponse=String::new();
		
        for i in &req.domain {
		let mut domaine = i.clone();
		let mut domaine2 = i.clone();
		
	    
	    let mut result = sqlx::query("SELECT * FROM servers INNER JOIN domains WHERE `servers`.`domaine` = `domains`.`domain` AND `servers`.`domaine` = ?;")
	    	.bind(&domaine)
		.bind(&domaine2)
	    	.fetch(&mut pool);
		
	    println!("result={:?}",result);
	    	while let Some(row) = result.next().await {
	    	
			tamp.push(row.expect("mais voilà c'était sûr en fait!"));
	    	}
	    	for j in &tamp{
	    		for i in j.columns(){
	    			reponse=reponse + j.get(i.ordinal());
				println!("reponse={:?}",reponse);
	    		}
	    	
	    	}
    	}
    	let retour = Re{resultat: vec![reponse]};
    	
    	let renvoi = serde_json::to_string(&retour);
    	
    	return HttpResponse::Ok().body(renvoi.expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAh!!!!").clone());
	

}

#[post("/cidr")]
async fn recupcidr(req: Json<Cidr>) -> HttpResponse {
	println!("bougez-vous, j'arrive!");
	let regex_cidr = Regex::new(r"^([0-9]{1,3}.){3}[0-9]{1,3}/(\d|[1-2]\d|3[0-2])$").unwrap();
	let mut cidr=req.CIDR.clone();
	if cidr.eq("") {
		return HttpResponse::Ok().body("rien reçu");
	}
	
	
	let cidre = cidr_notation(cidr.as_str());
	
	
	
	
	let mut pool = mysql::MySqlConnectOptions::new()
    	.host("mysql.default")
    	.username("ice_crawler_user")
    	.password("fuI0hwM9bKhf0NrtZpM08xadJ1YtUB0XyanSZykG")
    	.database("ice_crawler_DB")
    	.connect().await.expect("defaut de connexion");
        
        
        let mut debut = cidre.debut.clone();
        let mut fin = cidre.fin.clone();
        
    
    let mut result = sqlx::query("SELECT * FROM servers INNER JOIN domains WHERE `servers`.`ip` >= ? AND `servers`.`ip` <= ?;")
    	.bind(&debut)
	.bind(&fin)
    	.fetch(&mut pool);
        
    let mut tamp=Vec::new();
	let mut reponse=String::new();
    	while let Some(row) = result.next().await {
    	
        	tamp.push(row.expect("mais voilà c'était sûr en fait!"));
    	}
    	for j in tamp{
    		for i in j.columns(){
    			reponse=reponse + j.get(i.ordinal());
    		}
    	
    	}
    	let retour = Re{resultat: vec![reponse]};
    	
    	let renvoi = serde_json::to_string(&retour);
    	
    	return HttpResponse::Ok().body(renvoi.expect("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAh!!!!").clone());
}


#[post("/")]
async fn admission(req: Json<DATAResult>) -> HttpResponse {
	println!("bougez-vous, j'arrive!");
    let test = req.tls.ip.clone();
    if test.eq("") {
        return HttpResponse::Ok().body("error empty data structure!!");
    }
    
    
    
    let mut tlsliste=String::new();
    
    for i in &req.tls.liste {
    	tlsliste = tlsliste + &", ".to_string() + &i;
    }
    
    let mut altnames=String::new();
    
    for i in &req.dns.certificate.extensions_server.subject_alternative_names {
    	altnames = altnames + &", ".to_string() + &i;
    }
    
    
    
    let mut altnames2=String::new();
    
    for i in &req.dns.certificate.extensions_intermediate.subject_alternative_names {
    	altnames2 = altnames2 + &", ".to_string() + &i;
    }
    
    let mut mechspf=String::new();
    
    for i in &req.dns.spf.mechanisms {
    	mechspf = mechspf + &", ".to_string() + &i;
    }
    
    
    
    let mut spfip=String::new();
    
    for i in &req.dns.spf.ip {
    	spfip = spfip + &", ".to_string() + &i;
    }
    
    
    let mut spfinc=String::new();
    
    for i in &req.dns.spf.include {
    	spfinc = spfinc + &", ".to_string() + &i;
    }
    
    
    
    let mut altnames3=String::new();
    
    for i in &req.dns.certificate.extensions_intermediate.subject_alternative_names {
    	altnames3 = altnames3 + &", ".to_string() + &i;
    }
    
    
    let mut spfip2=String::new();
    
    for i in &req.dns.spf.ip {
    	spfip2 = spfip2 + &", ".to_string() + &i;
    }
    
    
    
    let mut spfmech2=String::new();
    
    for i in &req.dns.spf.mechanisms {
    	spfmech2 = spfmech2 + &", ".to_string() + &i;
    }
    
    
    let mut spfinc2=String::new();
    
    for i in &req.dns.spf.include {
    	spfinc2 = spfinc2 + &", ".to_string() + &i;
    }
    
    
    
    
    
    
    /*let pool = mysql::MySqlPoolOptions::new()
        .connect("mysql://ice_crawler_user:fuI0hwM9bKhf0NrtZpM08xadJ1YtUB0XyanSZykG@mysql.default")
        .await;*/
        
      
      
      let mut pool = mysql::MySqlConnectOptions::new()
    	.host("mysql.default")
    	.username("ice_crawler_user")
    	.password("fuI0hwM9bKhf0NrtZpM08xadJ1YtUB0XyanSZykG")
    	.database("ice_crawler_DB")
    	.connect().await.expect("defaut de connexion");
        
        
        
    
    match sqlx::query("INSERT INTO servers (`ip`, `domaine`, `tls.certificat`, `tls.liste`, `tls.cyfaible`, `tls.starttls`, `tls.note`) VALUES ( ?,?,?,?,?,?,? ); INSERT INTO domains (`domain`,`note`,`bimi.version`,`bimi.url_expediteur`,`bimi.url_politique`,`bimi.url_reputation`,`bimi.hash`,`bimi.s`,`certificate.signature_algorithm_server`,`certificate.IssuerDetails.city`,`certificate.IssuerDetails.state`,`certificate.IssuerDetails.locality`,`certificate.IssuerDetails.organization`,`certificate.IssuerDetails.common_name`,`certificate.ValidityDetails.not_before`,`certificate.ValidityDetails.not_after`,`certificate.ValidityDetails.is_valid`,`certificate.SubjectDetails.city`,`certificate.SubjectDetails.state`,`certificate.SubjectDetails.locality`,`certificate.SubjectDetails.organization`,`certificate.SubjectDetails.common_name`,`certificate.ExtensionsDetails.subject_alternative_names`,`certificate.signature_algorithm_intermediate`,`certificate.issuer_intermediate.city`,`certificate.issuer_intermediate.state`,`certificate.issuer_intermediate.locality`,`certificate.issuer_intermediate.organization`,`certificate.issuer_intermediate.common_name`,`certificate.validity_intermediate.not_before`,`certificate.validity_intermediate.not_after`,`certificate.validity_intermediate.is_valid`,`certificate.subject_intermediate.city`,`certificate.subject_intermediate.state`,`certificate.subject_intermediate.locality`,`certificate.subject_intermediate.organization`,`certificate.subject_intermediate.common_name`,`certificate.extensions_intermediate.subject_alternative_names`,`dane.forme_certificat`,`dane.signature_certificat`,`dane.signature_cle_publique`,`dane.presence_hash`,`dane.hash`,`dmarc.v`,`dmarc.p` ,`dmarc.sp`,`dmarc.pct`,`dmarc.ruf` ,`dmarc.rua`,`dmarc.ri`,`dmarc.rf`,`dmarc.aspf`,`dmarc.adkim`,`dmarc.fo`,`mta_sts.version`,`mta_sts.sn`,`spf.version`,`spf.mechanisms`,`spf.qualifier`,`spf.ip`,`spf.include` ,`spf.all` ,`tls_rpt.v`,`tls_rpt.rua`,`bimi.note`,`certificate.note`,`dane.note` ,`dmarc.note` ,`mta_sts.note`,`spf.note`,`tls_rpt.note`) VALUES ( ?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,? );")
    	.bind(&req.tls.ip.clone())
        .bind(&req.dns.domain.clone())
        .bind(&req.tls.certificat.clone())
        .bind(&tlsliste.clone())
        .bind(&req.tls.cyfaible.clone())
        .bind(&req.tls.starttls.clone())
        .bind(&req.tls.note)
    	.bind(&req.dns.domain.clone())
    	.bind(&req.dns.note.to_string())
    	.bind(&req.dns.bimi.version.clone())
    	.bind(&req.dns.bimi.url_expediteur.clone())
    	.bind(&req.dns.bimi.url_politique.clone())
    	.bind(&req.dns.bimi.url_reputation.clone())
    	.bind(&req.dns.bimi.hash.clone())
    	.bind(&req.dns.bimi.s.clone())
    	.bind(&req.dns.certificate.signature_algorithm_server.clone())
    	.bind(&req.dns.certificate.issuer_intermediate.city.clone())
    	.bind(&req.dns.certificate.issuer_intermediate.state.clone())
    	.bind(&req.dns.certificate.issuer_intermediate.locality.clone())
    	.bind(&req.dns.certificate.issuer_intermediate.organization.clone())
    	.bind(&req.dns.certificate.issuer_intermediate.common_name.clone())
    	.bind(&req.dns.certificate.validity_intermediate.not_before.clone())
    	.bind(&req.dns.certificate.validity_intermediate.not_after.clone())
    	.bind(&req.dns.certificate.validity_intermediate.is_valid.to_string())
    	.bind(&req.dns.certificate.subject_intermediate.city.clone())
    	.bind(&req.dns.certificate.subject_intermediate.state.clone())
    	.bind(&req.dns.certificate.subject_intermediate.locality.clone())
    	.bind(&req.dns.certificate.subject_intermediate.organization.clone())
    	.bind(&req.dns.certificate.subject_intermediate.common_name.clone())
    	.bind(&altnames.clone())
    	.bind(&req.dns.certificate.signature_algorithm_intermediate.clone())
    	.bind(&req.dns.certificate.issuer_intermediate.city.clone())
    	.bind(&req.dns.certificate.issuer_intermediate.state.clone())
    	.bind(&req.dns.certificate.issuer_intermediate.locality.clone())
    	.bind(&req.dns.certificate.issuer_intermediate.organization.clone())
    	.bind(&req.dns.certificate.issuer_intermediate.common_name.clone())
    	.bind(&altnames2.clone())
    	.bind(&req.dns.certificate.validity_intermediate.not_after.clone())
    	.bind(&spfip.to_string())
    	.bind(&spfinc.clone())
    	.bind(&req.dns.certificate.subject_intermediate.state.clone())
    	.bind(&req.dns.certificate.subject_intermediate.locality.clone())
    	.bind(&req.dns.certificate.subject_intermediate.organization.clone())
    	.bind(&req.dns.certificate.subject_intermediate.common_name.clone())
    	.bind(&altnames3.clone())
    	.bind(&req.dns.dane.forme_certificat.clone())
    	.bind(&req.dns.dane.signature_certificat.to_string())
    	.bind(&req.dns.dane.signature_cle_publique.to_string())
    	.bind(&req.dns.dane.presence_hash.to_string())
    	.bind(&req.dns.dane.hash.clone())
    	.bind(&req.dns.dmarc.v.clone())
    	.bind(&req.dns.dmarc.p.clone())
    	.bind(&mechspf.clone())
    	.bind(&req.dns.dmarc.pct.clone())
    	.bind(&req.dns.dmarc.ruf.clone())
    	.bind(&req.dns.dmarc.rua.clone())
    	.bind(&req.dns.dmarc.ri.clone())
    	.bind(&req.dns.dmarc.rf.clone())
    	.bind(&req.dns.dmarc.aspf.clone())
    	.bind(&req.dns.dmarc.adkim.clone())
    	.bind(&req.dns.dmarc.fo.clone())
    	.bind(&req.dns.mta.version.clone())
    	.bind(&req.dns.mta.sn.clone())
    	.bind(&req.dns.spf.version.clone())
    	.bind(&spfmech2.clone())
    	.bind(&req.dns.spf.qualifier.clone())
    	.bind(&spfip2.clone())
    	.bind(&spfinc2.clone())
    	.bind(&req.dns.spf.all.clone())
    	.bind(&req.dns.tls.v.clone())
    	.bind(&req.dns.tls.rua.clone())
    	.bind(&req.dns.bimi.note.to_string())
    	.bind(&req.dns.certificate.note.to_string())
    	.bind(&req.dns.dane.note.to_string())
    	.bind(&req.dns.dmarc.note.to_string())
    	.bind(&req.dns.mta.note.to_string())
    	.bind(&req.dns.spf.note.to_string())
    	.bind(&req.dns.tls.note.to_string())
    	.execute(&mut pool)
        .await {
        Ok(_) => {
            return HttpResponse::Ok().body("ok");
        }
        Err(err) => {
            return HttpResponse::Ok().body("c'est la merde!!!!");
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
	
    HttpServer::new( move || {
	let cors = Cors::default().allow_any_origin().send_wildcard();
	App::new()
		.wrap(cors)
    		.service(admission)
            	.service(recupcidr)
            	.service(recupdomain)
            	.service(ajoututilisateur)
            	.service(rechercheutilisateur)
    })
    .bind(("0.0.0.0", 9009)).expect("REASON")
    .run()
    .await
    
}


