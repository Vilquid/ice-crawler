use serde::Serialize;
use std::process::Command;


// The TLS-RPT record is a key-value string and separates the key and values with an equal (=) character. Additionally, it separates the key-value pairs with a semicolon and ignores any whitespaces. The two derivatives are:
// “v”- It is the version indicator, and it is the primary key-value pair within the record.
// 2. “rua”- It defines the address to which the reports need to be delivered. It allows multiple values, separated by a comma.
#[derive(Serialize, Debug)]
pub struct TLSRecord
{
	pub v: String,
	pub rua: String,
	pub note: f32,
}


pub(crate) fn tls_rtp(domain: String) -> TLSRecord
{
	// Run the `dig` command to retrieve the TLS-RPT record for the domain
	let output = Command::new("dig")
		.arg("_report._tls.".to_string() + &domain)
		.arg("TXT")
		.arg("+short")
		.output()
		.expect("failed to execute process");

	// Convertit la sortie de dig en chaîne de caractères
	let output_str = String::from_utf8(output.stdout).expect("invalid utf8");

	let mut tls_record = TLSRecord
	{
		v: "vide".to_string().to_owned(),
		rua: "vide".to_string().to_owned(),
		note: 0.0,
	};

	if output_str.is_empty()
	{
		return tls_record;
	}

	for line in output_str.lines()
	{
		if line.contains("v=TLSRPT")
		{
			let session_string = line.trim_matches('\"').trim();
			// let session_info: Vec<&str> = session_string.split(" ").collect();

			let session_string = session_string.replace(" ", "");

			let parts: Vec<&str> = session_string.split(";").collect();

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
					"v" => tls_record.v = value.to_string(),
					"rua" => tls_record.rua = value.to_string(),
					_ => (),
				}
			}
		}
	}

	let mut note:f32 = 0.0;

	if tls_record.v != "vide"
	{
		note += 5;
	}

	if tls_record.rua != "vide"
	{
		note += 5;
	}

	tls_record.note = note;

	return tls_record;
}
