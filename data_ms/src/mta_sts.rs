use std::process::Command;
use serde::Serialize;


/// # Brief
/// Structure de MTA-STS
/// # Attributes
/// - version *String* : version de MTA-STS
/// - sn *String* : numéro de série
/// - note *String* : note de MTA-STS
/// # Warning
/// version doit être un *i32* et pas un *String*.
#[derive(Serialize, Debug)]
pub struct MTARecord
{
	#[serde(with = "serde_json::json")]
	pub version: String,
	pub sn: String,
	#[serde(serialize_with = "serialize_f32_without_quotes")]
	pub note: f32,
}


fn serialize_f32_without_quotes<S>(value: &f32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}


/// # Brief
/// Fonction qui récupère les informations de MTA-STS de l'adresse IP ou du nom de domaine passé.e en paramètre.
/// # Arguments
/// - ip_dn *&str* : adresse IP ou nom de domaine
/// # Return
/// Retourne une structure *MTARecord*.
pub(crate) fn mta(domain: String) -> MTARecord
{
	// Run the `dig` command to retrieve the MTA-STS record for the domain
	let output = Command::new("dig")
		.arg("_mta-sts.".to_string() + &domain)
		.arg("TXT")
		.arg("+short")
		.output()
		.expect("failed to execute process");

	// Convertit la sortie de dig en chaîne de caractères
    let output_str = String::from_utf8(output.stdout).expect("invalid utf8");

	let mut mta_record = MTARecord
	{
		version: "vide".to_string(),
		sn: "vide".to_string(),
		note: 0.0,
	};

	if output_str.is_empty()
	{
		return mta_record;
	}

	let session_string = output_str;
	if session_string.contains("v=STS")
	{
		let session_string = session_string.trim_matches('\"').trim();
		let session_info: Vec<&str> = session_string.split(" ").collect();
		mta_record.version = String::from(session_info[0].split("=").collect::<Vec<&str>>()[1]);

		if session_info.len() == 1
		{
			let output = session_string.trim_matches('\"').trim();

			let parts: Vec<&str> = output.clone().split(";").collect();

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
					"v" => mta_record.version = value.to_string(),
					"id" => mta_record.sn = value.to_string(),
					_ => (),
				}
			}
		}

		if session_info.len() >= 2
		{
			mta_record.sn = String::from(session_info[1].split("=").collect::<Vec<&str>>()[1].trim_matches('\"').trim_matches(';'));
		}
	}

	if mta_record.version.contains("id") || mta_record.version.contains("ve")
	{
		mta_record.version = mta_record.version.replace("id", "");
		mta_record.version = mta_record.version.replace("ve", "vide");
	}

	let mut note:f32 = 0.0;

	if mta_record.version != "vide"
	{
		note += 5.0;
	}

	if mta_record.sn != "vide"
	{
		note += 5.0;
	}

	mta_record.note = note;

	return mta_record;
}
