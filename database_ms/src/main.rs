use actix_cors::Cors;
use std::ops::Deref;
use actix_web::{get, post, error::ResponseError, web::Path, web::Json, web::Data, HttpResponse, http::{header::ContentType, StatusCode}, App, HttpServer, HttpRequest};
use serde::{Serialize, Deserialize, Serializer};
use serde_json::{json, to_string};
use std::fs;
use sqlx::*;
use std::thread::yield_now;
use std::thread::sleep;
use futures::StreamExt;
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
	pub note: String,
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
	pub note: String,
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
	pub note: String,
}

#[derive(Deserialize,Serialize, Encode, Type)]
pub struct DANERecord
{
	pub forme_certificat: String,
	pub signature_certificat: bool,
	pub signature_cle_publique: bool,
	pub presence_hash: bool,
	pub hash: String,
	pub note: String,
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
	pub note: String,
}

#[derive(Deserialize,Serialize, Encode, Type)]
pub struct MTARecord
{
	pub version: String,
	pub sn: String,
	pub note: String,
}

#[derive(Deserialize,Serialize, Encode, Type)]
pub struct TLSRecord
{
	pub v: String,
	pub rua: String,
	pub note: String,
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
	pub note: String,
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

		

		
		
		
#[get("/users")]
async fn rechercheutilisateur(req: HttpRequest) -> HttpResponse {
	println!("bougez-vous, j'arrive!");
	let requet = req.query_string();
    if requet.eq("") {
        return HttpResponse::Ok().body("rien reçu");
    }
    
    let mut caracteristique = String::new();
    let mut select = String::new();
    let mut controle = String::from("&");
    let mut flag=0;
    for i in requet.chars(){
        if i.eq(&'=') {
            controle = '='.to_string();
            caracteristique =  caracteristique + &i.to_string();
            flag=0;
        }
        else if i.eq(&'&') {
            if flag == 2 {
                caracteristique = caracteristique + &"\"".to_string();
            }
            controle = '&'.to_string();
            caracteristique =  caracteristique + &" AND ".to_string();
            select = select + &", ".to_string();
        }
        else if controle.eq("&") {
            select = select + &i.to_string();
            caracteristique =  caracteristique + &i.to_string();
        }
        else if controle.eq("="){
            if flag == 0 {
                if i.eq(&'0') || i.eq(&'1') || i.eq(&'2') || i.eq(&'3') || i.eq(&'4') || i.eq(&'5') || i.eq(&'6') || i.eq(&'7') || i.eq(&'8') || i.eq(&'9') || i.eq(&'T') {
                    flag=1;
                }
                else{
                    flag = 2;
                    caracteristique = caracteristique + &"\"".to_string();
                }
            }
                    
            caracteristique =  caracteristique + &i.to_string();
        }
        //println!("{select}");
    }
    if flag == 2 {
        caracteristique = caracteristique + &"\"".to_string();
    }
    //let mut ordre=select.clone();
    select = "SELECT ".to_string() + &select + &" FROM users WHERE ".to_string() + &caracteristique.clone();
        
        
    let mut conn = mysql::MySqlConnectOptions::new()
    	.host("mysql.default")
    	.username("ice_crawler_user")
    	.password("fuI0hwM9bKhf0NrtZpM08xadJ1YtUB0XyanSZykG")
    	.database("ice_crawler_DB")
    	.connect().await.expect("skill issue");
    
    let mut result = sqlx::query(requet)
        .fetch(&mut conn);
    
    
    
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

#[post("/users")]
async fn ajoututilisateur(req: Json<Utilisateur>) -> HttpResponse {
	println!("bougez-vous, j'arrive!");
	let test = req.hash.clone();
   	if test.eq("") {
        	return HttpResponse::Ok().body("error empty data structure!!");
    	}
    	
    	let mut requete=String::from("INSERT INTO users (mail, hash, sel) VALUES ( "); 
    	requete=requete + &req.mail.clone() + &",".to_string() + &req.hash.clone() + &",".to_string() + &req.sel.clone() + &");".to_string();
    	let mut pool = mysql::MySqlConnectOptions::new()
    		.host("mysql.default")
    		.username("ice_crawler_user")
    		.password("fuI0hwM9bKhf0NrtZpM08xadJ1YtUB0XyanSZykG")
    		.database("ice_crawler_DB")
    		.connect().await.expect("skill issue");
        
        
    	sqlx::query(requete.as_str())
        	.execute(&mut pool)
        	.await.expect("bruh");
       
       return HttpResponse::Ok().body("ok");
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
	let mut requete = String::from("SELECT * FROM servers INNER JOIN domains ON `servers.domaine`=`domains.domain` WHERE domains.domain=");
	let taille = test.len();
	let mut flag = 1;
	for i in test {
		if flag < taille {
			requete = requete + &"\"".to_string() + &i.clone() + &"\" OR domains.domain=".to_string();
			flag += 1;
		}
		else {
			requete = requete + &"\"".to_string() + &i.clone() + &"\" ;".to_string();
		}
	}
	
	let mut conn = mysql::MySqlConnectOptions::new()
    		.host("mysql.default")
    		.username("ice_crawler_user")
    		.password("fuI0hwM9bKhf0NrtZpM08xadJ1YtUB0XyanSZykG")
    		.database("ice_crawler_DB")
    		.connect().await.expect("defaut de connexion");
    
    	let mut result = sqlx::query(requete.as_str())
        		.fetch(&mut conn);
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

#[post("/cidr")]
async fn recupcidr(req: Json<Cidr>) -> HttpResponse {
	println!("bougez-vous, j'arrive!");
	let regex_cidr = Regex::new(r"^([0-9]{1,3}.){3}[0-9]{1,3}/(\d|[1-2]\d|3[0-2])$").unwrap();
	let mut cidr=req.CIDR.clone();
	if cidr.eq("") {
		return HttpResponse::Ok().body("rien reçu");
	}
	
	
	let cidre = cidr_notation(cidr.as_str());
	
				
	let mut requete = String::from("SELECT * FROM servers INNER JOIN domains ON `servers.domaine`=`domains.domain` WHERE servers.ip >= \"");
	requete=requete + &cidre.debut.clone() + &"\" AND servers.ip <= \"".to_string() + &cidre.fin.clone() + &"\";".to_string();
		
	println!("requete={}",requete);
	
	let mut conn = mysql::MySqlConnectOptions::new()
    	.host("mysql.default")
    	.username("ice_crawler_user")
    	.password("fuI0hwM9bKhf0NrtZpM08xadJ1YtUB0XyanSZykG")
    	.database("ice_crawler_DB")
    	.connect().await.expect("defaut de connexion");
    
    	let mut result = sqlx::query(requete.as_str())
        	.fetch(&mut conn);
    	
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
    
    //let mut requete=String::from("INSERT INTO servers (`ip`, `domaine`, `tls.certificat`, `tls.liste`, `tls.cyfaible`, `tls.starttls`, `tls.note`) VALUES ( 192.168.22.223, google.com, hthqeh, qehteht, thrth, Sgrgge, drhrhrh)");
    let mut requete=String::from("INSERT INTO servers (`ip`, `domaine`, `tls.certificat`, `tls.liste`, `tls.cyfaible`, `tls.starttls`, `tls.note`) VALUES ( \" ");
    requete=requete + &req.tls.ip.clone() + &&"\", \"".to_string() + &req.dns.domain.clone() + &"\", \"".to_string() + &req.tls.certificat.clone() + &"\", \"".to_string();
    for i in &req.tls.liste {
    	requete=requete + &i.clone();
    }
    
    requete=requete + &"\", \"".to_string() + &req.tls.cyfaible.clone() + &"\", \"".to_string() + &req.tls.starttls.clone() + &"\", \"".to_string() + &req.tls.note.to_string() + &"\");".to_string();
    
    let mut requete2=String::from("INSERT INTO domains (`domain`, `bimi.version`, `bimi.url_expediteur`, `bimi.url_politique`, `bimi.url_reputation`, `bimi.hash`, `bimi.s`, `certificate.signature_algorithm_server`, `certificate.IssuerDetails.city`, `certificate.IssuerDetails.state`, `certificate.IssuerDetails.locality`, `certificate.IssuerDetails.organization`, `certificate.IssuerDetails.common_name`, `certificate.ValidityDetails.not_before`, `certificate.ValidityDetails.not_after`, `certificate.ValidityDetails.is_valid`, `certificate.SubjectDetails.city`, `certificate.SubjectDetails.state`, `certificate.SubjectDetails.locality`, `certificate.SubjectDetails.organization`, `certificate.SubjectDetails.common_name`, `certificate.ExtensionDetails.subject_alternative_names`, `certificate.signature_algorithm_intermediate`, `certificate.issuer_intermediate.city`, `certificate.issuer_intermediate.state`, `certificate.issuer_intermediate.locality`, `certificate.issuer_intermediate.oragnization`, `certificate.issuer_intermediate.common_name`, `certificate.issuer_intermediate.not_before`, `certificate.issuer_intermediate.not_after`, `certificate.validity_intermediate.city`, `certificate.subject_intermediate.city`, `certificate.subject_intermediate.state`, `certificate.subject_intermediate.locality`, `certificate.subject_intermediate.organization`, `certificate.subject_intermediate.common_name`, `certificate.extensions_intermediate.is_valid`, `dane.forme_certificat`, `dane.signature_certificat`, `dane.signature_cle_publique`, `dane.presence_hash`, `dane.hash`, `dmarc.v`, `dmarc.p`, `dmarc.sp`, `dmarc.pct`, `dmarc.ruf`, `dmarc.rua`, `dmarc.ri`, `dmarc.rf`, `dmarc.aspf`, `dmarc.adkim`, `dmarc.fo`, `mta_sts.verion`, `mata_sts.sn`, `spf.version`, `spf.mechanisms`, `spf.qualifier`, `spf.ip`, `spf.include`, `spf.all`, `tls_rpt.v`, `tls_rpt.rua`) VALUES ( ");
    
    requete2=requete2 + &req.dns.domain.clone() + &"\", \"".to_string() + &req.dns.bimi.version.clone() + &"\", \"".to_string() + &req.dns.bimi.url_expediteur.clone() + &"\", \"".to_string() + &req.dns.bimi.url_politique.clone() + &"\", \"".to_string() + &req.dns.bimi.url_reputation.clone() + &"\", \"".to_string() + &req.dns.bimi.hash.clone() + &"\", \"".to_string() + &req.dns.bimi.s.clone() + &"\", \"".to_string() + &req.dns.certificate.signature_algorithm_server.clone() + &"\", \"".to_string() + &req.dns.certificate.issuer_server.city.clone() + &"\", \"".to_string() + &req.dns.certificate.issuer_server.state.clone() + &"\", \"".to_string() + &req.dns.certificate.issuer_server.locality.clone() + &"\", \"".to_string() + &req.dns.certificate.issuer_server.organization.clone() + &"\", \"".to_string() + &req.dns.certificate.issuer_server.common_name.clone() + &"\", \"".to_string() + &req.dns.certificate.validity_server.not_before.to_string() + &"\", \"".to_string() + &req.dns.certificate.validity_server.not_after.to_string() + &"\", \"".to_string() + &req.dns.certificate.validity_server.is_valid.to_string() + &"\", \"".to_string() + &req.dns.certificate.subject_server.city.clone() + &"\", \"".to_string() + &req.dns.certificate.subject_server.state.clone() + &"\", \"".to_string() + &req.dns.certificate.subject_server.locality.clone() + &"\", \"".to_string() + &req.dns.certificate.subject_server.organization.clone() + &"\", \"".to_string() + &req.dns.certificate.subject_server.common_name.clone() + &"\", \"".to_string();
    
    
    for i in &req.dns.certificate.extensions_server.subject_alternative_names{
    	requete2=requete2 + &i;
    }
    
    requete2=requete2 + &"\", \"".to_string() + &req.dns.certificate.signature_algorithm_intermediate.clone() + &"\", \"".to_string() + &req.dns.certificate.issuer_intermediate.city.clone() + &"\", \"".to_string() + &req.dns.certificate.issuer_intermediate.state.clone() + &"\", \"".to_string() + &req.dns.certificate.issuer_intermediate.locality.clone() + &"\", \"".to_string() + &req.dns.certificate.issuer_intermediate.organization.clone() + &"\", \"".to_string() + &req.dns.certificate.issuer_intermediate.common_name.clone() + &"\", \"".to_string() + &req.dns.certificate.validity_intermediate.not_before.to_string() + &"\", \"".to_string() + &req.dns.certificate.validity_intermediate.not_after.to_string() + &"\", \"".to_string() + &req.dns.certificate.validity_intermediate.is_valid.to_string() + &"\", \"".to_string() + &req.dns.certificate.subject_intermediate.city.clone() + &"\", \"".to_string() + &req.dns.certificate.subject_intermediate.state.clone() + &"\", \"".to_string() + &req.dns.certificate.subject_intermediate.locality.clone() + &"\", \"".to_string() + &req.dns.certificate.subject_intermediate.organization.clone() + &"\", \"".to_string() + &req.dns.certificate.validity_intermediate.is_valid.to_string() + &"\", \"".to_string() + &req.dns.dane.forme_certificat.clone() + &"\", \"".to_string() + &req.dns.dane.signature_certificat.to_string() + &"\", \"".to_string() + &req.dns.dane.signature_cle_publique.to_string() + &"\", \"".to_string() + &req.dns.dane.presence_hash.to_string() + &"\", \"".to_string() + &req.dns.dane.hash.clone() + &"\", \"".to_string() + &req.dns.dmarc.v.clone() + &"\", \"".to_string() + &req.dns.dmarc.p.clone() + &"\", \"".to_string() + &req.dns.dmarc.sp.clone() + &"\", \"".to_string() + &req.dns.dmarc.pct.clone() + &"\", \"".to_string() + &req.dns.dmarc.ruf.clone() + &"\", \"".to_string() + &req.dns.dmarc.rua.clone() + &"\", \"".to_string() + &req.dns.dmarc.ri.clone() + &"\", \"".to_string() + &req.dns.dmarc.rf.clone() +&"\", \"".to_string() + &req.dns.dmarc.aspf.clone() + &"\", \"".to_string() + &req.dns.dmarc.adkim.clone() + &"\", \"".to_string() + &req.dns.dmarc.fo.clone() + &"\", \"".to_string() + &req.dns.mta.version.clone() + &"\", \"".to_string() + &req.dns.mta.sn.clone() + &"\", \"".to_string() + &req.dns.spf.version.clone() + &"\", \"".to_string();


    for i in &req.dns.spf.mechanisms {
    	requete2=requete2 + &i;
    }

    requete2=requete2 + &"\", \"".to_string() + &req.dns.spf.qualifier.clone() + &"\", \"".to_string();


for i in &req.dns.spf.ip {
    	requete2=requete2 + &i;
}

requete2=requete2 + &"\", \"".to_string();


for i in &req.dns.spf.include {
    	requete2=requete2 + &i;
}


requete2=requete2 +&"\", \"".to_string() + &req.dns.spf.all.clone() + &"\", \"".to_string() + &req.dns.tls.v.clone() + &"\", \"".to_string() + &req.dns.tls.rua.clone() + &"\"); ".to_string();
    
    let mut pool = mysql::MySqlConnectOptions::new()
    	.host("mysql.default")
    	.username("ice_crawler_user")
    	.password("fuI0hwM9bKhf0NrtZpM08xadJ1YtUB0XyanSZykG")
    	.database("ice_crawler_DB")
    	.connect().await.expect("skill issue");
	println!("requete={}",requete);
	println!("requete2={}",requete2);
        
        
    sqlx::query(requete.as_str())
        .execute(&mut pool)
        .await.expect("bruh1");
        
    sqlx::query(requete2.as_str())
        .execute(&mut pool)
        .await.expect("bruh2");

    
	return HttpResponse::Ok().body("ok");
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

