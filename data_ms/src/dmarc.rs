use std::process::Command;
use std::str::from_utf8;
use serde::Serialize;


/// # Brief
/// Structure DMARC
/// # Attributes
/// - v *i32* : version de DMARC
/// - p *String* : politique de traitement demandée
/// - sp *String* : politique de traitement demandée pour les sous-domaines
/// - pct *i32* :
/// - ruf *String* :
/// - rua *String* :
/// - ri *String* :
/// - rf *String* : format à utiliser pour les rapports d’informations légales spécifiques du message (liste de valeurs en texte clair séparées par des virgules)
/// - adkim *String* : mode d’alignement pour DKIM
/// - aspf *String* : mode d’alignement pour SPF
/// - fo *String* :
/// - note *String* : note de DMARC
/// # Warning
/// Cette structure n'est pas complète
#[derive(Serialize, Debug)]
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


pub(crate) fn dmarc(domain: String) -> DMARCRecord
{
	// Exécute dig et récupère sa sortie
	let output = Command::new("dig")
		.arg("_dmarc.".to_string() + &domain)
		.arg("TXT")
		.output()
		.expect("Failed to execute dig");

	// Convertit la sortie de dig en chaîne de caractères
	// let output_str = str::from_utf8(&output.stdout).unwrap();

	let mut dmarc_record = String::new();

	for line in from_utf8(&output.stdout).unwrap().lines()
	{
		if line.starts_with("_dmarc.")
		{
			dmarc_record = line[20..].to_string();
		}
	}

	let mut dmarc = DMARCRecord
	{
		v: "vide".to_string(),
		p: "vide".to_string(),
		sp: "vide".to_string(),
		pct: "vide".to_string(),
		ruf: "vide".to_string(),
		rua: "vide".to_string(),
		ri: "vide".to_string(),
		rf: "vide".to_string(),
		aspf: "vide".to_string(),
		adkim: "vide".to_string(),
		fo: "vide".to_string(),
		note: 0.0,
	};

	if dmarc_record.is_empty()
	{
		return dmarc;
	}

	let s = dmarc_record;
	let v_index = s.find("v=").unwrap();
	let result = &s[v_index..];
	let parts: Vec<&str> = result.split("; ").collect();

	for part in parts
	{
		let key_value: Vec<&str> = part.split("=").collect();

		if key_value.len() != 2
		{
			continue;
		}

		let key = key_value[0];
		let value = key_value[1];

		match key
		{
			"v" => dmarc.v = value.to_string(),
			"p" => dmarc.p = value.to_string(),
			"sp" => dmarc.sp = value.to_string(),
			"rua" => dmarc.rua = value.to_string(),
			"ruf" => dmarc.ruf = value.to_string(),
			"ri" => dmarc.ri = value.to_string(),
			"rf" => dmarc.rf = value.to_string(),
			"pct" => dmarc.pct = value.to_string(),
			"aspf" => dmarc.aspf = value.to_string(),
			"adkim" => dmarc.adkim = value.to_string(),
			"fo" => dmarc.fo = value.to_string(),
			_ => (),
		}
	}

	dmarc.rua = dmarc.rua.trim_matches('\"').trim().to_string();
	dmarc.rua = dmarc.rua.trim_matches(';').trim().to_string();
	dmarc.ruf = dmarc.ruf.trim_matches('\"').trim().to_string();
	dmarc.ruf = dmarc.ruf.trim_matches(';').trim().to_string();
	dmarc.fo = dmarc.fo.trim_matches('\"').trim().to_string();

	let mut note = 0.0;

	if dmarc.v != "vide"
	{
		note += 2.75;
	}
	
	if dmarc.p != "vide"
	{
		note += 2.75;
	}

	if dmarc.sp != "vide"
	{
		note += 0.5;
	}

	if dmarc.pct != "vide"
	{
		note += 0.5;
	}

	if dmarc.ruf != "vide"
	{
		note += 0.5;
	}

	if dmarc.rua != "vide"
	{
		note += 0.5;
	}

	if dmarc.ri != "vide"
	{
		note += 0.5;
	}

	if dmarc.rf != "vide"
	{
		note += 0.5;
	}

	if dmarc.aspf != "vide"
	{
		note += 0.5;
	}

	if dmarc.adkim != "vide"
	{
		note += 0.5;
	}

	if dmarc.fo != "vide"
	{
		note += 0.5;
	}

	dmarc.note = note;

	return dmarc;
}
