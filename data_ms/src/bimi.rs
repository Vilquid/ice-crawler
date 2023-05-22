use std::process::Command;
use serde::Serialize;



/// # Brief
/// Structure BIMI
/// # Attributes
/// - version *String* : version de BIMI
/// - url_expediteur *String* : URL de l'image de l'expéditeur
/// - url_politique *String* : URL de la politique de BIMI
/// - url_reputation *String* : URL de la réputation de l'expéditeur
/// - hash *String* : hash de l'image de l'expéditeur
/// - s *String* : signature de l'image de l'expéditeur
/// - note *String* : note de BIMI
#[derive(Serialize, Debug)]
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


pub(crate) fn bimi(domain: String) -> BIMIRecord
{
	// Run the `dig` command to retrieve the BIMI record for the domain
	let output = Command::new("dig")
		.arg("_bimi.".to_string() + &domain)
		.arg("TXT")
		.arg("+short")
		.output()
		.expect("failed to execute process");

	// Convertit la sortie de dig en chaîne de caractères
	let output_str = String::from_utf8(output.stdout).expect("invalid utf8");

	// Déclaration d'une structure BIMIRecord
	let mut bimi_record = BIMIRecord
	{
		version: "vide".to_string(),
		url_expediteur: "vide".to_string(),
		url_politique: "vide".to_string(),
		url_reputation: "vide".to_string(),
		hash: "vide".to_string(),
		s: "vide".to_string(),
		note: 0.0,
	};

	if output_str.is_empty()
	{
		return bimi_record;
	}

	for line in output_str.lines()
	{
		if line.contains("v=BIMI")
		{
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
					"v" => bimi_record.version= value.to_string(),
					"l" => bimi_record.url_expediteur = value.to_string(),
					"p" => bimi_record.url_politique = value.to_string(),
					"r" => bimi_record.url_reputation = value.to_string(),
					"hash" => bimi_record.hash = value.to_string(),
					"s" => bimi_record.s = value.to_string(),
					_ => (),
				}
			}
		}
	}

	let mut note: f32 = 0.0;

	if bimi_record.version == "BIMI"
	{
		note += 4.0;
	}

	if bimi_record.url_expediteur != "vide"
	{
		note += 1.2;
	}

	if bimi_record.url_politique != "vide"
	{
		note += 1.2;
	}

	if bimi_record.url_reputation != "vide"
	{
		note += 1.2;
	}

	if bimi_record.hash != "vide"
	{
		note += 1.2;
	}

	if bimi_record.s != "vide"
	{
		note += 1.2;
	}

	bimi_record
}
