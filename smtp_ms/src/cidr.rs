use std::net::{IpAddr, Ipv4Addr};
use regex::Regex;
use serde::Serialize;


/// # Brief
/// Structure qui contient la première et la dernière adresse IP d'un réseau
/// # Fields
/// - debut *String* : première adresse IP du réseau
/// - fin *String* : dernière adresse IP du réseau
#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub struct IpRange
{
	pub debut: String,
	pub fin: String,
}


/// # Brief
/// Fonction qui retourne la première et la dernière adresse IP d'un réseau en fonction de la notation CIDR passée en paramètre
/// # Attributes
/// - cidr *&str* : réseau à analyser avec une notation CIDR
/// # Return
/// Renvoie une structure *IpRange*
/// # Example
/// ```
/// use cidr::cidr_notation;
/// println!("{:#?}", cidr_notation("0.0.0.0/0"));
/// ```
/// # Warning
/// Fonctionnement particulier quand la CIDR se termine par /0
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

/// # Brief
/// Fonction qui parse une notation CIDR et retourne l'adresse IP et le nombre de bits
/// # Attributes
/// - cidr *&str* : notation CIDR à parser
/// # Return
/// Renvoie un tuple contenant l'adresse IP et le nombre de bits
/// # Example
/// ```
/// let (ip, bits) = parse_cidr(cidr).unwrap();
///
/// return IpRange
/// {
/// 	debut: network_start(ip, bits).to_string(),
/// 	fin: network_end(ip, bits).to_string(),
/// };
/// ```
/// # Warning
/// Si la notation CIDR est invalide, on retourne le réseau 192.168.0.0/24 par défaut.
fn parse_cidr(cidr: &str) -> Result<(IpAddr, u8), &str>
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
