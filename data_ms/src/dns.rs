use std::clone;
use regex::Regex;
use serde::Serialize;
use crate::bimi::{bimi, BIMIRecord};
use crate::certificate::{certificat, Cert};
use crate::dane::{dane, DANERecord};
use crate::dmarc::{dmarc, DMARCRecord};
use crate::ipinfo::ip_info;
use crate::mta_sts::{mta, MTARecord};
use crate::spf::{spf, SPFRecord};
use crate::tls_rpt::{tls_rtp, TLSRecord};


/// # Brief
/// Structure de DNS
/// # Attributes
/// - domain *String* : nom de domaine
/// - dmarc *DMARCRecord* : structure DMARC
/// - spf *SPFRecord* : structure SPF
/// - dane *DANERecord* : structure DANE
/// - bimi *BIMIRecord* : structure BIMI
/// - mta *MTARecord* : structure MTA-STS
/// - note *String* : note de DNS
#[derive(Serialize, Debug)]
pub struct DNSRecord
{
	pub domain: String,
	pub dmarc: DMARCRecord,
	pub spf: SPFRecord,
	pub dane: DANERecord,
	pub bimi: BIMIRecord,
	pub mta: MTARecord,
	pub tls: TLSRecord,
	pub certificate: Cert,
	pub note: String,
}


pub(crate) fn dns(ip_domain: &str) -> DNSRecord
{
	#[allow(unused_assignments)]
		let mut domain = String::new();

	// Si l'input match avec l'expression régulière, c'est une adresse IP
	if Regex::new(r"^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$").unwrap().is_match(ip_domain) == true
	{
		domain = ip_info(ip_domain);
	}

	// Sinon, c'est un nom de domaine
	else
	{
		domain = ip_domain.to_string();
	}

	let mut n =  DNSRecord
	{
		domain: domain.clone(),
		dmarc: dmarc(domain.clone()),
		spf: spf(domain.clone()),
		dane: dane(domain.clone()),
		bimi: bimi(domain.clone()),
		mta: mta(domain.clone()),
		tls: tls_rtp(domain.clone()),
		certificate: certificat(domain.clone()),
		note: "0".to_string(),
	};

	let mut note = 0.0;
	let dmarc = n.dmarc.note.parse::<f64>().unwrap();
	let spf = n.spf.note.parse::<f64>().unwrap();
	let dane = n.dane.note.parse::<f64>().unwrap();
	let bimi = n.bimi.note.parse::<f64>().unwrap();
	let mta = n.mta.note.parse::<f64>().unwrap();
	let tls = n.tls.note.parse::<f64>().unwrap();
	let certificate = n.certificate.note.parse::<f64>().unwrap();

	note = ((dane + dmarc + mta + tls) / 4.0 * 2.0 + (bimi + certificate + spf) / 3.0) / 3.0;

	n.note = note.to_string();

	return n;
}