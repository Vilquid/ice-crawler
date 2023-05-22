use std::process::Command;
use serde::Serialize;
use crate::dmarc::dmarc;


/// # Brief
/// Structure SPF
/// # Attributes
/// - version *String* : version de SPF
/// - mechanismes *Vec<String>* : liste des mécanismes de vérification
/// - ip *Vec<String>* : liste des adresses IPv4 autorisées
/// - include *Vec<String>* : liste des domaines inclus
/// - redirect *String* : domaine de redirection
/// - all *String* : mécanisme de vérification
/// - note *String* : note de SPF
#[derive(Serialize, Debug)]
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


pub(crate) fn spf(domain: String) -> SPFRecord
{
	// Exécute la commande `dig` et récupère la sortie standard
	let output = Command::new("dig")
		.arg(domain.clone())
		.arg("TXT")
		.arg("+short")
		.output()
		.expect("échec de l'exécution de la commande `dig`");

	// Transforme la sortie en chaîne de caractères
	let output_str = String::from_utf8(output.stdout).expect("invalid utf8");
	// Sépare la chaîne de caractères en lignes
	let lines: Vec<&str> = output_str.split("\n").collect();

	// Initialise la structure qui stockera les informations du record SPF
	let mut spf_record = SPFRecord
	{
		version: "vide".to_string(),
		mechanisms: vec![],
		qualifier: "vide".to_string(),
		ip: vec![],
		include: vec![],
		all: "vide".to_string(),
		note: 0.0,
	};

	// Retour d'une structure vide si le serveur ne renvoie rien d'intéressant
	if output_str.is_empty()
	{
		return spf_record;
	}

	// Pour chaque ligne, vérifie si elle contient le record SPF
	for line in lines
	{
		if line.contains("v=spf")
		{
			// Supprime les guillemets et les espaces en début et fin de chaîne
			let mut output = line.trim_matches('\"').trim();
			let output2 = &*output.replace(":", "=");

			let parts: Vec<&str> = output2.clone().split(" ").collect();

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
					"v" => spf_record.version = value.to_string(),
					"ip4" => spf_record.ip.push(value.to_string()),
					"include" => spf_record.include.push(value.to_string()),
					"redirect" => spf_record.include.push(value.to_string()),
					"all" => spf_record.all = value.to_string(),
					"mechanism" => spf_record.mechanisms.push(value.to_string()),
					"qualifier" => spf_record.qualifier = value.to_string(),
					_ => (),
				}
			}
		}
	}

	let mut note:f32 = 0.0;

	if spf_record.version != "vide"
	{
		note += 1.5;
	}

	if !spf_record.mechanisms.is_empty()
	{
		note += 1.5;
	}

	if spf_record.qualifier != "vide"
	{
		note += 1.5;
	}

	if !spf_record.ip.is_empty()
	{
		note += 1.5;
	}

	if !spf_record.include.is_empty()
	{
		note += 1.5;
	}

	if spf_record.all != "vide"
	{
		note += 1.5;
	}

	spf_record.note = note;

	spf_record
}
