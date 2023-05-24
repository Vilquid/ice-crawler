use std::process::Command;
use serde::{Serialize, Serializer};



/// # Brief
/// Structure DANE
/// # Attributes
/// - forme_certificat *i32* : forme du certificat
/// - signature_certificat *bool* : présence de signature du certificat
/// - signature_cle_publique *bool* : présence de signature de la clé publique
/// - presence_hash *bool* : présence d'un hash
/// - hash *String* : hash
/// - note *String* : note de dane
#[derive(Serialize, Debug)]
pub struct DANERecord
{
	pub forme_certificat: String,
	pub signature_certificat: bool,
	pub signature_cle_publique: bool,
	pub presence_hash: bool,
	pub hash: String,
	pub note: f32,
}


pub(crate) fn dane(domain: String) -> DANERecord
{
    // Run the `dig` command to retrieve the DANE record for the domain 
	let output = Command::new("dig")
        .arg("_443._tcp.".to_string() + &domain)
        .arg("TLSA")
        .arg("+short")
        .output()
        .expect("failed to execute process");

    // Convertit la sortie de dig en chaîne de caractères
    let output_str = String::from_utf8(output.stdout).expect("invalid utf8");

	// Création de la structure à renvoyer
	let mut dane_record = DANERecord
	{
		forme_certificat: "vide".to_string(),
		signature_certificat: false,
		signature_cle_publique: false,
		presence_hash: false,
		hash: "vide".to_string().to_owned(),
		note: 0.0,
	};

	// Si la sortie de dig est vide, on retourne un DANERecord vide
	if output_str.is_empty()
	{
		return dane_record;
	}

    // Affichage des variables pour le details 
	let words: Vec<&str> = output_str.split(' ').collect();

	if words.len() >= 5
	{
		dane_record.forme_certificat = words[0].parse().unwrap();

		if words[1] == "0"
		{
			dane_record.signature_certificat = true;
		}

		if words[1] == "1"
		{
			dane_record.signature_cle_publique = true;
		}

		if words[2] == "1"
		{
			dane_record.presence_hash = true;
		}

		// Si words[3] est presque aussi long qu'un hash
		if words[3].len() > 15
		{
			dane_record.hash = words[3].to_string().to_owned();
		}

		dane_record.forme_certificat = dane_record.forme_certificat.trim_matches(';').trim().to_string();
		dane_record.forme_certificat = dane_record.forme_certificat.trim_matches('\n').trim().to_string();
	}

	let mut note = 0.0;

	if dane_record.forme_certificat != "vide"
	{
		note += 2.0;
	}

	if dane_record.signature_certificat
	{
		note += 2.0;
	}

	if dane_record.signature_cle_publique
	{
		note += 2.0;
	}

	if dane_record.presence_hash
	{
		note += 2.0;
	}

	if dane_record.hash != "vide"
	{
		note += 2.0;
	}

	dane_record.note = note;

	dane_record
}
